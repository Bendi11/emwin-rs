//! Parsing for FM 42 AMDAR

use std::str::FromStr;

use chrono::NaiveTime;
use nom::{
    bytes::complete::{tag, take, take_till},
    character::{
        complete::{anychar, multispace0, multispace1, space0, space1},
        streaming::char,
    },
    combinator::{map_res, opt},
    multi::separated_list1,
    sequence::{preceded, terminated, tuple},
    IResult, branch::alt,
};
use uom::si::{
    angle::degree,
    f32::{Angle, Length, ThermodynamicTemperature, Velocity},
    length::foot,
    thermodynamic_temperature::degree_celsius,
    velocity::knot,
};

use crate::{header::WMOProductIdentifier, util::TIME_YYGGGG};

use super::{parse_degreesminutes, LatitudeDir, LongitudeDir};

/// A single AMDAR report parsed from FM 42 data
#[derive(Clone, Debug)]
pub struct AmdarReport {
    pub header: WMOProductIdentifier,
    pub items: Vec<AmdarReportItem>,
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
    pub humidity_or_dew_point: Option<HumidityOrDewPoint>,
    pub true_wind_direction: Angle,
    pub wind_speed: Velocity,
    pub turbulence: Option<Turbulence>,
    pub navigation_system: Option<NavigationSystem>,
    pub transmission_system: Option<TransmissionSystem>,
    pub precision: Option<TemperaturePrecision>,
}

#[derive(Clone, Copy, Debug)]
pub enum TransmissionSystem {
    ASDAR,
    /// false if ACARS not operative
    ASDARWithACARS(bool),
    ACARS,
    /// false if ASDAR not operative
    ACARSWithASDAR(bool),
}

#[derive(Clone, Copy, Debug)]
pub enum TemperaturePrecision {
    /// +/- 2.0 C
    Low,
    /// +/- 1.0 C
    High,
}

#[derive(Clone, Copy, Debug)]
pub enum NavigationSystem {
    Intertial,
    OMEGA,
}

#[derive(Clone, Copy, Debug)]
pub enum Turbulence {
    None,
    Light,
    Moderate,
    Severe,
}

#[derive(Clone, Copy, Debug)]
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

impl AmdarReport {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, header) = WMOProductIdentifier::parse(input)?;
        let (input, _) = preceded(multispace1, preceded(tag("AMDAR "), take(4usize)))(input)?;

        let (input, items) = separated_list1(
            multispace1,
            preceded(multispace0, AmdarReportItem::parse),
        )(input)?;

        Ok((input, Self { header, items }))
    }
}

