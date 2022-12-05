use goes_parse::formats::{metar::RunwayState, RunwayDesignatorDirection, codes::runway::RunwayDeposits};

use crate::GoesSqlContext;


impl GoesSqlContext {
    
    async fn insert_runway_state(&self, state: &RunwayState, data_id: u64) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
insert into weather.metar_runway_state
(runway, direction, deposits, contamination_from, contamination_to, deposits_depth_status, deposits_depth, braking_action_status, friction_coefficient)
values (?, ?, ?, ?, ?, ?, ?, ?, ?);
            "#
        )
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
    }
}
