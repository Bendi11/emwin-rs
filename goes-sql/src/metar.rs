use goes_parse::formats::{metar::{RunwayState, MetarSeaSurfaceReport, MetarReport, RunwayTrend, EmwinMetarReport, RunwayWindShear}, RunwayDesignatorDirection, codes::{runway::{RunwayDeposits, RunwayDepositDepth, RunwayContaminationLevel, RunwaySurfaceFriction, RunwaySurfaceBrakingAction}, sea::StateOfTheSea}, Compass};
use uom::si::{length::meter, f32::ThermodynamicTemperature, thermodynamic_temperature::degree_celsius, pressure::pascal, angle::radian};

use crate::GoesSqlContext;


impl GoesSqlContext {

    pub async fn insert_metar(&self, emwin: &EmwinMetarReport) -> Result<u64, sqlx::Error> {
        let EmwinMetarReport { month, metar, .. } = emwin;
        let data_id = self.insert_data().await?;
        
        for status in metar.runway_status.iter() {
            self.insert_runway_state(status, data_id).await?;
        }

        for sea in metar.sea.iter() {
            self.insert_metar_sea(sea, data_id).await?;
        }

        for (runway, len, trend) in metar.runway_range.iter() {
            sqlx::query(
                r#"
insert into weather.metar_runway (data_id, runway, direction, len, trend)
values (?, ?, ?, ?, ?);
                "#
            )
            .bind(data_id)
            .bind(runway.num)
            .bind(runway.dir.map(|d| match d {
                RunwayDesignatorDirection::Left => "LEFT",
                RunwayDesignatorDirection::Center => "CENTER",
                RunwayDesignatorDirection::Right => "RIGHT"
            }))
            .bind(len.get::<meter>())
            .bind(match trend {
                RunwayTrend::Closer => "CLOSER",
                RunwayTrend::Farther => "FARTHER",
                RunwayTrend::NoChange => "NO_CHANGE",
            })
            .execute(&self.conn)
            .await?;
        }

        self.insert_significant_weather(data_id, &metar.weather).await?;
        if let Some(ref recent) = metar.recent_weather {
            self.insert_significant_weather(data_id, &[*recent]).await?;
        }

        self.insert_cloud_report(data_id, &metar.clouds).await?;

        sqlx::query(
            r#"
insert into weather.metar (data_id, country, origin, vwind_ex_ccw, vwind_ex_cw, visibility, min_vis, min_vis_dir, air_temp, dewpoint_temp, qnh, runway_wind_shear_within)
values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);
            "#
        )
        .bind(data_id)
        .bind(metar.country.code.iter().collect::<String>())
        .bind(month.checked_add_signed(metar.origin))
        .bind(metar.variable_wind_dir.map(|w| w.extreme_ccw.get::<radian>()))
        .bind(metar.variable_wind_dir.map(|w| w.extreme_cw.get::<radian>()))
        .bind(metar.visibility.map(|v| v.get::<meter>()))
        .bind(metar.minimum_visibility.map(|v| v.visibility.get::<meter>()))
        .bind(metar.minimum_visibility.map(|v| match v.direction {
            Compass::North => "N",
            Compass::NorthEast => "NE",
            Compass::East => "E",
            Compass::SouthEast => "SE",
            Compass::South => "S",
            Compass::SouthWest => "SW",
            Compass::West => "W",
            Compass::NorthWest => "NW",
        }))
        .bind(metar.air_dewpoint_temperature.map(|(a, _)| a.get::<degree_celsius>()))
        .bind(metar.air_dewpoint_temperature.map(|(_, d)| d.get::<degree_celsius>()))
        .bind(metar.qnh.map(|q| q.get::<pascal>()))
        .bind(metar.runway_wind_shear.and_then(|v| match v {
            RunwayWindShear::Within(l) => Some(l.get::<meter>()),
            _ => None,
        }))
        .execute(&self.conn)
        .await?;

        Ok(data_id)
    }

    async fn insert_metar_sea(&self, sea: &MetarSeaSurfaceReport, data_id: u64) -> Result<(), sqlx::Error> {
        match sea {
            MetarSeaSurfaceReport::WaveHeight { temp, height } => {
                sqlx::query(
                r#"
insert into weather.metar_wave_height (data_id, temperature, height)
values (?, ?, ?);
            "#
                )
                .bind(data_id)
                .bind(temp.get::<degree_celsius>())
                .bind(height.get::<meter>())
                .execute(&self.conn)
                .await
                .map(|_| ())
            },
            MetarSeaSurfaceReport::StateOfSea { temp, state } =>
                self.insert_state_of_sea(*temp, *state, data_id).await
        }
    }

    async fn insert_state_of_sea(&self, temp: ThermodynamicTemperature, state: StateOfTheSea, data_id: u64) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
