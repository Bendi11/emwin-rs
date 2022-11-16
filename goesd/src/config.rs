use std::{path::{Path, PathBuf}, sync::Arc, process::ExitCode};

use serde::{Deserialize, Serialize};
use tokio::io::{AsyncWriteExt, AsyncReadExt};

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
    /// Folder that contains all GOES EMWIN output files
    #[serde(rename = "emwin-dir")]
    pub emwin_dir: PathBuf,
    /// Folder that contains all GOES image files
    #[serde(rename = "img-dir")]
    pub img_dir: PathBuf,
    #[serde(rename = "db-url")]
    pub db_url: String,
    /// What to do when we get an unrecognized file in the input directory
    pub unrecognized: UnrecognizedFileOpt,
    pub failure: UnrecognizedFileOpt,
}

pub const CONFIG_FOLDER: &str = "emwind/";
pub const CONFIG_FILE: &str = "config.toml";

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

pub async fn read_cfg() -> Result<Arc<Config>, ExitCode> {
    Ok(Arc::new(match dirs::config_dir() {
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
                            return Err(ExitCode::FAILURE);
                        }

                        match toml::from_slice(&buf) {
                            Ok(config) => config,
                            Err(e) => {
                                log::error!(
                                    "Failed to deserialize configuration from file {}: {}",
                                    config_path.display(),
                                    e,
                                );
                                return Err(ExitCode::FAILURE);
                            }
                        }
                    }
                    Err(e) => {
                        log::error!(
                            "Failed to open configuration file at {}: {}",
                            config_path.display(),
                            e,
                        );
                        return Err(ExitCode::FAILURE);
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
    }))
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
            emwin_dir: dirs::home_dir().unwrap_or("~".into()).join("goes/text"),
            img_dir: dirs::home_dir().unwrap_or("~".into()).join("goes/img"),
            unrecognized: UnrecognizedFileOpt::Delete,
            failure: UnrecognizedFileOpt::Move(
                dirs::home_dir().unwrap_or("~".into()).join("emwind/fail/"),
            ),
            db_url: "mysql://root:@localhost/weather".to_owned(),
        }
    }
}
