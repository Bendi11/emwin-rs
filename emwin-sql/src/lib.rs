//! Encoding and decoding decoded EMWIN files from a database
use sqlx::Executor;

mod taf;

/// Context containing a database connection used to execute queries for EMWIN data
#[derive(Clone, Debug,)]
pub struct EmwinSqlContext<E: for<'c> Executor<'c>> {
    conn: E,
}

impl<E: for<'c> Executor<'c>> EmwinSqlContext<E> {
    pub fn new(conn: E) -> Self {
        Self { conn }
    }
}
