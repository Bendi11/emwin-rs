use std::{
    path::{Path, PathBuf},
    process::ExitCode,
    sync::Arc,
    time::Duration,
};

use config::{CONFIG_FILE, CONFIG_FOLDER};
use dispatch::on_create;
use emwin_parse::{
    dt::{
        code::CodeForm,
        product::{Analysis, Forecast},
        upperair::UpperAirData,
        AnalysisSubType, DataTypeDesignator, ForecastSubType, UpperAirDataSubType,
    },
    formats::{amdar::AmdarReport, rwr::RegionalWeatherRoundup, taf::TAFReport},
    header::GoesFileName,
};
use emwin_sql::EmwinSqlContext;
use notify::{event::CreateKind, Event, EventKind, RecommendedWatcher, Watcher};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::mpsc::{channel, Receiver},
};

use crate::config::{Config, CONFIG};

pub mod config;
pub mod dispatch;

fn main() -> ExitCode {
    if let Err(e) = stderrlog::new().verbosity(log::LevelFilter::max()).show_module_names(false).init() {
        eprintln!("Failed to initialize logger: {}", e);
    }

    log::trace!("emwind started!");

    let rt = Arc::new(tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to initialize tokio runtime")
    );
    
    let rt_clone = rt.clone();
    let (tx, rx) = channel(10);
    let watcher = match RecommendedWatcher::new(
        move |res| {
            rt_clone.block_on(async {
                log::trace!("got filesystem event");
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

    rt.block_on(watch(watcher, rx))
}

async fn watch(mut watcher: RecommendedWatcher, mut rx: Receiver<Event>) -> ExitCode {
    let _ = CONFIG.set(match dirs::config_dir() {
        Some(dir) => {
            let config_path = dir.join(CONFIG_FOLDER).join(CONFIG_FILE);
            if config_path.exists() {
                match tokio::fs::File::open(&config_path).await {
                    Ok(mut file) => {
                        let mut buf = Vec::with_capacity(128);
                        if let Err(e) = file.read_to_end(&mut buf).await {
                            log::error!(
                                "Failed to read configuration file {}: {}",
                                config_path.display(),
                                e
                            );
                            return ExitCode::FAILURE;
                        }

                        match toml::from_slice(&buf) {
                            Ok(config) => config,
                            Err(e) => {
                                log::error!(
                                    "Failed to deserialize configuration from file {}: {}",
                                    config_path.display(),
                                    e,
                                );
                                return ExitCode::FAILURE;
                            }
                        }
                    }
                    Err(e) => {
                        log::error!(
                            "Failed to open configuration file at {}: {}",
                            config_path.display(),
                            e,
                        );
                        return ExitCode::FAILURE;
                    }
                }
            } else {
                let config = Config::default();
                if let Err(e) = std::fs::create_dir_all(config_path.parent().unwrap()) {
                    log::warn!(
                        "Failed to create configuration directory {}: {}, using default configuration",
                        config_path.display(),
                        e,
                    );
                } else {
                    config::write_config(&config_path, &config).await;
                }
                config
            }
        }
        None => {
            log::warn!(
                "Failed to find system configuration directory, using default configuration"
            );
            Config::default()
        }
    });

    if let Err(e) = watcher.watch(&CONFIG.wait().goes_dir, notify::RecursiveMode::Recursive) {
        log::error!(
            "Failed to subscribe to filesystem events for {}: {}",
            CONFIG.wait().goes_dir.display(),
            e,
        );
        return ExitCode::FAILURE;
    }

    log::trace!("watching {} for filesystem events", CONFIG.wait().goes_dir.display());

    let pool = match MySqlPool::connect(&CONFIG.wait().db_url).await {
        Ok(p) => p,
        Err(e) => {
            log::error!(
                "Failed to connect to database at {}: {}",
                CONFIG.wait().db_url,
                e,
            );
            return ExitCode::FAILURE;
        }
    };

    log::trace!("connected to database on {}", CONFIG.wait().db_url);

    let ctx = Arc::new(EmwinSqlContext::new(pool));

    while let Some(event) = rx.recv().await {
        log::trace!("Got filesystem event");
        match event.kind {
            EventKind::Create(CreateKind::File) => {

                let ctx = Arc::clone(&ctx);
                if let Err(e) = tokio::spawn(async move { on_create(event, ctx).await }).await {
                    log::error!("Failed to spawn file reader task: {}", e);
                }
            }
            _ => (),
        }
    }

    ExitCode::SUCCESS
}
