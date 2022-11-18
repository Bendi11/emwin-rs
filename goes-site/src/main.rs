use std::{path::{Path, PathBuf}, process::ExitCode, sync::Arc};

use actix_files::Files;
use actix_web::{error, get, web::{self, Data}, App, HttpServer, Responder, HttpResponse, middleware::{Logger, self}, Result};
use goes_cfg::Config;
use sqlx::{MySqlPool, Row};

#[derive(askama::Template)]
#[template(path="index.html")]
pub struct Index;

#[derive(askama::Template)]
#[template(path="latest.html")]
pub struct Latest {
    fd: String,
    mesoscale1: String,
}

#[get("index.html")]
async fn index() -> impl Responder {
    Index
}

fn map_path(cfg: &Config) -> impl FnOnce(&str) -> String + '_ {
    move |path: &str| PathBuf::from(path)
        .strip_prefix(&cfg.img_dir)
        .map(Path::to_owned)
        .unwrap_or_else(|e| {
            log::error!(
                "Failed to remove image prefix {} from path {}: {}",
                cfg.img_dir.display(),
                path,
                e,
            );
            PathBuf::new()
        })
        .to_string_lossy()
        .into_owned()
}

#[get("latest.html")]
async fn latest(sql: Data<MySqlPool>, cfg: Data<Config>) -> Result<Latest> {
    let fd = sqlx::query(
r#"
SELECT (file_name)
FROM goesimg.files
WHERE start_dt=(SELECT max(start_dt) FROM goesimg.files WHERE sector='FULL_DISK' AND channel='FULL_COLOR') AND sector='FULL_DISK' AND channel='FULL_COLOR';
"#
    )
        .fetch_one(sql.get_ref())
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))
        .and_then(|v| v
            .try_get::<&str, _>(0)
            .map(map_path(cfg.get_ref()))
            .map_err(|e| error::ErrorBadRequest(e.to_string()))
        );

    let mesoscale1 = sqlx::query(
r#"
SELECT (file_name)
FROM goesimg.files
WHERE start_dt=(SELECT max(start_dt) FROM goesimg.files WHERE sector='MESOSCALE1' AND channel='FULL_COLOR_LINES') AND sector='MESOSCALE1' AND channel='FULL_COLOR_LINES';
"#
    )
    .fetch_one(sql.get_ref())
    .await
    .map_err(|e| error::ErrorBadRequest(e.to_string()))
    .and_then(|v| v
        .try_get::<&str, _>(0)
        .map(map_path(cfg.get_ref()))
        .map_err(|e| error::ErrorBadRequest(e.to_string()))
    );


    Ok(Latest {
        fd: fd.unwrap_or_else(|e| {
            log::error!("Failed to fetch latest full disk from database: {}", e);
            "".to_owned()
        }),
        mesoscale1: mesoscale1.unwrap_or_else(|e| {
            log::error!("Failed to fetch the latest mesoscale 1 image from database: {}", e);
            "".to_owned()
        }),
    })
}

#[actix_web::main]
async fn main() -> ExitCode {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let config = match Config::read().await {
        Ok(cfg) => Data::new(cfg),
        Err(e) => return e,
    };
    

    let static_dir = Path::new("/usr/share/goes-site/static");
    
    let db_conn = match sqlx::MySqlPool::connect(&config.db_url).await {
        Ok(pool) => Data::new(pool),
        Err(e) => {
            log::error!("Failed to connect to SQL database: {}", e);
            return ExitCode::FAILURE
        }
    };

    let bound = match HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .app_data(db_conn.clone())
            .app_data(config.clone())
            .service(index)
            .wrap(logger)
            .wrap(middleware::Compress::default())
            .service(Files::new("/", static_dir))
            .service(Files::new("/", &config.img_dir))
        })
        .bind("0.0.0.0:8000") {
            Ok(bound) => bound,
            Err(e) => {
                log::error!("Failed to bind HTTP server to 0.0.0.0:8000: {}", e);
                return ExitCode::FAILURE
            }
        };

    if let Err(e) = bound.run().await {
        log::error!("Failed to start HTTP server: {}", e);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
