use chrono::{Duration, NaiveDateTime, NaiveDate};
use emwin_parse::formats::taf::{TAFReportItem, TAFReportKind, TAFReportItemGroupKind};
use sqlx::{Executor, MySql, MySqlExecutor, Row};
use uom::si::{f32::Length, length::meter};

use crate::EmwinSqlContext;


impl EmwinSqlContext {
    /// Insert a TAF report item into the database connection, returning the ID of the inserted
    /// row of the `weather.taf_item` table
    pub async fn insert_taf(&self, month: NaiveDate, taf: &TAFReportItem) -> Result<u64, sqlx::Error> {
        let data = self.insert_data().await?;

        let item_id = sqlx::query(
r#"
INSERT INTO weather.taf_item (month, country, origin_off, from_off, to_off, data_id)
VALUES (?, ?, ?, ?, ?)
RETURNING id;
"#,
        )
        .bind(month)
        .bind(taf.country.code.iter().collect::<String>())
        .bind(taf.origin_date.num_seconds())
        .bind(taf.time_range.0.num_seconds())
        .bind(taf.time_range.1.num_seconds())
        .bind(data)
        .fetch_one(&self.conn)
        .await?
        .try_get(0)?;

        self.insert_taf_visibility(data, taf.horizontal_vis).await?;
        if let Some(weather) = taf.significant_weather {
            self.insert_significant_weather(data, &[weather]).await?; 
        }

        self.insert_cloud_report(data, &taf.clouds).await?;

        for group in taf.groups.iter() {
            sqlx::query(
r#"
INSERT INTO weather.taf_group (item_id, data_id, kind, from_off, to_off, probability)
VALUES (?, ?, ?, ?, ?, ?);
"#
            )
            .bind(item_id)
            .bind(data)
            .bind(match group.kind {
                TAFReportItemGroupKind::TimeIndicator(..) => "TIMED",
                TAFReportItemGroupKind::Change(..) => "CHANGE",
                TAFReportItemGroupKind::TemporaryChange { .. } => "TEMP",
                TAFReportItemGroupKind::Probable { .. } => "PROB",
            })
            .bind(group.kind.from().num_seconds())
            .bind(group.kind.to().map(|t| t.num_seconds()))
            .bind(group.kind.probability())
            .execute(&self.conn)
            .await?;
        }

        Ok(item_id)
    }

    async fn insert_taf_visibility(&self, data_id: u64, vis: Option<Length>) -> Result<(), sqlx::Error> {
        sqlx::query(
r#"
INSERT INTO weather.taf_visbility (data_id, horizontal_visibility)
VALUES (?, ?);
"#
        )
        .bind(data_id)
        .bind(vis.map(|v| v.get::<meter>()))
        .execute(&self.conn)
        .await?;

        Ok(())
    }
}
