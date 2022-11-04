use chrono::{NaiveTime, Duration};
use nom::{IResult, combinator::{recognize, map_opt, opt, map_res}, bytes::complete::{take, tag, take_till}, character::{complete::{anychar, space1, digit1}, streaming::char}, branch::alt, sequence::{preceded, terminated, separated_pair, tuple}, Parser, multi::many0};
use uom::si::{f32::{Angle, Velocity, Length}, angle::degree, velocity::{knot, meter_per_second}, length::{mile, meter}};

use crate::{header::CCCC, util::{TIME_YYGGGG, parse_yygg}, formats::codetbl::parse_1690};



/// Aerodome forecast report in AM 51 TAF format
#[derive(Clone, Debug,)]
pub struct TAFReport {
    pub items: Vec<TAFReportItem>,
}

/// A single TAF forecast
#[derive(Clone, Debug,)]
pub struct TAFReportItem {
    pub kind: TAFReportKind,
    pub country: CCCC,
    /// Offset from the current month to the time this was reported
    pub origin_date: NaiveTime,
    pub time_range: Option<(NaiveTime, NaiveTime)>,
    pub wind: Option<TAFWind>,
    pub significant_weather: Option<TAFSignificantWeatherReport>,
    pub visibility: Length,
}

#[derive(Clone, Copy, Debug,)]
pub enum TAFReportKind {
    Report,
    Amendment,
    Correction,
}

#[derive(Clone, Debug,)]
pub struct TAFSignificantWeatherReport {
    pub horizontal_vis: Length,
    pub significant_weather: Option<SignificantWeather>,
    pub clouds: Vec<TAFSignificantWeatherReportClouds>,
}

#[derive(Clone, Copy, Debug,)]
pub struct TAFSignificantWeatherReportClouds {
    pub amount: Option<CloudAmount>,
    pub altitude: Length,
}

#[derive(Clone, Copy, Debug)]
pub struct TAFWind {
    pub direction: Angle,
    pub speed: Velocity,
    pub max_speed: Option<Velocity>,
}

/// Cloud amount NsNsNs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CloudAmount {
    Few,
    Scattered,
    Broken,
    Overcast,
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

impl TAFReportItem {
    /// Attempt to parse a `TAFReportItem`, returning `None` if the report is NIL
    pub fn parse(input: &str) -> IResult<&str, Option<Self>> {
        let (input, kind) = preceded(
            tag("TAF"),
            opt(preceded(space1, map_opt(
                take(3usize),
                |s: &str| Some(match s {
                    "AMD" => TAFReportKind::Amendment,
                    "COR" => TAFReportKind::Correction,
                    _ => return None,
                })
            )))
        )(input)?;


        let kind = kind.unwrap_or(TAFReportKind::Report);

        let (input, country) = preceded(space1, map_res(
            take(4usize),
            |s: &str| s.parse::<CCCC>()
        ))(input)?;


        let (input, origin_date) = preceded(space1, 
            terminated(
                map_res(
                    take(6usize),
                    |s: &str| NaiveTime::parse_from_str(s, TIME_YYGGGG),
                ),
                char('Z'),
            )
        )(input)?;

        let (input, Some(from_to)) = preceded(
            space1,
            alt((
                tag("NIL").map(|_| None),
                separated_pair(
                    parse_yygg,
                    char('/'),
                    parse_yygg
                ).map(|v| Some(v))
            ))
        )(input)? else {
            let (input, _) = char('=')(input)?;
            return Ok((input, None))
        };

        let (input, direction) = preceded(
            space1,
            map_res(
                take(3usize),
                |s: &str| match s {
                    "VRB" => Ok(Angle::new::<degree>(0f32)),
                    _ => Ok(Angle::new::<degree>(s.parse::<f32>()?))
                },
            ),
        )(input)?;

        let (input, speed) = map_res(
            take(2usize),
            |s: &str| s.parse::<f32>(),
        )(input)?;

        let (input, max_speed) = opt(
            preceded(
                char('G'),
                map_res(
                    take(2usize),
                    |s: &str| s.parse::<f32>(),
                ),
            ),
        )(input)?;

        let (input, (speed, max_speed)) = map_res(
            take(3usize),
            |s: &str| match s {
                "KTS" => Ok((Velocity::new::<knot>(speed), max_speed.map(Velocity::new::<knot>))),
                "MPS" => Ok((Velocity::new::<meter_per_second>(speed), max_speed.map(Velocity::new::<meter_per_second>))),
                _ => return Err("Unknown unit of speed for wind speed")
            }
        )(input)?;

        let wind = TAFWind {
            direction,
            speed,
            max_speed,
        };

        let vis_sm = terminated(
            tuple((
                map_res(
                    digit1,
                    |s: &str| s.parse::<f32>(),
                ),
                opt(preceded(
                    char('/'),
                    map_res(
                        digit1,
                        |s: &str| s.parse::<f32>(),
                    ),
                )),
            )),
            tag("SM"),
        );
        
        enum VisFirst { Number(f32), SM(f32) }

        let (input, cavok) = opt(preceded(space1, tag("CAVOK")))(input)?;

        let (input, vis_first) = preceded(
            space1,
            alt((
                map_res(
                    digit1,
                    |s: &str| Ok(VisFirst::Number(s.parse::<f32>()?))
                ),
                map_res(
                    vis_sm,
                    |(first, denominator)| Ok(match denominator {
                        Some(d) => VisFirst::SM(first / d),
                        None => VisFirst::SM(first),
                    })
                ),
            )),
        )(input)?;

        let (input, visibility) = match vis_first {
            VisFirst::Number(whole) => match opt(vis_sm)(input)? {
                (input, Some((numerator, Some(denominator)))) => (input, Length::new::<mile>(whole + numerator / denominator)),
                (input, Some((numerator, None))) => (input, Length::new::<mile>(whole + numerator)),
                (input, None) => (input, Length::new::<meter>(whole)),
            },
            VisFirst::SM(vis) => (input, Length::new::<mile>(vis)),
        };

        let (input, weather) = opt(
            preceded(
                space1,
                alt((
                    SignificantWeather::parse.map(|w| Some(w)),
                    map_res(
                        take(3usize),
                        |s: &str| if s == "NSW" {
                            Ok(None) 
                        } else {
                            Err(())
                        }
                    )
                )),
            )
        )(input)?;

        let weather = weather.flatten();

        fn parse_clouds(input: &str) -> IResult<&str, Option<TAFSignificantWeatherReportClouds>> {
            let (input, amount) = opt(preceded(
                space1,
                alt((
                    tag("VV").map(|_| None),
                    map_res(
                        take(3usize),
                        |s: &str| Ok(match s {
                            "FEW" => Some(CloudAmount::Few),
                            "SCT" => Some(CloudAmount::Scattered),
                            "BKN" => Some(CloudAmount::Broken),
                            "OVC" => Some(CloudAmount::Overcast),
                            _ => return Err(()),
                        })
                    )
                ))
            ))(input)?;

            Ok(match amount {
                Some(amount) => {
                    let (input, altitude) = parse_1690(input)?;
                    (
                        input,
                        Some(TAFSignificantWeatherReportClouds {
                            amount,
                            altitude,
                        }),
                    )
                },
                None => (input, None),
            })
        }

        let (input, clouds) = many0(map_res(parse_clouds, |c| c.ok_or(())))(input)?;
    }
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
