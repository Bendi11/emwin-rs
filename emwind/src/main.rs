use std::{
    path::{Path, PathBuf},
    process::ExitCode,
    time::Duration,
};

use emwin_parse::{
    dt::{
        code::CodeForm, product::{Analysis, Forecast}, upperair::UpperAirData, AnalysisSubType,
        DataTypeDesignator, UpperAirDataSubType, ForecastSubType,
    },
    formats::{amdar::AmdarReport, rwr::RegionalWeatherRoundup, taf::TAFReport},
    header::GoesFileName,
};
use notify::{event::CreateKind, Event, EventKind, RecommendedWatcher, Watcher};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::mpsc::{channel, Receiver},
};

/// Action to take when an unrecognized file appears in the input directory
#[derive(Serialize, Deserialize)]
#[serde(tag = "on", content = "path")]
pub enum UnrecognizedFileOpt {
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "leave")]
    None,
    #[serde(rename = "move")]
    Move(PathBuf),
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    /// Folder that contains all GOES output files
    #[serde(rename = "goes-dir")]
    pub goes_dir: PathBuf,
    /// What to do when we get an unrecognized file in the input directory
    pub unrecognized: UnrecognizedFileOpt,
    pub failure: UnrecognizedFileOpt,
}

pub const CONFIG: OnceCell<Config> = OnceCell::new();

pub const CONFIG_FOLDER: &str = "emwind/";
pub const CONFIG_FILE: &str = "config.toml";

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
                    write_config(&config_path, &config).await;
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
                    },
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

impl UnrecognizedFileOpt {
    /// Attempt to execute the given action for a file at `path`
    pub async fn do_for(&self, path: impl AsRef<Path>) {
        match self {
            Self::Delete => {
                if let Err(e) = tokio::fs::remove_file(&path).await {
                    log::error!("Failed to delete file {}: {}", path.as_ref().display(), e);
                }
            }
            Self::Move(to) => {
                if let Err(e) = tokio::fs::copy(
                    &path,
                    to.join(
                        path.as_ref()
                            .file_name()
                            .unwrap_or(path.as_ref().as_os_str()),
                    ),
                )
                .await
                {
                    log::error!(
                        "Failed to move file {} to {}: {}",
                        path.as_ref().display(),
                        to.display(),
                        e
                    );
                }
            }
            Self::None => (),
        }
    }
}

async fn write_config<P: AsRef<Path>>(path: P, config: &Config) {
    match tokio::fs::File::create(&path).await {
        Ok(mut file) => {
            let buf = match toml::to_vec(&config) {
                Ok(buf) => buf,
                Err(e) => {
                    log::error!("Failed to serialize default configuration: {}", e);
                    return;
                }
            };

            if let Err(e) = file.write_all(&buf).await {
                log::error!(
                    "Failed to write default configuration file {}: {}",
                    path.as_ref().display(),
                    e
                );
            }
        }
        Err(e) => {
            log::warn!(
                "Failed to create configuration file {}: {}, using default configuration",
                path.as_ref().display(),
                e,
            );
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            goes_dir: dirs::home_dir().unwrap_or("~".into()).join("goes/"),
            unrecognized: UnrecognizedFileOpt::Delete,
            failure: UnrecognizedFileOpt::Move(
                dirs::home_dir().unwrap_or("~".into()).join("emwind/fail/"),
            ),
        }
    }
}
