//! Encoding and decoding decoded EMWIN files from a database
use goes_parse::formats::codes::{
    clouds::{CloudAmount, CloudReport},
    weather::{
        SignificantWeather, SignificantWeatherDescriptor, SignificantWeatherIntensity,
        SignificantWeatherPhenomena, SignificantWeatherPrecipitation,
    },
    wind::WindSummary,
};
use sqlx::{MySqlPool, Row};
use uom::si::{angle::radian, length::meter, velocity::meter_per_second};

mod taf;
mod goes;

/// Context containing a database connection used to execute queries for EMWIN data
#[derive(Clone, Debug)]
pub struct GoesSqlContext {
    conn: MySqlPool,
}

impl GoesSqlContext {
    const UP: &str = include_str!("./sql/up.sql");

    pub fn new(conn: MySqlPool) -> Self {
        Self { conn }
    }

    /// Create all datatables if they do not yet exist
    pub async fn init(&self) -> Result<(), sqlx::Error> {
        sqlx::query(Self::UP).execute(&self.conn).await.map(|_| ())
    }

    /// Create a new data ID and return the ID
    async fn insert_data(&self) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
INSERT INTO weather.data ()
VALUES ()
RETURNING id;
"#,
        )
        .fetch_one(&self.conn)
        .await?
        .try_get(0usize)
    }

    /// Create multiple rows in the `weather.significant_weather` table for each element of
    /// `weather`
    async fn insert_significant_weather(
        &self,
        data_id: u64,
        weather: &[SignificantWeather],
    ) -> Result<(), sqlx::Error> {
        for weather in weather {
            sqlx::query(
                r#"
INSERT INTO weather.significant_weather (data_id, intensity, descriptor, precipitation, phenomena)
VALUES (?, ?, ?, ?, ?);
"#,
            )
            .bind(data_id)
            .bind(match weather.intensity {
                SignificantWeatherIntensity::Light => "LIGHT",
                SignificantWeatherIntensity::Moderate => "MODERATE",
                SignificantWeatherIntensity::Heavy => "HEAVY",
                SignificantWeatherIntensity::Vicinity => "VICINITY",
            })
            .bind(weather.descriptor.map(|d| match d {
                SignificantWeatherDescriptor::Shallow => "SHALLOW",
                SignificantWeatherDescriptor::Patches => "PATCHES",
                SignificantWeatherDescriptor::Partial => "PARTIAL",
                SignificantWeatherDescriptor::LowDrifting => "LOW_DRIFTING",
                SignificantWeatherDescriptor::Blowing => "BLOWING",
                SignificantWeatherDescriptor::Showers => "SHOWERS",
                SignificantWeatherDescriptor::Thunderstorm => "THUNDERSTORM",
                SignificantWeatherDescriptor::Supercooled => "SUPERCOOLED",
            }))
            .bind(weather.precipitation.bits())
            .bind(weather.phenomena.map(|p| match p {
                SignificantWeatherPhenomena::Mist => "MIST",
                SignificantWeatherPhenomena::Fog => "FOG",
                SignificantWeatherPhenomena::Smoke => "SMOKE",
                SignificantWeatherPhenomena::Ash => "ASH",
                SignificantWeatherPhenomena::Dust => "DUST",
                SignificantWeatherPhenomena::Sand => "SAND",
                SignificantWeatherPhenomena::Haze => "HAZE",
                SignificantWeatherPhenomena::DustSandSwirls => "DUST_SANDSWIRLS",
                SignificantWeatherPhenomena::Squalls => "SQUALLS",
                SignificantWeatherPhenomena::FunnelCloud => "FUNNEL_CLOUD",
                SignificantWeatherPhenomena::SandStorm => "SANDSTORM",
                SignificantWeatherPhenomena::DustStorm => "DUSTSTORM",
            }))
            .execute(&self.conn)
            .await?;
        }

        Ok(())
    }

    async fn insert_cloud_report(
        &self,
        data_id: u64,
        clouds: &[CloudReport],
    ) -> Result<(), sqlx::Error> {
        for clouds in clouds {
            sqlx::query(
                r#"
INSERT INTO weather.cloud_report (data_id, amount, altitude)
VALUES (?, ?, ?);
"#,
            )
            .bind(data_id)
            .bind(clouds.amount.map(|amt| match amt {
                CloudAmount::Few => "FEW",
                CloudAmount::Scattered => "SCATTERED",
                CloudAmount::Broken => "BROKEN",
                CloudAmount::Overcast => "OVERCAST",
            }))
            .bind(clouds.altitude.get::<meter>())
            .execute(&self.conn)
            .await?;
        }

        Ok(())
    }

    async fn insert_wind_summary(
        &self,
        data_id: u64,
        wind: &WindSummary,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
INSERT INTO weather.wind_summary (data_id, angle, speed, max_speed)
VALUES (?, ?, ?, ?);
"#,
        )
        .bind(data_id)
        .bind(wind.direction.get::<radian>())
        .bind(wind.speed.get::<meter_per_second>())
        .bind(wind.max_speed.map(|s| s.get::<meter_per_second>()))
        .execute(&self.conn)
        .await?;

        Ok(())
    }
}
