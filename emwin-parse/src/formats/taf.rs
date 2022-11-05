use std::num::ParseFloatError;

use chrono::{NaiveTime, Duration};
use nom::{combinator::{recognize, map_opt, opt, map_res}, bytes::complete::{take, tag, take_till}, character::{complete::{anychar, space1, digit1, multispace1}, streaming::char}, branch::alt, sequence::{preceded, terminated, separated_pair, tuple}, Parser, multi::{many0, separated_list0}, error::{FromExternalError, ErrorKind}};
use uom::si::{f32::{Angle, Velocity, Length}, angle::degree, velocity::{knot, meter_per_second}, length::{mile, meter}};

use crate::{header::{CCCC, WMOProductIdentifier}, util::{TIME_YYGGGG, parse_yygg}, formats::codetbl::parse_1690, ParseResult, ParseError};



/// Aerodome forecast report in AM 51 TAF format
#[derive(Clone, Debug,)]
pub struct TAFReport {
    pub header: WMOProductIdentifier,
    pub items: Vec<TAFReportItem>,
}

/// A single TAF forecast
#[derive(Clone, Debug,)]
pub struct TAFReportItem {
    pub kind: TAFReportKind,
    pub country: CCCC,
    /// Offset from the current month to the time this was reported
    pub origin_date: NaiveTime,
    pub time_range: (NaiveTime, NaiveTime),
    pub wind: Option<TAFWind>,
    pub horizontal_vis: Option<Length>,
    pub significant_weather: Option<SignificantWeather>,
    pub clouds: Vec<TAFSignificantWeatherReportClouds>,
    pub groups: Vec<TAFReportItemGroup>,
}

