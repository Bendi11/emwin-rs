use std::{path::Path, process::ExitCode, sync::Arc};

use actix_files::Files;
use actix_web::{error, get, web::{self, Data}, App, HttpServer, Responder, HttpResponse, middleware::Logger, Result};
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

#[get("latest.html")]
async fn latest(sql: Data<MySqlPool>) -> Result<impl Responder> {
    let fd = sqlx::query(
r#"
SELECT (path)
FROM goesimg.files
WHERE start_dt=(SELECT max(start_dt) FROM goesimg.files) AND sector='FULL_DISK';
"#
    )
        .fetch_one(sql.get_ref())
        .await
        .map_err(|e| error::ErrorBadRequest(e))?
        .try_get::<&str, _>(0)
        .map_err(|e| error::ErrorBadRequest(e))?
        .to_owned();

    let mesoscale1 = sqlx::query(
r#"
SELECT (path)
FROM GOESIMG.files
WHERE start_dt=(SELECT max(start_dt) FROM goesimg.files) AND sector='MESOSCALE1';
"#
    )
    .fetch_one(sql.get_ref())
    .await
    .map_err(|e| error::ErrorBadRequest(e))?
    .try_get::<&str, _>(0)
    .map_err(|e| error::ErrorBadRequest(e))?
    .to_owned();

    Ok(Latest {
        fd,
        mesoscale1,
    })
}

#[actix_web::main]
async fn main() -> ExitCode {
    std::env::set_var("RUST_LOG", "warn");
    let config = match Config::read().await {
        Ok(cfg) => cfg,
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
            .service(index)
            .wrap(logger)
            .service(Files::new("/static", static_dir))
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
