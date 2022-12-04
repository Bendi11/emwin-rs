use std::str::FromStr;

use actix_files::NamedFile;
use futures::TryStreamExt;
use goes_cfg::Config;
use serde::de::Error;

use actix_web::{
    error::{self, ErrorInternalServerError},
    get, post,
    web::{self, Data, Json},
    Responder, Result, Scope,
};
use chrono::{NaiveDateTime, SecondsFormat, Utc, DateTime};
use serde::{Deserialize, Deserializer};
use serde_json::{Map, Value};
use sqlx::{MySql, MySqlPool, QueryBuilder, Row};

use crate::map_path;

pub fn search_scope() -> Scope {
    web::scope("/search")
        .service(img_multi)
        .service(img_single_post)
        .service(img_single_get)
}

#[derive(Debug, Deserialize)]
pub enum TimeQuery {
    #[serde(rename = "latest")]
    Latest,
    #[serde(rename = "from")]
    From(#[serde(deserialize_with = "deserialize_input_dt")] NaiveDateTime),
    #[serde(rename = "to")]
    To(#[serde(deserialize_with = "deserialize_input_dt")] NaiveDateTime),
    #[serde(rename = "within")]
    Within {
        #[serde(deserialize_with = "deserialize_input_dt")]
        from: NaiveDateTime,
        #[serde(deserialize_with = "deserialize_input_dt")]
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
    pub rets: QueryReturn,
    pub limit: u16,
    pub page: u16,
}

bitflags::bitflags! {
    pub struct QueryReturn: u8 {
        const PATH = 0b00000001;
        const DATETIME = 0b00000010;
    }
}

fn deserialize_input_dt<'d, D: Deserializer<'d>>(d: D) -> Result<NaiveDateTime, D::Error>
where
    D::Error: serde::de::Error,
{
    let s = <String as Deserialize>::deserialize(d)?;

    Ok(NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M").map_err(D::Error::custom)?)
}

fn search_sql(form: &QueryForm, single: bool, rets: QueryReturn) -> QueryBuilder<MySql> {
    let mut qb: QueryBuilder<MySql> = QueryBuilder::new(
        r#"
with search as (
    select * from goesimg.files where (
"#,
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

    qb.push(")) select ");
    {
        let mut returns = qb.separated(", ");
        if rets.contains(QueryReturn::PATH) {
            returns.push("file_name");
        }
        if rets.contains(QueryReturn::DATETIME) {
            returns.push("start_dt");
        }
    }

    qb.push(" from search ");

    match form.time {
        TimeQuery::Within { from, to } => {
            qb.push("where (start_dt between ");
            qb.push_bind(from);
            qb.push(" AND ");
            qb.push_bind(to);
            qb.push(")");
        }
        TimeQuery::From(from) => {
            qb.push("where start_dt>=");
            qb.push_bind(from);
        }
        TimeQuery::To(to) => {
            qb.push("where start_dt<=");
            qb.push_bind(to);
        }
        TimeQuery::Latest if single => {
            qb.push("where start_dt=(select max(start_dt) from search)");
        }
        TimeQuery::Latest => {
            qb.push("order by start_dt desc");
        }
    }

    qb
}

fn decode_base64<T: for<'de> Deserialize<'de>>(base64: &[u8]) -> Result<T> {
    serde_json::from_slice::<T>(&base64::decode(base64).map_err(error::ErrorBadRequest)?)
        .map_err(error::ErrorBadRequest)
}

#[get("/img/single/{json}")]
pub async fn img_single_get(
    sql: Data<MySqlPool>,
    json: web::Path<String>,
) -> Result<impl Responder> {
    let json = decode_base64(json.as_bytes())?;
    img_single_ep(sql, &json).await
}

#[post("/img/single")]
pub async fn img_single_post(
    sql: Data<MySqlPool>,
    form: Json<QueryForm>,
) -> Result<impl Responder> {
    img_single_ep(sql, &form).await
}

pub async fn img_single_ep(sql: Data<MySqlPool>, form: &QueryForm) -> Result<impl Responder> {
    let mut qb = search_sql(form, true, QueryReturn::PATH);
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
pub async fn img_multi(
    sql: Data<MySqlPool>,
    cfg: Data<Config>,
    form: Json<MultiQueryForm>,
) -> Result<impl Responder> {
    if form.limit > 10 {
        return Err(error::ErrorBadRequest(
            "`limit` must be less than or equal to 10",
        ));
    }

    if form.rets.is_empty() {
        return Err(error::ErrorBadRequest("`rets` must not be empty"))
    }

    let mut qb = search_sql(&form.query, form.limit == 1, form.rets);

    qb.push("\nlimit ");
    qb.push_bind(form.limit);
    qb.push(" offset ");
    qb.push_bind(form.page.wrapping_mul(form.limit));

    qb.push(";");

    let mut query = qb.build().fetch(sql.get_ref());
    
    let mut images: Vec<Map<String, Value>> = vec![];
    while let Some(row) = query
        .try_next()
        .await
        .map_err(|e| error::ErrorBadRequest(e))?
    {
        let mut v = Map::new();
        if form.rets.contains(QueryReturn::PATH) {
            let path = row.try_get::<&str, _>("file_name")
                .map_err(|e| ErrorInternalServerError(e))
                .and_then(map_path(cfg.get_ref()))?
                .to_owned();
            v.insert("path".to_owned(), serde_json::to_value(path).map_err(ErrorInternalServerError)?);
        }

        if form.rets.contains(QueryReturn::DATETIME) {
            let dt = row.try_get::<DateTime<Utc>, _>("start_dt")
                .map_err(|e| ErrorInternalServerError(e))?;
            v.insert("datetime".to_owned(), serde_json::to_value(dt.to_rfc3339_opts(SecondsFormat::Secs, true)).map_err(ErrorInternalServerError)?);
        }

        images.push(v);
    }

    Ok(web::Json(images))
}

impl<'de> Deserialize<'de> for QueryReturn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
            D: Deserializer<'de> {
        let array = Vec::<&str>::deserialize(deserializer)?;
        let mut this = Self::empty();
        for term in array {
            this |= Self::from_str(term).map_err(|_| D::Error::custom("Unknown query return term"))?;
        }

        Ok(this)
    }
}

impl FromStr for QueryReturn {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "path" => Ok(Self::PATH),
            "datetime" => Ok(Self::DATETIME),
            _ => Err(())
        }
    }
}
