use sqlx::Connection;


/// Setup all data tables needed to record weather data
pub async fn setup_tables(conn: impl Connection) -> Result<(), sqlx::Error> {
    let taf_tbl = sqlx::query("
    CREATE TABLE IF NOT EXISTS taf-item (
        item-id int NOT NULL PRIMARY KEY AUTO_INCREMENT,
        kind ENUM('REPORT', 'AMENDMENT', 'CORRECTION'),
        country CHAR(4) NOT NULL,
        origin DATETIME NOT NULL,
        from DATETIME NOT NULL,
        to DATETIME NOT NULL,
        data-id int FOREIGN KEY REFERENCES taf-data(data-id),
    )
    ").execute(conn).await;

    Ok(())
}