impl AmdarReportItem {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, phase) = map_res(take(3usize), |s: &str| s.parse::<FlightPhase>())(input)?;

        let (input, aircraft_identifier) = preceded(space1, take_till(char::is_whitespace))(input)?;
        let (input, (angle, dir)) = preceded(
            space1,
            tuple((
                parse_degreesminutes::<2>,
                map_res(take(1usize), |c: &str| c.parse::<LatitudeDir>()),
            )),
        )(input)?;

        let lat = Angle::new::<degree>(dir.to_north(angle));

        let (input, (angle, dir)) = preceded(
            space1,
            tuple((
                parse_degreesminutes::<3>,
                map_res(take(1usize), |c: &str| c.parse::<LongitudeDir>()),
            )),
        )(input)?;

        let lon = Angle::new::<degree>(dir.to_east(angle));

        let (input, time) = preceded(
            space1,
            map_res(take(6usize), |s: &str| {
                NaiveTime::parse_from_str(s, TIME_YYGGGG)
            }),
        )(input)?;

        let (input, alt_sign) = preceded(
            space1,
            map_res(anychar, |c: char| {
                Ok(match c {
                    'F' => 1f32,
                    'A' => -1f32,
                    _ => return Err("Unknown pressure altimiter sign character"),
                })
            }),
        )(input)?;

        let (input, pressure_altitude) = map_res(take(3usize), |s: &str| s.parse::<f32>())(input)?;

        let pressure_altitude = Length::new::<foot>(alt_sign * pressure_altitude);

        let (input, air_temperature) = preceded(space1, Self::parse_temp)(input)?;

        let (input, humidity_or_dew_point) = preceded(
            space1,
            opt(map_res(
                take_till(|c: char| c.is_whitespace()),
                |s: &str| match s.len() {
                    5 => Ok::<HumidityOrDewPoint, nom::Err<nom::error::Error<&str>>>(
                        HumidityOrDewPoint::DewPoint(Self::parse_temp(s).map(|(_, r)| r)?),
                    ),
                    3 => Ok(HumidityOrDewPoint::RelativeHumidity(
                        s.parse::<f32>().map_err(|_| {
                            nom::Err::Error(nom::error::Error::new(s, nom::error::ErrorKind::Float))
                        })? / 100f32,
                    )),
                    _ => {
                        return Err(nom::Err::Error(nom::error::Error::new(
                            s,
                            nom::error::ErrorKind::OneOf,
                        )))
                    }
                },
            )),
        )(input)?;

        let (input, true_wind_direction) = preceded(
            space0,
            terminated(map_res(take(3usize), |s: &str| s.parse::<f32>()), char('/')),
        )(input)?;

        let true_wind_direction = Angle::new::<degree>(true_wind_direction);

        let (input, wind_speed) = map_res(take(3usize), |s: &str| s.parse::<f32>())(input)?;

        let wind_speed = Velocity::new::<knot>(wind_speed);

        let (input, turbulence) = preceded(
            space1,
            preceded(
                tag("TB"),
                map_res(anychar, |c: char| {
                    Ok(Some(match c {
                        '0' => Turbulence::None,
                        '1' => Turbulence::Light,
                        '2' => Turbulence::Moderate,
                        '3' => Turbulence::Severe,
                        '/' => return Ok(None),
                        _ => return Err("invalid turbulence value"),
                    }))
                }),
            ),
        )(input)?;

        let (input, (navigation_system, transmission_system, precision)) = preceded(
            tuple((space1, char('S'))),
            tuple((
                map_res(anychar, |c: char| {
                    Ok(Some(match c {
                        '0' => NavigationSystem::Intertial,
                        '1' => NavigationSystem::OMEGA,
                        '/' => return Ok(None),
                        _ => return Err("invalid navigation system character"),
                    }))
                }),
                map_res(anychar, |c: char| {
                    Ok(Some(match c {
                        '0' => TransmissionSystem::ASDAR,
                        '1' => TransmissionSystem::ASDARWithACARS(false),
                        '2' => TransmissionSystem::ASDARWithACARS(true),
                        '3' => TransmissionSystem::ACARS,
                        '4' => TransmissionSystem::ACARSWithASDAR(false),
                        '5' => TransmissionSystem::ACARSWithASDAR(true),
                        '/' => return Ok(None),
                        _ => return Err("invalid transmission system character"),
                    }))
                }),
                map_res(anychar, |c: char| {
                    Ok(Some(match c {
                        '1' => TemperaturePrecision::Low,
                        '0' => TemperaturePrecision::High,
                        '/' => return Ok(None),
                        _ => return Err("invalid temperature precision character"),
                    }))
                }),
            )),
        )(input)?;

        let (input, _) = alt((
            tag("="),
            terminated(
                take_till(|c: char| c == '=' || c == '\n'),
                char('=')
            )
        ))(input)?;

        Ok((
            input,
            Self {
                phase,
                aircraft_identifier: aircraft_identifier.to_owned(),
                lat,
                lon,
                time,
                pressure_altitude,
                air_temperature,
                humidity_or_dew_point,
                true_wind_direction,
                wind_speed,
                turbulence,
                navigation_system,
                transmission_system,
                precision,
            },
        ))
    }

    fn parse_temp(input: &str) -> IResult<&str, ThermodynamicTemperature> {
        let (input, temperature_sign) = map_res(take(2usize), |s: &str| {
            Ok(match s {
                "PS" => 1f32,
                "MS" => -1f32,
                _ => return Err("Invalid temperature sign string"),
            })
        })(input)?;

        let (input, temperature) = map_res(take(3usize), |s: &str| s.parse::<f32>())(input)?;

        Ok((
            input,
            ThermodynamicTemperature::new::<degree_celsius>(temperature / 10f32 * temperature_sign),
        ))
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

#[derive(Clone, Copy, Debug)]
pub struct InvalidFlightPhase;

impl std::fmt::Display for InvalidFlightPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid flight phase string")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const AMDAR: &str = include_str!("./test/amdar.txt");

    #[test]
    pub fn test_amdar() {
        let (_, amdar) = AmdarReport::parse(AMDAR).unwrap();
        assert_eq!(amdar.items.len(), 8);
    }
}
