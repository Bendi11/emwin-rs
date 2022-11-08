use std::path::{Path, PathBuf};

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;

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
    #[serde(rename = "db-url")]
    pub db_url: String,
    /// What to do when we get an unrecognized file in the input directory
    pub unrecognized: UnrecognizedFileOpt,
    pub failure: UnrecognizedFileOpt,
}

pub const CONFIG: OnceCell<Config> = OnceCell::new();
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

pub async fn write_config<P: AsRef<Path>>(path: P, config: &Config) {
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
            db_url: "mysql://root:@localhost/weather".to_owned(),
        }
    }
}
