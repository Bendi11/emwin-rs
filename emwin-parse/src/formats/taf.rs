use chrono::NaiveTime;
use nom::{IResult, combinator::{recognize, map_opt, opt, map_res}, bytes::complete::take, character::complete::anychar, branch::alt};
use uom::si::f32::{Angle, Velocity, Length};

use crate::header::CCCC;


/// Aerodome forecast report in AM 51 TAF format
#[derive(Clone, Debug,)]
pub struct TAFReport {
    pub items: Vec<TAFReportItem>,
}

/// A single TAF forecast
#[derive(Clone, Debug,)]
pub struct TAFReportItem {
    pub country: CCCC,
    pub origin_date: NaiveTime,
    pub time_range: Option<(NaiveTime, NaiveTime)>,
    pub wind: Option<TAFWind>,
    pub horizontal_vis: Length,
    pub significant_weather: SignificantWeather,
}

#[derive(Clone, Copy, Debug)]
pub struct TAFWind {
    pub direction: Angle,
    pub speed: Velocity,
    pub max_speed: Option<Velocity>,
}


/// Significant weather reported in FM 15 and 51
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SignificantWeather {
    pub intensity: SignificantWeatherIntensity,
    pub descriptor: Option<SignificantWeatherDescriptor>,
    pub precipitation: SignificantWeatherPrecipitation,
    pub phenomena: Option<SignificantWeatherPhenomena>,
}

#[derive(Clone,Copy,Debug, PartialEq, Eq)]
pub enum SignificantWeatherIntensity {
    Light,
    Moderate,
    Heavy,
    Vicinity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SignificantWeatherDescriptor {
    Shallow,
    Patches,
    Partial,
    LowDrifting,
    Blowing,
    Showers,
    Thunderstorm,
    Supercooled,
}

bitflags::bitflags! {
    pub struct SignificantWeatherPrecipitation: u8 {
        const DRIZZLE   = 0b00000001;
        const RAIN      = 0b00000010;
        const SNOW      = 0b00000100;
        const SNOWGRAIN = 0b00001000;
        const ICEPELLET = 0b00010000;
        const HAIL      = 0b00100000;
        const SMALLHAIL = 0b01000000;
        const UNKNOWN   = 0b10000000;
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SignificantWeatherPhenomena {
    Mist,
    Fog,
    Smoke,
    Ash,
    Dust,
    Sand,
    Haze,
    DustSandSwirls,
    Squalls,
    FunnelCloud,
    SandStorm,
    DustStorm,
}

impl SignificantWeather {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, intensity) = opt(
            alt((
                map_opt(
                    anychar,
                    |c: char| Some(match c {
                        '-' => SignificantWeatherIntensity::Light,
                        '+' => SignificantWeatherIntensity::Heavy,
                        _ => return None,
                    })
                ),
                map_opt(
                    take(2usize),
                    |s: &str| (s == "VC").then_some(SignificantWeatherIntensity::Moderate),
                )
            ))
        )(input)?;

        let intensity = intensity.unwrap_or(SignificantWeatherIntensity::Moderate);
        let (input, descriptor) = opt(map_opt(
            take(2usize),
            |s: &str| Some(match s {
                "MI" => SignificantWeatherDescriptor::Shallow,
                "BC" => SignificantWeatherDescriptor::Patches,
                "PR" => SignificantWeatherDescriptor::Partial,
                "DR" => SignificantWeatherDescriptor::LowDrifting,
                "BL" => SignificantWeatherDescriptor::Blowing,
                "SH" => SignificantWeatherDescriptor::Showers,
                "TS" => SignificantWeatherDescriptor::Thunderstorm,
                "FZ" => SignificantWeatherDescriptor::Supercooled,
                _ => return None,
            })
        ))(input)?;

        let (input, precipitation) = SignificantWeatherPrecipitation::parse(input)?;
        let (input, phenomena) = opt(map_opt(
            take(2usize),
            |s: &str| Some(match s {
                "BR" => SignificantWeatherPhenomena::Mist,
                "FG" => SignificantWeatherPhenomena::Fog,
                "FU" => SignificantWeatherPhenomena::Smoke,
                "VA" => SignificantWeatherPhenomena::Ash,
                "DU" => SignificantWeatherPhenomena::Dust,
                "SA" => SignificantWeatherPhenomena::Sand,
                "HZ" => SignificantWeatherPhenomena::Haze,
                "PO" => SignificantWeatherPhenomena::DustSandSwirls,
                "SQ" => SignificantWeatherPhenomena::Squalls,
                "FC" => SignificantWeatherPhenomena::FunnelCloud,
                "SS" => SignificantWeatherPhenomena::SandStorm,
                "DS" => SignificantWeatherPhenomena::DustStorm,
                _ => return None,
            })
        ))(input)?;

        Ok((
            input,
            Self {
                intensity,
                descriptor,
                precipitation,
                phenomena,
            }
        ))
    }
}

impl SignificantWeatherPrecipitation {
    pub fn parse(mut input: &str) -> IResult<&str, Self> {
        let mut me = Self::empty();
        while let (new_input, Some(prec)) = opt(
            map_res(
                take(2usize),
                |s: &str| Ok(match s {
                    "DZ" => Self::DRIZZLE,
                    "RA" => Self::RAIN,
                    "SN" => Self::SNOW,
                    "SG" => Self::SNOWGRAIN,
                    "PL" => Self::ICEPELLET,
                    "GR" => Self::HAIL,
                    "GS" => Self::SMALLHAIL,
                    "UP" => Self::UNKNOWN,
                    _ => return Err(()),
                })
            )
        )(input)? {
            input = new_input;
            me |= prec;
        }

        Ok((input, me))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_significant_weather() {
        let (_, sigwth) = SignificantWeather::parse("+SNRA").unwrap();
        let correct_weather = SignificantWeather {
            intensity: SignificantWeatherIntensity::Heavy,
            descriptor: None,
            precipitation: SignificantWeatherPrecipitation::RAIN | SignificantWeatherPrecipitation::SNOW,
            phenomena: None,
        };

        assert_eq!(sigwth, correct_weather);
    }
}
