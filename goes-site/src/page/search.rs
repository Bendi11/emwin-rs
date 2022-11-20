use std::collections::HashMap;
use futures::TryStreamExt;
use goes_cfg::Config;
use serde::de::Error;

use actix_web::{web::{Data, Form, self}, Result, Responder, Scope, error};
use askama::Template;
use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer};
use sqlx::{MySqlPool, QueryBuilder, MySql, Row};

use crate::map_path;

#[derive(Template)]
#[template(path="search_results.html")]
struct SearchResults {
    images: Vec<String>,
}

#[derive(Template)]
#[template(path="search_query.html")]
struct SearchQuery;


pub fn search_scope() -> Scope {
    web::scope("/search")
        .route("img.html", web::get().to(search_query))
        .route("result-img.html", web::post().to(search_results))
}

#[derive(Debug, Deserialize)]
pub struct QueryForm {
    #[serde(rename="from_dt")]
    #[serde(deserialize_with="deserialize_input_dt")]
    from_dt: Option<NaiveDateTime>,
    #[serde(deserialize_with="deserialize_input_dt")]
    to_dt: Option<NaiveDateTime>,
    #[serde(rename="acronym-select")]
    pub acronym: String,
    #[serde(rename="channel-select")]
    pub channel: String,
    #[serde(rename="sector-select")]
    pub sector: String,
    #[serde(rename="satellite-select")]
    pub satellite: String,

}

fn deserialize_input_dt<'d, D: Deserializer<'d>>(d: D) -> Result<Option<NaiveDateTime>, D::Error>
where D::Error: serde::de::Error {
    let s = <&str as Deserialize>::deserialize(d)?;
    

    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M")
            .map_err(D::Error::custom)?
        ))
    }
}

pub async fn search_results(sql: Data<MySqlPool>, cfg: Data<Config>, form: Form<QueryForm>) -> Result<impl Responder> {
    log::error!("{:#?}", form); 
    
    let mut qb: QueryBuilder<MySql> = QueryBuilder::new(
r#"SELECT (file_name) FROM goesimg.files WHERE (acronym="#
    );

    qb.push_bind(&form.acronym);
    qb.push(" AND channel=");
    qb.push_bind(&form.channel);

    qb.push(" AND sector=");
    qb.push_bind(&form.sector);

    qb.push(" AND satellite=");
    qb.push_bind(&form.satellite);

    match (form.from_dt, form.to_dt) {
        (Some(from), Some(to)) => {
            qb.push(" AND (start_dt BETWEEN ");
            qb.push_bind(from);
            qb.push(" AND ");
            qb.push_bind(to);
            qb.push(")");
        },
        (Some(from), None) => {
            qb.push("AND start_dt>=");
            qb.push_bind(from);
        },
        (None, Some(to)) => {
            qb.push("AND start_dt<=");
            qb.push_bind(to);
        },
        (None, None) => (),
    }

    qb.push(") LIMIT 5;");
    let mut query = qb
        .build()
        .fetch(sql.get_ref());
    
    let mut images = vec![];
    while let Some(row) = query.try_next().await.map_err(|e| error::ErrorBadRequest(e))? {
        images.push(row
            .try_get::<&str, _>(0)
            .map(map_path(cfg.get_ref()))
            .map_err(|e| error::ErrorBadRequest(e))?);
    }

    Ok(SearchResults { images })
}

pub async fn search_query() -> Result<impl Responder> {
    Ok(SearchQuery)
}
