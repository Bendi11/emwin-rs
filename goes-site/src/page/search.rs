use std::path::PathBuf;

use actix_files::NamedFile;
use futures::TryStreamExt;
use goes_cfg::Config;
use serde::de::Error;

use actix_web::{web::{Data, self, Json}, Result, Responder, Scope, error::{self, ErrorInternalServerError}, post, get};
use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer};
use sqlx::{MySqlPool, QueryBuilder, MySql, Row};

use crate::map_path;


pub fn search_scope() -> Scope {
    web::scope("/search")
        .service(img_multi)
        .service(img_single_post)
        .service(img_single_get)
}

#[derive(Debug, Deserialize)]
pub enum TimeQuery {
    #[serde(rename="latest")]
    Latest,
    #[serde(rename="from")]
    From(
        #[serde(deserialize_with="deserialize_input_dt")]
        NaiveDateTime
    ),
    #[serde(rename="to")]
    To(
        #[serde(deserialize_with="deserialize_input_dt")]
        NaiveDateTime,
    ),
    #[serde(rename="within")]
    Within {
        #[serde(deserialize_with="deserialize_input_dt")]
        from: NaiveDateTime,
        #[serde(deserialize_with="deserialize_input_dt")]
        to: NaiveDateTime,
    },
}

#[derive(Debug, Deserialize)]
pub struct QueryForm {
    #[serde(flatten)]
    pub time: TimeQuery, 
    pub acronym: Option<String>,
    pub channel: Option<String>,
    pub sector: Option<String>,
    pub satellite: Option<String>, 
}

#[derive(Debug, Deserialize)]
pub struct MultiQueryForm {
    #[serde(flatten)]
    pub query: QueryForm,
    pub limit: u16,
    pub page: u16,
}

fn deserialize_input_dt<'d, D: Deserializer<'d>>(d: D) -> Result<NaiveDateTime, D::Error>
where D::Error: serde::de::Error {
    let s = <String as Deserialize>::deserialize(d)?;
    
    Ok(NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M")
        .map_err(D::Error::custom)?
    )
}

fn search_sql(form: &QueryForm, single: bool) -> QueryBuilder<MySql> {
    let mut qb: QueryBuilder<MySql> = QueryBuilder::new(
r#"
with search as (
    select file_name, start_dt from goesimg.files where (
"#
    );

    {
        let mut params = qb.separated(" and ");
        if let Some(ref acronym) = form.acronym {
            params.push("acronym=");
            params.push_bind_unseparated(acronym);
        }
        if let Some(ref channel) = form.channel {
            params.push("channel=");
            params.push_bind_unseparated(channel);
        }
        if let Some(ref sector) = form.sector {
            params.push("sector=");
            params.push_bind_unseparated(sector);
        }
        if let Some(ref satellite) = form.satellite {
            params.push("satellite=");
            params.push_bind_unseparated(satellite);
        }
    }

    qb.push(
        r#"
))
select file_name from search
"#
    );
   
    match form.time {
        TimeQuery::Within { from, to } => {
            qb.push("where (start_dt between ");
            qb.push_bind(from);
            qb.push(" AND ");
            qb.push_bind(to);
            qb.push(")");
        },
        TimeQuery::From(from) => {
            qb.push("where start_dt>=");
            qb.push_bind(from);
        },
        TimeQuery::To(to) => {
            qb.push("where start_dt<=");
            qb.push_bind(to);
        },
        TimeQuery::Latest if single => {
            qb.push("where start_dt=(select max(start_dt) from search)");
        },
        TimeQuery::Latest => {
            qb.push("order by start_dt desc");
        }
    }

    log::error!("{:#?}", qb.sql());
    
    qb
}

fn decode_base64<T: for<'de> Deserialize<'de>>(base64: &[u8]) -> Result<T> {
    serde_json::from_slice::<T>(
        &base64::decode(base64)
            .map_err(error::ErrorBadRequest)?
    ).map_err(error::ErrorBadRequest)
}

#[get("/img/single/{json}")]
pub async fn img_single_get(sql: Data<MySqlPool>, json: web::Path<String>) -> Result<impl Responder> {
    let json = decode_base64(json.as_bytes())?; 
    img_single_ep(sql, &json).await
}

#[post("/img/single")]
pub async fn img_single_post(sql: Data<MySqlPool>, form: Json<QueryForm>) -> Result<impl Responder> {
    img_single_ep(sql, &form).await
}

pub async fn img_single_ep(sql: Data<MySqlPool>, form: &QueryForm) -> Result<impl Responder> {
    let mut qb = search_sql(form, true);
    qb.push(';');

    let file = qb
        .build()
        .fetch_one(sql.get_ref())
        .await
        .map_err(error::ErrorInternalServerError)?
        .try_get::<String, _>(0usize)
        .map_err(error::ErrorInternalServerError)?;

    Ok(NamedFile::open(file)?)
}

#[post("/img/multi")]
pub async fn img_multi(sql: Data<MySqlPool>, cfg: Data<Config>, form: Json<MultiQueryForm>) -> Result<impl Responder> { 
    if form.limit > 10 {
        return Err(error::ErrorBadRequest("`limit` must be less than or equal to 10"))
    }

    let mut qb = search_sql(&form.query, form.limit == 1);

    qb.push("\nlimit ");
    qb.push_bind(form.limit);
    qb.push(" offset ");
    qb.push_bind(form.page.wrapping_mul(form.limit));

    qb.push(";");

    let mut query = qb
        .build()
        .fetch(sql.get_ref());
    
    let mut images: Vec<PathBuf> = vec![];
    while let Some(row) = query.try_next().await.map_err(|e| error::ErrorBadRequest(e))? {
        images.push(row
            .try_get::<&str, _>(0)
            .map_err(|e| ErrorInternalServerError(e))
            .and_then(map_path(cfg.get_ref()))?
            .to_owned()
        );
    }

    Ok(web::Json(images))
}
