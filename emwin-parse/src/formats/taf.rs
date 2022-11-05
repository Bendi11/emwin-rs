use std::num::ParseFloatError;

use chrono::NaiveTime;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::{
        complete::{anychar, digit1, multispace0, multispace1, space0, space1},
        streaming::char,
    },
    combinator::{map_opt, map_res, opt},
    error::context,
    sequence::{preceded, separated_pair, terminated, tuple},
    Parser,
};
use uom::si::{
    angle::degree,
    f32::{Angle, Length, Velocity},
    length::{meter, mile},
    velocity::{knot, meter_per_second},
};

use crate::{
    formats::codetbl::parse_1690,
    header::{WMOProductIdentifier, CCCC},
    parse::time::{yygg, yygggg}, ParseResult,
};

/// Aerodome forecast report in AM 51 TAF format
#[derive(Clone, Debug)]
pub struct TAFReport {
    pub header: WMOProductIdentifier,
    pub items: Vec<TAFReportItem>,
}

/// A single TAF forecast
#[derive(Clone, Debug)]
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

#[derive(Clone, Copy, Debug)]
pub enum TAFReportKind {
    Report,
    Amendment,
    Correction,
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug)]
pub struct TAFReportItemGroup {
    pub kind: TAFReportItemGroupKind,
    pub wind: Option<TAFWind>,
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
    },
}

impl TAFReport {
    pub fn parse(input: &str) -> ParseResult<&str, TAFReport> {
        let (input, header) = WMOProductIdentifier::parse(input)?;

        let mut input = input;
        let mut items = vec![];

        while !input.is_empty() {
            let (new_input, report) = match context(
                "TAF report item",
                preceded(multispace0, TAFReportItem::parse),
            )(input)
            {
                Ok((i, Some(r))) => (i, r),
                Ok((_, None)) => continue,
                Err(e) => {
                    eprintln!(
                        "Failed to parse a TAF report item: {}",
                        crate::display_error(e)
                    );
                    input = multispace1(input)?.0;
                    continue;
                }
            };

            items.push(report);
            input = new_input;
        }

        Ok((input, Self { header, items }))
    }
}

impl TAFReportItem {
    /// Attempt to parse a `TAFReportItem`, returning `None` if the report is NIL
    pub fn parse(input: &str) -> ParseResult<&str, Option<Self>> {
        let (input, kind) = context(
            "TAF item header",
            opt(preceded(
                tag("TAF"),
                opt(preceded(
                    multispace1,
                    map_opt(take(3usize), |s: &str| {
                        Some(match s {
                            "AMD" => TAFReportKind::Amendment,
                            "COR" => TAFReportKind::Correction,
                            _ => return None,
                        })
                    }),
                )),
            )),
        )(input)?;

        let kind = kind.flatten().unwrap_or(TAFReportKind::Report);

        let (input, country) = context(
            "country code",
            preceded(
                multispace0,
                map_res(take(4usize), |s: &str| s.parse::<CCCC>()),
            ),
        )(input)?;

        let (input, origin_date) = context(
            "time of report origin",
            preceded(
                space1,
                terminated(
                    yygggg,
                    char('Z'),
                ),
            ),
        )(input)?;

        let (input, Some(time_range)) = context("from / to report date", preceded(
            space1,
            alt((
                tag("NIL").map(|_| None),
                separated_pair(
                    yygg,
                    char('/'),
                    yygg
                ).map(|v| Some(v))
            ))
        ))(input)? else {
            let (input, _) = char('=')(input)?;
            return Ok((input, None))
        };

        let (input, wind) = context(
            "wind levels",
            alt((parse_wind.map(Some), tag("CNL").map(|_| None))),
        )(input)?;

        let (input, (horizontal_vis, significant_weather, clouds)) =
            parse_vis_weather_clouds(input)?;

        let mut input = input;
        let mut groups = vec![];

        loop {
            let (new_input, group) = preceded(
                multispace0,
                alt((
                    TAFReportItemGroup::parse.map(|v| Some(v)),
                    char('=').map(|_| None),
                )),
            )(input)?;

            input = new_input;
            match group {
                Some(group) => groups.push(group),
                None => break,
            }
        }

        Ok((
            input,
            Some(Self {
                country,
                kind,
                origin_date,
                horizontal_vis,
                significant_weather,
                clouds,
                time_range,
                wind,
                groups,
            }),
        ))
    }
}

