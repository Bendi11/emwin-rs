use chrono::NaiveTime;
use nom::{IResult, combinator::{recognize, map_opt, opt, map_res}, bytes::complete::take};
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
#[derive(Clone, Copy, Debug,)]
pub struct SignificantWeather {
    pub intensity: SignificantWeatherIntensity,
    pub descriptor: Option<SignificantWeatherDescriptor>,
    pub precipitation: SignificantWeatherPrecipitation,
    pub phenomena: Option<SignificantWeatherPhenomena>,
}

#[derive(Clone,Copy,Debug)]
pub enum SignificantWeatherIntensity {
    Light,
    Heavy,
    Vicinity,
}

#[derive(Clone, Copy, Debug)]
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


#[derive(Clone, Copy, Debug)]
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

impl SignificantWeatherPrecipitation {
    pub fn parse(input: &str) -> IResult<&str, Self> {
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
