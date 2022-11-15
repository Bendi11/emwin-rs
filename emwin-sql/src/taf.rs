use chrono::NaiveDate;
use emwin_parse::formats::taf::{TAFReportItem, TAFReportItemGroupKind};
use sqlx::Row;
use uom::si::{f32::Length, length::meter};

use crate::EmwinSqlContext;


impl EmwinSqlContext {
    /// Insert a TAF report item into the database connection, returning the ID of the inserted
    /// row of the `weather.taf_item` table
    pub async fn insert_taf(&self, month: NaiveDate, taf: &TAFReportItem) -> Result<u64, sqlx::Error> {
        let data = self.insert_data().await?;

        let item_id = sqlx::query(
r#"
INSERT INTO weather.taf_item (month, country, origin_off, from_off, to_off, visibility, data_id)
VALUES (?, ?, ?, ?, ?, ?, ?)
RETURNING id;
"#,
        )
        .bind(month)
        .bind(taf.country.code.iter().collect::<String>())
        .bind(taf.origin_date.num_seconds())
        .bind(taf.time_range.0.num_seconds())
        .bind(taf.time_range.1.num_seconds())
        .bind(taf.horizontal_vis.map(|v| v.get::<meter>()))
        .bind(data)
        .fetch_one(&self.conn)
        .await?
        .try_get(0)?;

        self.insert_significant_weather(data, &taf.significant_weather).await?; 

        self.insert_cloud_report(data, &taf.clouds).await?;

        for group in taf.groups.iter() {
            sqlx::query(
r#"
INSERT INTO weather.taf_group (data_id, kind, from_off, to_off, visibility, probability)
VALUES (?, ?, ?, ?, ?, ?);
"#
            )
            .bind(data)
            .bind(match group.kind {
                TAFReportItemGroupKind::TimeIndicator(..) => "TIMED",
                TAFReportItemGroupKind::Change(..) => "CHANGE",
                TAFReportItemGroupKind::TemporaryChange { .. } => "TEMP",
                TAFReportItemGroupKind::Probable { .. } => "PROB",
            })
            .bind(group.kind.from().num_seconds())
            .bind(group.kind.to().map(|t| t.num_seconds()))
            .bind(group.visibility.map(|v| v.get::<meter>()))
            .bind(group.kind.probability())
            .execute(&self.conn)
            .await?;
        }

        Ok(item_id)
    }
}
