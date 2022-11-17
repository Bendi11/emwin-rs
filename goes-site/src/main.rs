use std::{path::Path, process::ExitCode, sync::Arc};

use actix_files::Files;
use actix_web::{get, web::{self, Data}, App, HttpServer, Responder, HttpResponse};
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
async fn latest(sql: Data<MySqlPool>) -> impl Responder {
    let fd = sqlx::query(
r#"
SELECT (path)
FROM goesimg.files
WHERE start_dt=(SELECT max(start_dt) FROM goesimg.files) AND sector='FULL_DISK';
"#
    )
        .fetch_one(sql.get_ref())
        .await?
        .try_get::<&str, _>(0)?
        .to_owned();

    let mesoscale1 = sqlx::query(
r#"
SELECT (path)
FROM GOESIMG.files
WHERE start_dt=(SELECT max(start_dt) FROM goesimg.files) AND sector='MESOSCALE1';
"#
    )
    .fetch_one(sql.get_ref())
    .await?
    .try_get::<&str, _>(0)?
    .to_owned();

    Ok::<_, Box<dyn std::error::Error>>(Latest {
        fd,
        mesoscale1,
    })
}

#[actix_web::main]
async fn main() -> ExitCode {
    if let Err(e) = stderrlog::new()
        .verbosity(log::LevelFilter::max())
        .show_module_names(false)
        .init()
    {
        eprintln!("Failed to initialize logger: {}", e);
    }
    
    let config = match Config::read().await {
        Ok(cfg) => cfg,
        Err(e) => return e,
    };

    let static_dir = Path::new("goes-site/static");

    let db_conn = match sqlx::MySqlPool::connect(&config.db_url).await {
        Ok(pool) => Data::new(pool),
        Err(e) => {
            log::error!("Failed to connect to SQL database: {}", e);
            return ExitCode::FAILURE
        }
    };

    let bound = match HttpServer::new(move ||
        App::new()
            .app_data(db_conn.clone())
            .service(index)
            .service(Files::new("/static", static_dir))
        )
        .bind("localhost:8000") {
            Ok(bound) => bound,
            Err(e) => {
                log::error!("Failed to bind HTTP server to localhost:8000: {}", e);
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
