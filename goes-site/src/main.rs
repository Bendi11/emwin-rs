use std::{path::Path, process::ExitCode};

use actix_files::Files;
use actix_web::{web::Data, App, HttpServer, middleware::{Logger, self}};
use goes_cfg::Config;
use page::{latest::latest_scope, search::search_scope};

pub mod page;


fn map_path(cfg: &Config) -> impl FnOnce(&str) -> actix_web::Result<&std::path::Path> + '_ {
    move |path: &str| Path::new(path)
        .strip_prefix(&cfg.img_dir)
        .map_err(|e| {
            log::error!(
                "Failed to remove image prefix {} from path {}: {}",
                cfg.img_dir.display(),
                path,
                e,
            );

            actix_web::error::ErrorInternalServerError(e)
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
            .wrap(logger)
            .wrap(middleware::Compress::default())
            .service(latest_scope())
            .service(search_scope())
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
