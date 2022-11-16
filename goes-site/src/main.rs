use std::path::Path;

use actix_files::Files;
use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};

#[derive(askama::Template)]
#[template(path="index.html")]
pub struct Index;

#[get("index.html")]
async fn index() -> impl Responder {
    Index
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let static_dir = Path::new("goes-site/static");

    HttpServer::new(move ||
        App::new()
            .service(index)
            .service(Files::new("/static", static_dir))
        )
        .bind("localhost:8000")?
        .run()
        .await
}
