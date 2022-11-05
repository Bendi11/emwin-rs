use chrono::NaiveTime;
use nom::{
    branch::alt,
    bytes::complete::{take, take_till, take_until},
    character::{
        complete::{anychar, multispace0, multispace1, space0, space1},
        streaming::char,
    },
    combinator::{map_opt, map_res, opt},
    error::context,
    multi::many_till,
    sequence::{preceded, separated_pair, terminated, tuple},
    Parser,
};
use nom_supreme::tag::complete::tag;
use uom::si::{
    f32::{Angle, Length, Velocity},
    velocity::{knot, meter_per_second},
};

use crate::{
    formats::{codes::visibility::vvvv, codetbl::parse_1690},
    header::{WMOProductIdentifier, CCCC},
    parse::{
        fromstr,
        recover::recover,
        time::{yygg, yygggg},
    },
    ParseResult,
};

use super::codes::{
    weather::SignificantWeather,
    wind::{ddd, ff},
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
                preceded(
                    multispace0,
                    recover(
                        TAFReportItem::parse,
                        terminated(take_until("="), char('=')).or(nom::combinator::rest),
                    ),
                ),
            )(input)
            {
                Ok((i, Some(Some(r)))) => (i, r),
                Ok((i, _)) => {
                    input = i;
                    continue;
                }
                Err(e) => {
                    log::error!(
                        "Failed to parse a TAF report item: {}",
                        crate::display_error(e)
                    );

                    break;
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
            preceded(space1, terminated(yygggg, char('Z'))),
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
            preceded(
                space0,
                alt((TAFWind::parse.map(Some), tag("CNL").map(|_| None))),
            ),
        )(input)?;

        let (input, (horizontal_vis, significant_weather, clouds)) =
            parse_vis_weather_clouds(input)?;

        let mut input = input;
        let mut groups = vec![];

        loop {
            let (new_input, group) = preceded(
                multispace0,
                alt((
                    recover(TAFReportItemGroup::parse, take_till(|c| c == '\n')),
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
            let (input, probability) = context("item group probability", fromstr(2))(input)?;

            context(
                "item following probability estimate",
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
                ),
            )(input)
        }

        let (input, kind) = context(
            "item group",
            alt((
                context(
                    "BECMG group",
                    preceded(
                        tuple((tag("BECMG"), space1)),
                        parse_from_to.map(|(from, to)| TAFReportItemGroupKind::Change(from, to)),
                    ),
                ),
                context(
                    "TEMPO group",
                    preceded(
                        tuple((tag("TEMPO"), space1)),
                        parse_from_to.map(|(from, to)| TAFReportItemGroupKind::TemporaryChange {
                            probability: 100f32,
                            from,
                            to,
                        }),
                    ),
                ),
                context(
                    "FM group",
                    preceded(tag("FM"), yygggg.map(TAFReportItemGroupKind::TimeIndicator)),
                ),
                context("PROB group", preceded(tag("PROB"), parse_prob)),
            )),
        )(input)?;

        let (input, wind) = opt(preceded(space0, TAFWind::parse))(input)?;
        let (input, (visibility, weather, clouds)) = parse_vis_weather_clouds(input)?;

        let (input, _) = opt(preceded(
            space0,
            preceded(tag("WS"), many_till(anychar, alt((tag("KT"), tag("MPS"))))),
        ))(input)?;

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

impl TAFWind {
    pub fn parse(input: &str) -> ParseResult<&str, Self> {
        let (input, direction) = ddd(input)?;
        let (input, speed) = ff(input)?;

        let (input, max_speed) = opt(preceded(char('G'), fromstr(2)))(input)?;

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
            Self {
                direction,
                speed,
                max_speed,
            },
        ))
    }
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
    let (input, cavok) = opt(preceded(space1, tag("CAVOK")))(input)?;

    Ok(match cavok.is_some() {
        true => (input, (None, None, vec![])),
        false => {
            let (input, visibility) = vvvv(input)?;

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

#[cfg(test)]
mod test {
    use crate::formats::codes::weather::{
        SignificantWeatherIntensity, SignificantWeatherPrecipitation,
    };

    use super::*;

    const TAF: &str = include_str!("./test/taf.txt");
    const ITEM: &str = r#"KIAD 052059Z 0521/0624 18015G24KT P6SM FEW050 BKN250
  FM052200 16010G18KT P6SM SCT050 BKN250
  FM060300 17008G16KT P6SM SCT030 BKN100
  FM060900 18006KT P6SM VCSH SCT015 BKN030 WS020/20030KT
  FM061400 18008G16KT P6SM VCSH SCT015 BKNERROR030
  FM062000 19010G17KT P6SM SCT025 BKN050="#;

    #[test]
    pub fn test_taf() {
        let (_, _) =
            TAFReportItem::parse(ITEM).unwrap_or_else(|e| panic!("{}", crate::display_error(e)));
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
