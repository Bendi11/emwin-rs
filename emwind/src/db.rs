use emwin_parse::formats::taf::{TAFReport, TAFReportItem, TAFReportKind};
use sqlx::{Connection, Executor, MySqlPool};

pub const UP: &str = include_str!("./sql/up.sql");

fn encode_taf_kind(kind: &TAFReportKind) -> &'static str {
    match kind {
        TAFReportKind::Report => "REPORT",
        TAFReportKind::Amendment => "AMENDMENT",
        TAFReportKind::Correction => "CORRECTION",
    }
}

/// Insert a new TAF report into the database
pub async fn insert_taf(pool: &MySqlPool, taf: &TAFReportItem) -> Result<u64, sqlx::Error> {
    
}

/// Setup all data tables needed to record weather data
pub async fn setup_tables(conn: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query(UP)
        .execute(conn)
        .await?;
    Ok(())
}
