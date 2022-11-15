use std::{process::ExitCode, sync::Arc, time::Duration};

use dispatch::on_create;
use goes_sql::EmwinSqlContext;
use notify::{event::CreateKind, Event, EventKind, RecommendedWatcher, Watcher};

use sqlx::MySqlPool;
use tokio::sync::mpsc::{channel, Receiver};

use crate::config::Config;

pub mod config;
pub mod dispatch;

fn main() -> ExitCode {
    if let Err(e) = stderrlog::new()
        .verbosity(log::LevelFilter::max())
        .show_module_names(false)
        .init()
    {
        eprintln!("Failed to initialize logger: {}", e);
    }

    log::trace!("emwind started!");

    let rt = Arc::new(
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to initialize tokio runtime"),
    );

    let rt_clone = rt.clone();
    let (tx, rx) = channel(10);
    let watcher = match RecommendedWatcher::new(
        move |res| {
            rt_clone.block_on(async {
                match res {
                    Ok(event) => {
                        if let Err(e) = tx.send(event).await {
                            log::error!("Failed to send event through channel: {}", e);
                        }
                    }
                    Err(e) => log::error!("Failed to receive filesystem event: {}", e),
                }
            });
        },
        notify::Config::default().with_poll_interval(Duration::from_secs(600)),
    ) {
        Ok(watcher) => watcher,
        Err(e) => {
            log::error!("Failed to create filesystem watcher: {}", e);
            return ExitCode::FAILURE;
        }
    };

    rt.block_on(async {
        let config = match config::read_cfg().await {
            Ok(cfg) => cfg,
            Err(e) => return e,
        };

        watch(config, watcher, rx).await
    })
}

async fn watch(config: Arc<Config>, mut watcher: RecommendedWatcher, mut rx: Receiver<Event>) -> ExitCode {
    if let Err(e) = watcher.watch(&config.goes_dir, notify::RecursiveMode::Recursive) {
        log::error!(
            "Failed to subscribe to filesystem events for {}: {}",
            config.goes_dir.display(),
            e,
        );
        return ExitCode::FAILURE;
    }

    log::trace!(
        "watching {} for filesystem events",
        config.goes_dir.display()
    );

    let pool = match MySqlPool::connect(&config.db_url).await {
        Ok(p) => p,
        Err(e) => {
            log::error!("Failed to connect to database at {}: {}", config.db_url, e,);
            return ExitCode::FAILURE;
        }
    };

    log::trace!("connected to database on {}", config.db_url);

    let ctx = Arc::new(EmwinSqlContext::new(pool));
    if let Err(e) = ctx.init().await {
        log::error!("Failed to initialize database: {}", e);
    }

    while let Some(event) = rx.recv().await {
        match event.kind {
            EventKind::Create(CreateKind::File) => {
                let config = Arc::clone(&config);
                let ctx = Arc::clone(&ctx);
                if let Err(e) =
                    tokio::spawn(async move { on_create(event, ctx, config).await }).await
                {
                    log::error!("Failed to spawn file reader task: {}", e);
                }
            }
            _ => (),
        }
    }

    ExitCode::SUCCESS
}
