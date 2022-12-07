use actix_files::NamedFile;
use actix_web::{
    error,
    web::{self, Data},
    Responder, Result, Scope,
};
use goes_cfg::Config;
use sqlx::{MySqlPool, Row};

use crate::map_path;

use super::search::{img_single_ep, QueryForm, TimeQuery};

pub fn latest_scope() -> Scope {
    web::scope("/latest")
        //.service(web::resource("/fd.html").to(latest_fd))
        .service(web::resource("/fd_fc.jpg").to(latest_fd_fc))
}

async fn latest_fd_fc(sql: Data<MySqlPool>) -> Result<impl Responder> {
    img_single_ep(sql, &QueryForm {
        time: TimeQuery::Latest,
        sector: Some("FULL_DISK".to_owned()),
        channel: Some("FULL_COLOR".to_owned()),
        acronym: None,
        satellite: None,
    }).await
}