impl TAFReportItemGroup {
    pub fn parse(input: &str) -> ParseResult<&str, Self> {
        fn parse_from_to(input: &str) -> ParseResult<&str, (NaiveTime, NaiveTime)> {
            separated_pair(yygg, char('/'), yygg)(input)
        }

        fn parse_prob(input: &str) -> ParseResult<&str, TAFReportItemGroupKind> {
            let (input, probability) = map_res(take(2usize), |s: &str| s.parse::<f32>())(input)?;

            preceded(
                space1,
                alt((
                    preceded(tuple((tag("TEMPO"), space1)), parse_from_to).map(
                        move |(from, to)| TAFReportItemGroupKind::TemporaryChange {
                            probability,
                            from,
                            to,
                        },
                    ),
                    parse_from_to.map(move |(from, to)| TAFReportItemGroupKind::Probable {
                        probability,
                        from,
                        to,
                    }),
                )),
            )(input)
        }

        let (input, kind) = context(
            "item group",
            alt((
                preceded(
                    tuple((tag("BECMG"), space1)),
                    parse_from_to.map(|(from, to)| TAFReportItemGroupKind::Change(from, to)),
                ),
                preceded(
                    tuple((tag("TEMPO"), space1)),
                    parse_from_to.map(|(from, to)| TAFReportItemGroupKind::TemporaryChange {
                        probability: 100f32,
                        from,
                        to,
                    }),
                ),
                preceded(
                    tag("FM"),
                    yygggg.map(TAFReportItemGroupKind::TimeIndicator),
                ),
                preceded(tag("PROB"), parse_prob),
            )),
        )(input)?;

        let (input, wind) = opt(parse_wind)(input)?;
        let (input, (visibility, weather, clouds)) = parse_vis_weather_clouds(input)?;

        Ok((
            input,
            Self {
                kind,
                wind,
                visibility,
                weather,
                clouds,
            },
        ))
    }
}

fn parse_wind(input: &str) -> ParseResult<&str, TAFWind> {
    let (input, direction) = context(
        "wind direction",
        preceded(
            space1,
            map_res(take(3usize), |s: &str| match s {
                "VRB" => Ok::<_, ParseFloatError>(Angle::new::<degree>(0f32)),
                _ => Ok(Angle::new::<degree>(s.parse::<f32>()?)),
            }),
        ),
    )(input)?;

    let (input, speed) = context(
        "wind speed",
        map_res(take(2usize), |s: &str| s.parse::<f32>()),
    )(input)?;

    let (input, max_speed) = opt(preceded(
        char('G'),
        map_res(take(2usize), |s: &str| s.parse::<f32>()),
    ))(input)?;

    let (input, (speed, max_speed)) = context(
        "wind speed units",
        alt((
            map_res(tag("KT"), |_| {
                Ok::<_, &str>((
                    Velocity::new::<knot>(speed),
                    max_speed.map(Velocity::new::<knot>),
                ))
            }),
            map_res(tag("MPS"), |_| {
                Ok::<_, &str>((
                    Velocity::new::<meter_per_second>(speed),
                    max_speed.map(Velocity::new::<meter_per_second>),
                ))
            }),
        )),
    )(input)?;

    Ok((
        input,
        TAFWind {
            direction,
            speed,
            max_speed,
        },
    ))
}

fn parse_vis_weather_clouds(
    input: &str,
) -> ParseResult<
    &str,
    (
        Option<Length>,
        Option<SignificantWeather>,
        Vec<TAFSignificantWeatherReportClouds>,
    ),
