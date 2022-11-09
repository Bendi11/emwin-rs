use emwin_parse::formats::taf::{TAFReportItem, TAFReportKind};
use sqlx::{Executor, MySql, MySqlExecutor, Row};
use uom::si::{f32::Length, length::meter};

use crate::EmwinSqlContext;


impl<E: for<'c> MySqlExecutor<'c> + Copy> EmwinSqlContext<E> {
    /// Insert a TAF report item into the database connection, returning the ID of the inserted
    /// row of the `weather.taf_item` table
    pub async fn insert_taf(&self, taf: &TAFReportItem) -> Result<u64, sqlx::Error> {
        let data = self.insert_data().await?;

        let item_id = sqlx::query(
r#"
INSERT INTO weather.taf_item (country, origin, from_off, to_off, data_id)
VALUES (?, ?, ?, ?, ?, ?)
RETURNING id;
"#,
        )
        .bind(taf.country.code.iter().collect::<String>())
        .bind(taf.origin_date.num_seconds())
        .bind(taf.time_range.0.num_seconds())
        .bind(taf.time_range.1.num_seconds())
        .bind(data)
        .fetch_one(self.conn)
        .await?
        .try_get(0)?;

        self.insert_taf_visibility(data, taf.horizontal_vis).await?;
         

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
        .execute(self.conn)
        .await?;

        Ok(())
    }
}