#[derive(Clone, Copy, Debug,)]
pub enum TAFReportKind {
    Report,
    Amendment,
    Correction,
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

#[derive(Clone, Debug,)]
pub struct TAFReportItemGroup {
    pub kind: TAFReportItemGroupKind,
    pub wind: TAFWind,
    pub visibility: Option<Length>,
    pub weather: Option<SignificantWeather>,
    pub clouds: Vec<TAFSignificantWeatherReportClouds>,
}

#[derive(Clone, Copy, Debug)]
pub enum TAFReportItemGroupKind {
    TimeIndicator(NaiveTime),
    Change(NaiveTime, NaiveTime),
    TemporaryChange {
        probability: f32,
        from: NaiveTime,
        to: NaiveTime,
    },
    Probable {
        probability: f32,
        from: NaiveTime,
        to: NaiveTime,
    }
}

impl TAFReport {
    pub fn parse(input: &str) -> ParseResult<&str, TAFReport> {
        let (input, header) = WMOProductIdentifier::parse(input)?;

        let mut input = input;
        let mut items = vec![];

        while !input.is_empty() {
            let (new_input, report) = match preceded(multispace1, TAFReportItem::parse)(input) {
                Ok((i, Some(r))) => (i, r),
                Ok((_, None)) => continue,
                Err(e) => {
                    log::warn!("Failed to parse a TAF report item: {}", e);
                    input = multispace1(input)?.0;
                    continue
                }
            };
            
            items.push(report);
            input = new_input;
        }

        Ok((
            input,
            Self {
                header,
                items,
            }
        ))
    }
}

impl TAFReportItem {
    /// Attempt to parse a `TAFReportItem`, returning `None` if the report is NIL
    pub fn parse(input: &str) -> ParseResult<&str, Option<Self>> {
        let (input, kind) = preceded(
            tag("TAF"),
            opt(preceded(multispace1, map_opt(
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

        let (input, Some(time_range)) = preceded(
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

        let (input, wind) = alt((parse_wind.map(Some), tag("CNL").map(|_| None)))(input)?;
        
        let (input, (horizontal_vis, significant_weather, clouds)) = parse_vis_weather_clouds(input)?;
        
        let (input, groups) = preceded(multispace1, separated_list0(tuple((tag("\n\n\n"), space1)), TAFReportItemGroup::parse))(input)?;

        Ok((input, Some(Self {
            country,
            kind,
            origin_date,
            horizontal_vis,
            significant_weather,
            clouds,
            time_range,
            wind,
            groups,
        })))
    }
}

impl TAFReportItemGroup {
    pub fn parse(input: &str) -> ParseResult<&str, Self> {
        fn parse_from_to(input: &str) -> ParseResult<&str, (NaiveTime, NaiveTime)> {
            separated_pair(parse_yygg, char('/'), parse_yygg)(input)
        }

        let (input, first) = take(2usize)(input)?;
        let (input, kind) = match first {
            "BE" => preceded(
                tuple((tag("CMG"), space1)),
                parse_from_to.map(|(from, to)| TAFReportItemGroupKind::Change(from, to)),
            )(input)?,
            "TE" => preceded(
                tuple((tag("MPO"), space1)),
                parse_from_to.map(|(from, to)| TAFReportItemGroupKind::TemporaryChange { probability: 100f32, from, to, })
            )(input)?,
            "FM" => map_res(
                take(6usize),
                |s: &str| Ok::<_, chrono::ParseError>(TAFReportItemGroupKind::TimeIndicator(NaiveTime::parse_from_str(s, TIME_YYGGGG)?)),
            )(input)?,
            "PR" => {
                let (input, probability) = preceded(
                    tag("OB"),
                    map_res(take(2usize), |s: &str| s.parse::<f32>())
                )(input)?;

                preceded(
                    space1,
                    alt((
                        preceded(tuple((tag("TEMPO"), space1)), parse_from_to).map(move |(from, to)| 
                            TAFReportItemGroupKind::TemporaryChange { probability, from, to, }
                        ),
                        parse_from_to.map(move |(from, to)| TAFReportItemGroupKind::Probable { probability, from, to, })
                    ))
                )(input)?
            },
            _ => return Err(nom::Err::Error(ParseError::<&str>::from_external_error(first, ErrorKind::Fail, "Invalid TAF report group"))),
        };

        let (input, wind) = parse_wind(input)?;
        let (input, (visibility, weather, clouds)) = parse_vis_weather_clouds(input)?;

        Ok((input, Self {
            kind,
            wind,
            visibility,
            weather,
            clouds,
        }))
    }
}

fn parse_wind(input: &str) -> ParseResult<&str, TAFWind> {
    let (input, direction) = preceded(
        space1,
        map_res(
            take(3usize),
            |s: &str| match s {
                "VRB" => Ok::<_, ParseFloatError>(Angle::new::<degree>(0f32)),
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

    Ok((input, TAFWind {
        direction,
        speed,
        max_speed,
    }))
}

fn parse_vis_weather_clouds(input: &str)
    -> ParseResult<&str, (Option<Length>, Option<SignificantWeather>, Vec<TAFSignificantWeatherReportClouds>)> {
        let mut vis_sm = terminated(
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

        Ok(match cavok.is_some() {
            true => (input, (None, None, vec![])),
            false => {
                let (input, vis_first) = preceded(
                    space1,
                    alt((
                        map_res(
                            digit1,
                            |s: &str| Ok::<VisFirst, ParseFloatError>(VisFirst::Number(s.parse::<f32>()?))
                        ),
                        map_res(
                            &mut vis_sm,
                            |(first, denominator)| Ok::<VisFirst, ParseFloatError>(match denominator {
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
                                    Err("weather string is not NSW")
                                }
                            )
                        )),
                    )
                )(input)?;

                let weather = weather.flatten();

                fn parse_clouds(input: &str) -> ParseResult<&str, Option<TAFSignificantWeatherReportClouds>> {
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
                                    _ => return Err("invalid cloud amount code"),
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

                let (input, clouds) = many0(map_res(
                    parse_clouds,
                    |c| c.ok_or("failed to parse cloud description")
                    ))(input)?;

                (input, (Some(visibility), weather, clouds))
            }
        })
}

impl SignificantWeather {
    pub fn parse(input: &str) -> ParseResult<&str, Self> {
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
    pub fn parse(mut input: &str) -> ParseResult<&str, Self> {
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
                    _ => return Err("invalid precipitation code"),
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

    const TAF: &str = include_str!("./test/taf.txt");

    #[test]
    pub fn test_taf() {
        let (_, taf) = TAFReport::parse(TAF).unwrap_or_else(|e| match e {
            nom::Err::Error(e) | nom::Err::Failure(e) => panic!("{}", e),
            e => panic!("{}", e),
        });
    }

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
