use emwin_parse::formats::taf::TAFReportItem;
use sqlx::Executor;

use crate::EmwinSqlContext;


impl<E: for<'c> Executor<'c>> EmwinSqlContext<E> {
    /// Insert a TAF report item into the database connection, returning the ID of the inserted
    /// row of the `weather.taf_item` table
    pub async fn insert_taf(&self, taf: &TAFReportItem) -> Result<u64, sqlx::Error> {
        
    }
}
