use std::{process::ExitCode, sync::Arc, time::Duration};

use dispatch::{emwin_dispatch, img_dispatch};
use goes_sql::EmwinSqlContext;
use notify::{event::CreateKind, Event, EventKind, RecommendedWatcher, Watcher};

use sqlx::MySqlPool;
use tokio::{sync::mpsc::{channel, Receiver, Sender}, runtime::Runtime};

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
     
    rt.clone().block_on(async move {
        let config = match config::read_cfg().await {
            Ok(cfg) => cfg,
            Err(e) => return e,
        };

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
        
        
        let (emwin_tx, emwin_rx) = channel(10);
        let (img_tx, img_rx) = channel(10);
        let (emwin_watcher, img_watcher) = match (create_watcher(rt.clone(), emwin_tx), create_watcher(rt.clone(), img_tx)) {
            (Ok(emwin_w), Ok(img_w)) => (emwin_w, img_w),
            (Err(e), _) | (_, Err(e)) => return e,
        };

        let emwin_task = tokio::task::spawn(emwin_task(config.clone(), ctx.clone(), emwin_rx, emwin_watcher));
        let img_task = tokio::task::spawn(img_task(config, ctx, img_rx, img_watcher)); 

        tokio::select! {
            Ok(v) = emwin_task => return v,
            Ok(v) = img_task => return v,
        } 
    })
}

fn create_watcher(rt: Arc<Runtime>, tx: Sender<Event>) -> Result<RecommendedWatcher, ExitCode> {
    Ok(match RecommendedWatcher::new(move |res| {
            rt.block_on(async {
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
            return Err(ExitCode::FAILURE);
        }
    })
}

async fn img_task(config: Arc<Config>, ctx: Arc<EmwinSqlContext>, mut rx: Receiver<Event>, mut watcher: RecommendedWatcher) -> ExitCode {
    if let Err(e) = watcher.watch(&config.img_dir, notify::RecursiveMode::Recursive) {
        log::error!(
            "Failed to subscribe to filesystem events for {}: {}",
            config.img_dir.display(),
            e,
        );
        return ExitCode::FAILURE;
    }

    while let Some(event) = rx.recv().await {
        match event.kind {
            EventKind::Create(CreateKind::File) => {
                let config = Arc::clone(&config);
                let ctx = Arc::clone(&ctx);
                if let Err(e) =
                    tokio::spawn(async move { img_dispatch(event, ctx, config).await }).await
                {
                    log::error!("Failed to spawn file reader task: {}", e);
                }
            }
            _ => (),
        }
    }

    ExitCode::SUCCESS
}

async fn emwin_task(config: Arc<Config>, ctx: Arc<EmwinSqlContext>, mut rx: Receiver<Event>, mut watcher: RecommendedWatcher) -> ExitCode {
    if let Err(e) = watcher.watch(&config.emwin_dir, notify::RecursiveMode::Recursive) {
        log::error!(
            "Failed to subscribe to filesystem events for {}: {}",
            config.emwin_dir.display(),
            e,
        );
        return ExitCode::FAILURE;
    }

    while let Some(event) = rx.recv().await {
        match event.kind {
            EventKind::Create(CreateKind::File) => {
                let config = Arc::clone(&config);
                let ctx = Arc::clone(&ctx);
                if let Err(e) =
                    tokio::spawn(async move { emwin_dispatch(event, ctx, config).await }).await
                {
                    log::error!("Failed to spawn file reader task: {}", e);
                }
            }
            _ => (),
        }
    }

    ExitCode::SUCCESS
}
