use emwin_parse::formats::taf::{TAFReport, TAFReportItem, TAFReportKind};
use sqlx::{Connection, Executor, MySqlPool};

fn encode_taf_kind(kind: &TAFReportKind) -> &'static str {
    match kind {
        TAFReportKind::Report => "REPORT",
        TAFReportKind::Amendment => "AMENDMENT",
        TAFReportKind::Correction => "CORRECTION",
    }
}

/// Insert a new TAF report into the database
pub async fn insert_taf(pool: &MySqlPool, taf: &TAFReportItem) -> Result<u64, sqlx::Error> {
    let item_id = sqlx::query!(
r#"
    INSERT INTO taf-item (kind, country, origin, from, to)
    VALUES (?, ?, ?, ?, ?)
    RETURNING item-id
"#,
    )
    .bind(encode_taf_kind(&taf.kind))
    .execute(pool)
    .await?;

    Ok(item_id)
}

/// Setup all data tables needed to record weather data
pub async fn setup_tables(conn: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
    CREATE TABLE IF NOT EXISTS taf-item (
        item-id int NOT NULL PRIMARY KEY AUTO_INCREMENT,
        kind ENUM('REPORT', 'AMENDMENT', 'CORRECTION'),
        country CHAR(4) NOT NULL,
        origin DATETIME NOT NULL,
        from DATETIME NOT NULL,
        to DATETIME NOT NULL,
        data-id int FOREIGN KEY REFERENCES taf-data(data-id),
    )
    ",
    )
    .execute(conn)
    .await?;

    sqlx::query(
        "
    CREATE TABLE IF NOT EXISTS taf-data (
        taf-data-id int NOT NULL PRIMARY KEY AUTO_INCREMENT,
        wind JSON,
        visibility FLOAT,
        weather JSON,
        cloud JSON,
    )
    ",
    )
    .execute(conn)
    .await?;

    sqlx::query(
        "
    CREATE TABLE IF NOT EXISTS taf-group (
        group-id int NOT NULL PRIMARY KEY AUTO_INCREMENT,
        item-id int NOT NULL FOREIGN KEY REFERENCES taf-item(item-id),
        kind ENUM('TIMED', 'CHANGE', 'TEMP', 'PROB'),
        probability float,
        from DATETIME NOT NULL,
        to DATETIME,
        data-id int NOT NULL FOREIGN KEY REFERENCES taf-data(data-id)
    )
    ",
    )
    .execute(conn)
    .await?;

    Ok(())
}
