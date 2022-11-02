use std::{
    path::{Path, PathBuf},
    process::ExitCode,
    time::Duration,
};

use notify::{event::CreateKind, Event, EventKind, RecommendedWatcher, Watcher};
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::mpsc::{channel, Receiver},
};

#[derive(Deserialize, Serialize)]
pub struct Config {
    /// Folder that contains all GOES output files
    #[serde(rename = "goes-dir")]
    pub goes_dir: PathBuf,
}

pub const CONFIG_FOLDER: &str = "emwind/";
pub const CONFIG_FILE: &str = "config.toml";

#[tokio::main]
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
    let config = match dirs::config_dir() {
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
    };

    if let Err(e) = watcher.watch(&config.goes_dir, notify::RecursiveMode::Recursive) {
        log::error!(
            "Failed to subscribe to filesystem events for {}: {}",
            config.goes_dir.display(),
            e,
        );
        return ExitCode::FAILURE;
    }

    while let Some(event) = rx.recv().await {
        match event.kind {
            EventKind::Create(CreateKind::File) => {}
            _ => (),
        }
    }

    ExitCode::SUCCESS
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
            goes_dir: "~/goes/".into(),
        }
    }
}
