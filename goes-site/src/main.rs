use std::{path::{Path, PathBuf}, process::ExitCode, sync::Arc};

use actix_files::Files;
use actix_web::{get, web::Data, App, HttpServer, Responder, middleware::{Logger, self}};
use goes_cfg::Config;
use page::latest::{latest, latest_fd_fc_ep};

pub mod page;

#[derive(askama::Template)]
#[template(path="index.html")]
pub struct Index;

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
            .service(latest)
            .service(latest_fd_fc_ep)
            .wrap(logger)
            .wrap(middleware::Compress::default())
            .service(Files::new("/assets", &config.img_dir).show_files_listing())
            .service(Files::new("/", static_dir))
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
