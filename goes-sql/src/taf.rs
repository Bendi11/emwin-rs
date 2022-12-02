use chrono::NaiveDate;
use goes_parse::formats::taf::{TAFReportItem, TAFReportItemGroupKind};
use sqlx::Row;
use uom::si::length::meter;

use crate::GoesSqlContext;

impl GoesSqlContext {
    /// Insert a TAF report item into the database connection, returning the ID of the inserted
    /// row of the `weather.taf_item` table
    pub async fn insert_taf(
        &self,
        month: NaiveDate,
        taf: &TAFReportItem,
    ) -> Result<u64, sqlx::Error> {
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

        self.insert_significant_weather(data, &taf.significant_weather)
            .await?;
        self.insert_cloud_report(data, &taf.clouds).await?;
        if let Some(ref wind) = taf.wind {
            self.insert_wind_summary(data, wind).await?;
        }

        for group in taf.groups.iter() {
            let group_data = self.insert_data().await?;

            sqlx::query(
                r#"
INSERT INTO weather.taf_group (item_id, data_id, kind, from_off, to_off, visibility, probability)
VALUES (?, ?, ?, ?, ?, ?, ?);
"#,
            )
            .bind(item_id)
            .bind(group_data)
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

            self.insert_significant_weather(group_data, &group.weather)
                .await?;
            self.insert_cloud_report(group_data, &group.clouds).await?;
            if let Some(ref wind) = group.wind {
                self.insert_wind_summary(group_data, wind).await?;
            }
        }

        Ok(item_id)
    }
}
