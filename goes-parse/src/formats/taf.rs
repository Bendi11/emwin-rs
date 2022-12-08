use chrono::NaiveDate;
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
use uom::si::f32::Length;

use crate::{
    formats::codes::visibility::vvvv,
    header::{WMOProductIdentifier, CCCC},
    parse::{
        fromstr_n, multi_opt,
        recover::recover,
        time::{yygg, yygggg, DayHourMinute},
    },
    ParseResult,
};

use super::codes::{clouds::CloudReport, weather::SignificantWeather, wind::WindSummary};

/// Aerodome forecast report in AM 51 TAF format
#[derive(Clone, Debug)]
pub struct TAFReport {
    pub header: WMOProductIdentifier,
    pub month: NaiveDate,
    pub items: Vec<TAFReportItem>,
}

/// A single TAF forecast
#[derive(Clone, Debug)]
pub struct TAFReportItem {
    pub kind: TAFReportKind,
    pub country: CCCC,
    /// Offset from the current month to the time this was reported
    pub origin_date: DayHourMinute,
    pub time_range: (DayHourMinute, DayHourMinute),
    pub wind: Option<WindSummary>,
    pub horizontal_vis: Option<Length>,
    pub significant_weather: Vec<SignificantWeather>,
    pub clouds: Vec<CloudReport>,
    pub groups: Vec<TAFReportItemGroup>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub enum TAFReportKind {
    Report,
    Amendment,
    Correction,
}

#[derive(Clone, Debug)]
pub struct TAFReportItemGroup {
    pub kind: TAFReportItemGroupKind,
    pub wind: Option<WindSummary>,
    pub visibility: Option<Length>,
    pub weather: Vec<SignificantWeather>,
    pub clouds: Vec<CloudReport>,
}

#[derive(Clone, Copy, Debug)]
pub enum TAFReportItemGroupKind {
    TimeIndicator(DayHourMinute),
    Change(DayHourMinute, DayHourMinute),
    TemporaryChange {
        probability: f32,
        from: DayHourMinute,
        to: DayHourMinute,
    },
    Probable {
        probability: f32,
        from: DayHourMinute,
        to: DayHourMinute,
    },
}

impl TAFReport {
    pub fn parse<'a>(month: NaiveDate) -> impl FnMut(&'a str) -> ParseResult<&'a str, Self> {
        move |input| Self::parse_full(input, month)
    }

    pub fn parse_full(input: &str, month: NaiveDate) -> ParseResult<&str, TAFReport> {
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

        Ok((
            input,
            Self {
                header,
                month,
                items,
            },
        ))
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
                alt((WindSummary::parse.map(Some), tag("CNL").map(|_| None))),
            ),
        )(input)?;

        let (input, (horizontal_vis, significant_weather, clouds)) =
            parse_vis_weather_clouds(input)?;

        let mut input = input;
        let mut groups = vec![];

        loop {
            let (new_input, end) = opt(preceded(multispace0, char('=')))(input)?;
            if end.is_some() {
                break;
            }

            input = new_input;

            let (new_input, group) = preceded(
                multispace0,
                recover(
                    TAFReportItemGroup::parse,
                    alt((take_till(|c| c == '\n' || c == '='), nom::combinator::rest)),
                ),
            )(input)?;

            input = new_input;
            match group {
                Some(group) => groups.push(group),
                None => continue,
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

impl TAFReportItemGroupKind {
    /// Get the offset from the month that this group is reporting from
    pub const fn from(&self) -> DayHourMinute {
        match self {
            Self::TimeIndicator(from)
            | Self::Change(from, _)
            | Self::TemporaryChange { from, .. }
            | Self::Probable { from, .. } => *from,
        }
    }

    /// Get the offset from the month that this report is expected to last until
    pub const fn to(&self) -> Option<DayHourMinute> {
        match self {
            Self::Probable { to, .. } | Self::Change(_, to) | Self::TemporaryChange { to, .. } => {
                Some(*to)
            }
            _ => None,
        }
    }

    /// Get the probability of this event occuring
    pub const fn probability(&self) -> Option<f32> {
        match self {
            Self::Probable { probability, .. } | Self::TemporaryChange { probability, .. } => {
                Some(*probability)
            }
            _ => None,
        }
    }
}

impl TAFReportItemGroup {
    pub fn parse(input: &str) -> ParseResult<&str, Self> {
        fn parse_from_to(input: &str) -> ParseResult<&str, (DayHourMinute, DayHourMinute)> {
            separated_pair(yygg, char('/'), yygg)(input)
        }

        fn parse_prob(input: &str) -> ParseResult<&str, TAFReportItemGroupKind> {
            let (input, probability) = context("item group probability", fromstr_n(2))(input)?;

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

        let (input, wind) = preceded(
            space0,
            recover(WindSummary::parse, take_till(|c: char| c.is_whitespace())),
        )(input)?;
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

fn parse_vis_weather_clouds(
    input: &str,
) -> ParseResult<&str, (Option<Length>, Vec<SignificantWeather>, Vec<CloudReport>)> {
    let (input, cavok) = opt(preceded(space1, tag("CAVOK")))(input)?;

    Ok(match cavok.is_some() {
        true => (input, (None, vec![], vec![])),
        false => {
            let (input, visibility) = opt(vvvv)(input)?;

            let (input, weather) = multi_opt(preceded(
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
            ))
            .parse(input)?;

            let mut input = input;
            let mut clouds = vec![];
            loop {
                let (new_input, cloud) = preceded(space0, opt(CloudReport::parse))(input)?;
                input = new_input;
                match cloud {
                    Some(Some(c)) => clouds.push(c),
                    _ => break,
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
  FM061400 18008G16KT P6SM VCSH SCT015 BKNERROR030 SCT070
  FM062000 1901LALALAALALAL0G17KT P6SM SCT025 BKN050="#;

    #[test]
    pub fn test_taf() {
        let (_, item) =
            TAFReportItem::parse(ITEM).unwrap_or_else(|e| panic!("{}", crate::display_error(e)));
        assert_eq!(item.unwrap().groups.len(), 5);
        let (_, _) = TAFReport::parse(chrono::Local::now().naive_utc().date())
            .parse(TAF)
            .unwrap_or_else(|e| match e {
                nom::Err::Error(e) | nom::Err::Failure(e) => panic!(
                    "{}",
                    e.map_locations(|s| &s[0..s.find('\n').unwrap_or(s.len())])
                ),
                e => panic!("{}", e),
            });
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
