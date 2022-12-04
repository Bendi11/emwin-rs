use std::{process::ExitCode, sync::Arc, time::Duration};

use dispatch::img_dispatch;
use fuser::MountOption;
use goes_sql::GoesSqlContext;
use notify::{event::CreateKind, Event, EventKind, RecommendedWatcher, Watcher};

use sqlx::MySqlPool;
use tokio::{
    runtime::Runtime,
    sync::mpsc::{channel, Receiver, Sender},
};

use goes_cfg::Config;

use crate::fuse::EmwinFS;

pub mod dispatch;
pub mod fuse;

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
        let config = match Config::read().await {
            Ok(cfg) => Arc::new(cfg),
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

        let ctx = Arc::new(GoesSqlContext::new(pool));
        if let Err(e) = ctx.init().await {
            log::error!("Failed to initialize database: {}", e);
        }

        let (img_tx, img_rx) = channel(10);
        let img_watcher = match create_watcher(rt.clone(), img_tx) {
            Ok(img_w) => img_w,
            Err(e) => return e,
        };

        let cfg = config.clone();
        let context = ctx.clone();
        let runtime = rt.clone();
        let emwin_task = tokio::task::spawn_blocking(move || {
            let fs = EmwinFS::new(runtime, context);
            if let Err(e) = fuser::mount2(
                fs,
                &cfg.emwin_dir,
                &[
                    MountOption::AutoUnmount,
                    MountOption::AllowOther,
                    MountOption::NoExec,
                    MountOption::NoAtime,
                    MountOption::FSName("EMWIN in-memory FS".to_owned()),
                    MountOption::NoSuid,
                    MountOption::RW,
                ],
            ) {
                log::error!("Failed to mount filesystem at {}: {}", cfg.emwin_dir.display(), e);
            }

            log::error!("Filesystem unmounted at {}", cfg.emwin_dir.display());
        });

        let img_task = tokio::task::spawn(img_task(config, ctx, img_rx, img_watcher));
        tokio::select! {
            _ = emwin_task => return ExitCode::FAILURE,
            Ok(v) = img_task => return v,
        }
    })
}

fn create_watcher(rt: Arc<Runtime>, tx: Sender<Event>) -> Result<RecommendedWatcher, ExitCode> {
    Ok(
        match RecommendedWatcher::new(
            move |res| {
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
        },
    )
}

async fn img_task(
    config: Arc<Config>,
    ctx: Arc<GoesSqlContext>,
    mut rx: Receiver<Event>,
    mut watcher: RecommendedWatcher,
) -> ExitCode {
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
