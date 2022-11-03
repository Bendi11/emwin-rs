//! Parsing for FM 42 AMDAR

use std::str::FromStr;

use chrono::NaiveTime;
use nom::{IResult, combinator::{map_res, all_consuming}, bytes::complete::{take, take_till}, sequence::{preceded, Tuple, tuple}, character::complete::{space1, anychar}, branch::alt};
use uom::si::{f32::{Pressure, Angle, Length, ThermodynamicTemperature, MassDensity}, angle::degree, pressure::hectopascal, length::foot, thermodynamic_temperature::degree_celsius};

use crate::util::TIME_YYGGGG;

use super::{parse_degreesminutes, LatitudeDir, LongitudeDir};


/// A single AMDAR report parsed from FM 42 data
#[derive(Clone, Debug,)]
pub struct AmdarReport {
    
}

#[derive(Clone, Debug)]
pub struct AmdarReportItem {
    pub phase: FlightPhase,
    pub aircraft_identifier: String,
    pub lat: Angle,
    pub lon: Angle,
    pub time: NaiveTime,
    /// Measure in hundreds of feet above the standard datum plane of 1013.2 hPa
    pub pressure_altitude: Length,
    /// Measure of temperature at the given altitude
    pub air_temperature: ThermodynamicTemperature,
    pub humidity_or_dew_point: HumidityOrDewPoint,
}


#[derive(Clone, Copy, Debug,)]
pub enum HumidityOrDewPoint {
    /// Ranging from [0., 1.] for relative humidity
    RelativeHumidity(f32),
    /// Dew point temperature
    DewPoint(ThermodynamicTemperature),
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
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, phase) = map_res(
            take(3usize),
            |s: &str| s.parse::<FlightPhase>()
        )(input)?;

        let (input, aircraft_identifier) = take_till(char::is_whitespace)(input)?;
        let (input, (angle, dir)) = preceded(
            space1,
            tuple(
                (
                    parse_degreesminutes,
                    map_res(
                        take(1usize),
                        |c: &str| c.parse::<LatitudeDir>()
                    )
                )
            )
        )(input)?;

        let lat = Angle::new::<degree>(dir.to_north(angle));

        let (input, (angle, dir)) = preceded(
            space1,
            tuple(
                (
                parse_degreesminutes,
                    map_res(
                        take(1usize),
                        |c: &str| c.parse::<LongitudeDir>(),
                    )
                )
            )
        )(input)?;
        
        let lon = Angle::new::<degree>(dir.to_east(angle));

        let (input, time) = preceded(space1, map_res(take(6), |s: &str| NaiveTime::parse_from_str(s, TIME_YYGGGG)))(input)?;

        let (input, alt_sign) = preceded(
            space1,
            map_res(
                anychar,
                |c: char| Ok(match c {
                    'F' => 1f32,
                    'A' => -1f32,
                    other => return Err("Unknown pressure altimiter sign character"),
                })
            )
        )(input)?;

        let (input, pressure_altitude) = map_res(
            take(3),
            |s: &str| s.parse::<f32>()
        )(input)?;

        let pressure_altitude = Length::new::<foot>(alt_sign * pressure_altitude);

        let (input, air_temperature) = preceded(space1, Self::parse_temp)(input)?;

        let (input, humidity_or_dew_point) = preceded(
            space1,
            map_res(
                take_till(|c: char| c.is_whitespace()),
                |s: &str| match s.len() {
                    5 => Ok(HumidityOrDewPoint::DewPoint(Self::parse_temp(s).map(|(_, r)| r)?)),
                    3 => Ok(
                        HumidityOrDewPoint::RelativeHumidity(
                            s.parse::<f32>()
                                .map_err(|e| nom::error::Error::new(s, nom::error::ErrorKind::Float))? / 100f32
                        )
                    ),
                }
            ),
        )(input)?;

    }

    fn parse_temp(input: &str) -> IResult<&str, ThermodynamicTemperature> {
        let (input, temperature_sign) = map_res(
            take(2usize),
            |s: &str| Ok(match s {
                "PS" => 1f32,
                "MS" => -1f32,
                _ => return Err("Invalid temperature sign string"),
            })
        )(input)?;

        let (input, temperature) = map_res(
            take(3usize),
            |s: &str| s.parse::<f32>(),
        )(input)?;

        Ok((input, ThermodynamicTemperature::new::<degree_celsius>(temperature / 10f32 * temperature_sign)))
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
