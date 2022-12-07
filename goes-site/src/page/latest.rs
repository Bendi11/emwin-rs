use actix_files::NamedFile;
use actix_web::{
    error,
    web::{self, Data},
    Responder, Result, Scope,
};
use goes_cfg::Config;
use sqlx::{MySqlPool, Row};

use crate::map_path;

pub fn latest_scope() -> Scope {
    web::scope("/latest")
        //.service(web::resource("/fd.html").to(latest_fd))
        .service(web::resource("/fd_fc.jpg").to(latest_fd_fc))
}

async fn latest_fd_fc(sql: Data<MySqlPool>, cfg: Data<Config>) -> Result<impl Responder> {
    let fd = sqlx::query(
r#"
SELECT (file_name)
FROM goesimg.files
WHERE start_dt=(SELECT max(start_dt) FROM goesimg.files WHERE sector='FULL_DISK' AND channel='FULL_COLOR') AND sector='FULL_DISK' AND channel='FULL_COLOR';
"#
    )
        .fetch_one(sql.get_ref())
        .await
        .map_err(|e| error::ErrorBadRequest(e))
        .and_then(|v| v
            .try_get::<&str, _>(0)
            .map_err(|e| error::ErrorBadRequest(e))
            .and_then(map_path(cfg.get_ref()))
            .map(std::path::Path::to_owned)
        )?;

    Ok(NamedFile::open(cfg.img_dir.join(fd))?)
}
