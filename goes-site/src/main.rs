use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};

#[derive(askama::Template)]
#[template(source = "<h1>Hello, World</h1>", ext="html")]
pub struct Index;

#[get("index.html")]
async fn index() -> impl Responder {
    Index
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
        App::new()
            .service(index)
        )
        .bind("localhost:8000")?
        .run()
        .await
}
