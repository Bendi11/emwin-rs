use std::{
    path::{Path, PathBuf},
    process::ExitCode,
    time::Duration,
};

use config::{CONFIG_FILE, CONFIG_FOLDER};
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
pub mod db;

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    if let Err(e) = stderrlog::new().show_module_names(false).init() {
        eprintln!("Failed to initialize logger: {}", e);
    }

    let (tx, rx) = channel(10);
    let watcher = match RecommendedWatcher::new(
        move |res| {
            tokio::runtime::Handle::current().block_on(async {
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

    watch(watcher, rx).await
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

    while let Some(event) = rx.recv().await {
        match event.kind {
            EventKind::Create(CreateKind::File) => {
                if let Err(e) = tokio::spawn(async move { on_create(event).await }).await {
                    log::error!("Failed to spawn file reader task: {}", e);
                }
            }
            _ => (),
        }
    }

    ExitCode::SUCCESS
}

pub async fn on_create(event: Event) {
    for path in event.paths {
        match path.file_stem().map(std::ffi::OsStr::to_str).flatten() {
            Some(filename) => {
                let filename: GoesFileName = match filename.parse() {
                    Ok(f) => f,
                    Err(e) => {
                        log::error!("Failed to parse newly created filename {}: {}", filename, e);
                        CONFIG.wait().failure.do_for(&path).await;
                        return;
                    }
                };

                let read = async {
                    match tokio::fs::read_to_string(&path).await {
                        Ok(src) => Some(src),
                        Err(e) => {
                            log::error!("Failed to read file {}: {}", path.display(), e);
                            CONFIG.wait().failure.do_for(&path).await;
                            None
                        }
                    }
                };

                match filename.wmo_product_id {
                    DataTypeDesignator::Analysis(Analysis {
                        subtype: AnalysisSubType::Surface,
                        ..
                    }) => {
                        let Some(src) = read.await else { return };
                        let _ = match RegionalWeatherRoundup::parse(&src) {
                            Ok((_, rwr)) => rwr,
                            Err(e) => {
                                log::error!("Failed to parse regional weather roundup: {}", e);
                                CONFIG.wait().failure.do_for(&path).await;
                                return;
                            }
                        };
                    }
                    DataTypeDesignator::UpperAirData(UpperAirData {
                        subtype: UpperAirDataSubType::AircraftReport(CodeForm::AMDAR),
                        ..
                    }) => {
                        let Some(src) = read.await else { return };
                        let report = match AmdarReport::parse(&src) {
                            Ok((_, report)) => report,
                            Err(e) => {
                                log::error!("Failed to parse AMDAR upper air report: {}", e);
                                CONFIG.wait().failure.do_for(&path).await;
                                return;
                            }
                        };
                    }
                    DataTypeDesignator::Forecast(Forecast {
                        subtype: ForecastSubType::AerodomeVTLT12 | ForecastSubType::AerodomeVTGE12,
                        ..
                    }) => {
                        let Some(src) = read.await else { return };
                        let forecast = match TAFReport::parse(&src) {
                            Ok((_, forecast)) => forecast,
                            Err(e) => {
                                log::error!("Failed to parse TAF report: {}", e);
                                CONFIG.wait().failure.do_for(&path).await;
                                return;
                            }
                        };
                    }
                    _ => {
                        log::info!("Unknown EMWIN product: {:?}", filename.wmo_product_id);
                        CONFIG.wait().unrecognized.do_for(&path).await;
                    }
                }
            }
            None => {
                log::error!(
                    "Newly created file {} contains invalid unicode characters",
                    path.display()
                );
                CONFIG.wait().unrecognized.do_for(&path).await;
            }
        }
    }
}