insert into weather.metar_state_of_sea (data_id, temperature, state)
values (?, ?, ?);
            "#
        )
        .bind(data_id)
        .bind(temp.get::<degree_celsius>())
        .bind(match state {
            StateOfTheSea::Glassy => "GLASSY",
            StateOfTheSea::Rippled => "RIPPLED",
            StateOfTheSea::Wavelets => "WAVELETS",
            StateOfTheSea::Slight => "SLIGHT",
            StateOfTheSea::Moderate => "MODERATE",
            StateOfTheSea::Rough => "ROUGH",
            StateOfTheSea::VeryRough => "VERY_ROUGH",
            StateOfTheSea::High => "HIGH",
            StateOfTheSea::VeryHigh => "VERY_HIGH",
            StateOfTheSea::Phenomenal => "PHENOMENAL",
        })
        .execute(&self.conn)
        .await
        .map(|_| ())
    }
    
    async fn insert_runway_state(&self, state: &RunwayState, data_id: u64) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
insert into weather.metar_runway_state
(data_id, runway, direction, deposits, contamination_from, contamination_to, deposits_depth_status, deposits_depth, braking_action_status, friction_coefficient)
values (?, ?, ?, ?, ?, ?, ?, ?, ?);
            "#
        )
        .bind(data_id)
        .bind(state.runway.num)
        .bind(state.runway.dir.map(|dir| match dir {
            RunwayDesignatorDirection::Left => "LEFT",
            RunwayDesignatorDirection::Center => "CENTER",
            RunwayDesignatorDirection::Right => "RIGHT",
        }))
        .bind(match state.deposits {
            RunwayDeposits::Clear => "CLEAR",
            RunwayDeposits::Damp => "DAMP",
            RunwayDeposits::Wet => "WET",
            RunwayDeposits::RimeFrost => "RIME_FROST",
            RunwayDeposits::DrySnow => "DRY_SNOW",
            RunwayDeposits::WetSnow => "WET_SNOW",
            RunwayDeposits::Slush => "SLUSH",
            RunwayDeposits::Ice => "ICE",
            RunwayDeposits::CompactedSnow => "COMPACTED_SNOW",
            RunwayDeposits::FrozenRuts => "FROZEN_RUTS",
            RunwayDeposits::NotReported => "NOT_REPORTED",
        })
        .bind(match state.level {
            RunwayContaminationLevel::Percent { from, .. } => Some(from),
            _ => None,
        })
        .bind(match state.level {
            RunwayContaminationLevel::Percent { to, .. } => Some(to),
            _ => None,
        })
        .bind(match state.depth {
            RunwayDepositDepth::Depth(..) => "REPORTED",
            RunwayDepositDepth::Inoperable => "INOPERABLE",
            RunwayDepositDepth::NotReported => "NOT_REPORTED",
        })
        .bind(match state.depth {
            RunwayDepositDepth::Depth(d) => Some(d.get::<meter>()),
            _ => None,
        })
        .bind(match state.friction {
            RunwaySurfaceFriction::Coefficient(..) => "COEFFICIENT",
            RunwaySurfaceFriction::BrakingAction(b) => match b {
                RunwaySurfaceBrakingAction::Poor => "POOR",
                RunwaySurfaceBrakingAction::MediumPoor => "MEDIUM_POOR",
                RunwaySurfaceBrakingAction::Medium => "MEDIUM",
                RunwaySurfaceBrakingAction::MediumGood => "MEDIUM_GOOD",
                RunwaySurfaceBrakingAction::Good => "GOOD",
            },
            RunwaySurfaceFriction::Unreliable => "UNRELIABLE",
            RunwaySurfaceFriction::NotReported => "NOT_REPORTED",
        })
        .bind(match state.friction {
            RunwaySurfaceFriction::Coefficient(coeff) => Some(coeff),
            _ => None,
        })
        .execute(&self.conn)
        .await
        .map(|_| ())
    }
}
