//! Encoding and decoding decoded EMWIN files from a database
use emwin_parse::formats::codes::weather::{SignificantWeather, SignificantWeatherIntensity, SignificantWeatherDescriptor, SignificantWeatherPrecipitation};
use sqlx::{Executor, Row, MySqlExecutor};

mod taf;

/// Context containing a database connection used to execute queries for EMWIN data
#[derive(Clone, Debug,)]
pub struct EmwinSqlContext<E: for<'c> MySqlExecutor<'c> + Copy> {
    conn: E,
}

impl<E: for<'c> MySqlExecutor<'c> + Copy> EmwinSqlContext<E> {
    pub fn new(conn: E) -> Self {
        Self { conn }
    }
    
    /// Create a new data ID and return the ID
    async fn insert_data(&self) -> Result<u64, sqlx::Error> {
        sqlx::query(
r#"
INSERT INTO weather.data
VALUES ()
RETURNING id;
"#
        )
            .fetch_one(self.conn)
            .await?
            .try_get(0usize)
    }
    
    /// Create multiple rows in the `weather.significant_weather` table for each element of
    /// `weather`
    async fn insert_significant_weather(&self, data_id: u64, weather: &[SignificantWeather]) -> Result<(), sqlx::Error> {
        for weather in weather {
            sqlx::query(
r#"
INSERT INTP weather.significant_weather (data_id, intensity, descriptor, precipitation, phenomena)
VALUES (?, ?, ?, ?, ?);
"#
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
            .bind({
                let mut s = String::new();
                if weather.precipitation.contains(SignificantWeatherPrecipitation::DRIZZLE) {
                    s.push_str("DRIZZLE,")
                }
                if weather.precipitation.contains(SignificantWeatherPrecipitation::RAIN) {
                    s.push_str("RAIN,")
                }
                if weather.precipitation.contains(SignificantWeatherPrecipitation::SNOW) {
                    s.push_str("SNOW,")
                }
                if weather.precipitation.contains(SignificantWeatherPrecipitation::SNOWGRAIN) {
                    s.push_str("SNOWGRAIN,")
                }
                if weather.precipitation.contains(SignificantWeatherPrecipitation::ICEPELLET) {
                    s.push_str("ICEPELLET,")
                }
                if weather.precipitation.contains(SignificantWeatherPrecipitation::HAIL) {
                    s.push_str("HAIL,")
                }
                if weather.precipitation.contains(SignificantWeatherPrecipitation::SMALLHAIL) {
                    s.push_str("SMALLHAIL,")
                }
                if weather.precipitation.contains(SignificantWeatherPrecipitation::UNKNOWN) {
                    s.push_str("")
                }

                s
            })
        }

        Ok(())
    }
}
