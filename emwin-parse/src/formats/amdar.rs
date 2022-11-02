//! Parsing for FM 42 AMDAR

use std::str::FromStr;

use nom::{IResult, combinator::map_res, bytes::complete::{take, take_till}, sequence::{preceded, Tuple, tuple}, character::complete::space1};

use super::{Latitude, Longitude, parse_degreesminutes};


/// A single AMDAR report parsed from FM 42 data
#[derive(Clone, Debug,)]
pub struct AmdarReport {
    
}

#[derive(Clone, Debug)]
pub struct AmdarReportItem {
    pub phase: FlightPhase,
    pub aircraft_identifier: String,
    pub lat: Latitude,
    pub lon: Longitude,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AircraftReportType {
    Routine,
    MaxWind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlightPhase {
    /// ///
    NA,
    /// LV{R|W}
    LevelFlight(AircraftReportType),
    /// ASC
    Ascent,
    /// DES
    Descent,
    /// UNS
    Unsteady,
}

impl AmdarReportItem {
    fn parse_degreesminutes()

    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, phase) = map_res(
            take(3usize),
            |s: &str| s.parse::<FlightPhase>()
        )(input)?;

        let (input, aircraft_identifier) = take_till(char::is_whitespace)(input)?;
        let (input, (angle, dir)) = preceded(
            space1,
            tuple(
                (parse_degreesminutes, map_res(
                    take(1usize),
                    |c: &str| c.parse::<LatitudeDir>()
                )
                )
            )
        )(input)?;
    }
}

impl FromStr for FlightPhase {
    type Err = InvalidFlightPhase;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "///" => Self::NA,
            "LVR" => Self::LevelFlight(AircraftReportType::Routine),
            "LVW" => Self::LevelFlight(AircraftReportType::MaxWind),
            "ASC" => Self::Ascent,
            "DES" => Self::Descent,
            "UNS" => Self::Unsteady,
            _ => return Err(InvalidFlightPhase),
        })
    }
}


#[derive(Clone, Copy, Debug,)]
pub struct InvalidFlightPhase;

impl std::fmt::Display for InvalidFlightPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid flight phase string")
    }
}
