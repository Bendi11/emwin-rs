use std::collections::HashMap;

use actix_web::{web::{Data, Form, self}, Result, Responder, Scope};
use askama::Template;
use sqlx::MySqlPool;

#[derive(Template)]
#[template(path="search_results.html")]
struct SearchResults;

#[derive(Template)]
#[template(path="search_query.html")]
struct SearchQuery;

pub fn search_scope() -> Scope {
    web::scope("/search")
        .route("img.html", web::post().to(search_query))
        .route("result-img.html", web::get().to(search_results))
}

pub async fn search_results(_sql: Data<MySqlPool>, form: Form<HashMap<String, String>>) -> Result<impl Responder> {
    log::error!("{:#?}", form);
    Ok(SearchResults)
}

pub async fn search_query() -> Result<impl Responder> {
    Ok(SearchQuery)
}