> {
    let mut vis_sm = context(
        "horizontal visibility",
        terminated(
            tuple((
                map_res(digit1, |s: &str| s.parse::<f32>()),
                opt(preceded(
                    char('/'),
                    map_res(digit1, |s: &str| s.parse::<f32>()),
                )),
            )),
            tag("SM"),
        ),
    );

    enum VisFirst {
        Number(f32),
        SM(f32),
    }

    let (input, cavok) = opt(preceded(space1, tag("CAVOK")))(input)?;

    Ok(match cavok.is_some() {
        true => (input, (None, None, vec![])),
        false => {
            let (input, vis_first) = opt(context(
                "cloud visibility",
                preceded(
                    space1,
                    alt((
                        map_res(&mut vis_sm, |(first, denominator)| {
                            Ok::<VisFirst, ParseFloatError>(match denominator {
                                Some(d) => VisFirst::SM(first / d),
                                None => VisFirst::SM(first),
                            })
                        }),
                        map_res(digit1, |s: &str| {
                            Ok::<VisFirst, ParseFloatError>(VisFirst::Number(s.parse::<f32>()?))
                        }),
                        map_res(tag("P6SM"), |_| {
                            Ok::<_, ParseFloatError>(VisFirst::SM(6f32))
                        }),
                    )),
                ),
            ))(input)?;

            let (input, visibility) = match vis_first {
                Some(vis_first) => match vis_first {
                    VisFirst::Number(whole) => match opt(vis_sm)(input)? {
                        (input, Some((numerator, Some(denominator)))) => (
                            input,
                            Some(Length::new::<mile>(whole + numerator / denominator)),
                        ),
                        (input, Some((numerator, None))) => {
                            (input, Some(Length::new::<mile>(whole + numerator)))
                        }
                        (input, None) => (input, Some(Length::new::<meter>(whole))),
                    },
                    VisFirst::SM(vis) => (input, Some(Length::new::<mile>(vis))),
                },
                None => (input, None),
            };

            let (input, weather) = opt(preceded(
                space1,
                alt((
                    SignificantWeather::parse.map(|w| Some(w)),
                    map_res(take(3usize), |s: &str| {
                        if s == "NSW" {
                            Ok(None)
                        } else {
                            Err("weather string is not NSW")
                        }
                    }),
                )),
            ))(input)?;

            let weather = weather.flatten();

            fn parse_clouds(
                input: &str,
            ) -> ParseResult<&str, Option<TAFSignificantWeatherReportClouds>> {
                let (input, val) = opt(preceded(space0, alt((tag("NSC"), tag("SKC")))))(input)?;

                if val.is_some() {
                    return Ok((input, None));
                }

                let (input, amount) = opt(preceded(
                    space0,
                    alt((
                        tag("VV").map(|_| None),
                        map_res(take(3usize), |s: &str| {
                            Ok(match s {
                                "FEW" => Some(CloudAmount::Few),
                                "SCT" => Some(CloudAmount::Scattered),
                                "BKN" => Some(CloudAmount::Broken),
                                "OVC" => Some(CloudAmount::Overcast),
                                _ => return Err("invalid cloud amount code"),
                            })
                        }),
                    )),
                ))(input)?;

                Ok(match amount {
                    Some(amount) => {
                        let (input, altitude) = parse_1690(input)?;
                        (
                            input,
                            Some(TAFSignificantWeatherReportClouds { amount, altitude }),
                        )
                    }
                    None => (input, None),
                })
            }

            let mut input = input;
            let mut clouds = vec![];
            loop {
                let (new_input, cloud) = parse_clouds(input)?;
                input = new_input;
                match cloud {
                    Some(c) => clouds.push(c),
                    None => break,
                }
            }

            (input, (visibility, weather, clouds))
        }
    })
}

impl SignificantWeather {
    pub fn parse(input: &str) -> ParseResult<&str, Self> {
        let (input, intensity) = opt(alt((
            map_opt(anychar, |c: char| {
                Some(match c {
                    '-' => SignificantWeatherIntensity::Light,
                    '+' => SignificantWeatherIntensity::Heavy,
                    _ => return None,
                })
            }),
            map_opt(take(2usize), |s: &str| {
                (s == "VC").then_some(SignificantWeatherIntensity::Moderate)
            }),
        )))(input)?;

        let intensity = intensity.unwrap_or(SignificantWeatherIntensity::Moderate);
        let (input, descriptor) = opt(map_opt(take(2usize), |s: &str| {
            Some(match s {
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
        }))(input)?;

        let (input, precipitation) = SignificantWeatherPrecipitation::parse(input)?;
        let (input, phenomena) = opt(map_opt(take(2usize), |s: &str| {
            Some(match s {
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
        }))(input)?;

        Ok((
            input,
            Self {
                intensity,
                descriptor,
                precipitation,
                phenomena,
            },
        ))
    }
}

impl SignificantWeatherPrecipitation {
    pub fn parse(mut input: &str) -> ParseResult<&str, Self> {
        let mut me = Self::empty();
        while let (new_input, Some(prec)) = opt(map_res(take(2usize), |s: &str| {
            Ok(match s {
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
        }))(input)?
        {
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
    const ITEM: &str = r#"TEMPO 2200/2204 VRB06KT BKN025"#;

    #[test]
    pub fn test_taf() {
        let (_, item) = TAFReportItemGroup::parse(ITEM)
            .unwrap_or_else(|e| panic!("{}", crate::display_error(e)));
        let (_, taf) = TAFReport::parse(TAF).unwrap_or_else(|e| match e {
            nom::Err::Error(e) | nom::Err::Failure(e) => panic!(
                "{}",
                e.map_locations(|s| &s[0..s.find('\n').unwrap_or(s.len())])
            ),
            e => panic!("{}", e),
        });

        panic!("{:#?}", taf);
    }

    #[test]
    pub fn test_significant_weather() {
        let (_, sigwth) = SignificantWeather::parse("+SNRA").unwrap();
        let correct_weather = SignificantWeather {
            intensity: SignificantWeatherIntensity::Heavy,
            descriptor: None,
            precipitation: SignificantWeatherPrecipitation::RAIN
                | SignificantWeatherPrecipitation::SNOW,
            phenomena: None,
        };

        assert_eq!(sigwth, correct_weather);
    }
}
