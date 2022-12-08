use chrono::NaiveDate;
use nom::{
    branch::alt,
    character::{
        complete::{anychar, multispace1, space0, space1, multispace0},
        streaming::char,
    },
    combinator::{map_res, opt},
    sequence::{preceded, separated_pair, terminated, tuple, delimited},
    Parser, error::context, bytes::complete::take_till,
};
use nom_supreme::tag::complete::tag;
use uom::si::{
    f32::{Angle, Length, Pressure, ThermodynamicTemperature},
    length::{decimeter, meter},
    pressure::{hectopascal, inch_of_mercury},
    angle::degree,
};

use crate::{
    header::{WMOProductIdentifier, CCCC},
    parse::{fromstr_n, multi, multi_opt, time::{yygggg, DayHourMinute}},
    ParseResult,
};

use super::{
    codes::{
        clouds::CloudReport,
        runway::{
            RunwayContaminationLevel, RunwayDepositDepth, RunwayDeposits, RunwaySurfaceFriction,
        },
        sea::StateOfTheSea,
        temperature,
        visibility::vvvv,
        weather::SignificantWeather,
        wind::WindSummary,
    },
    Compass, RunwayDesignator,
};

/// A METAR report parsed from EMWIN files, with additional header line
#[derive(Clone, Debug)]
pub struct EmwinMetarReport {
    pub header: WMOProductIdentifier,
    pub month: NaiveDate,
    pub metars: Vec<MetarReport>,
}

/// A single METAR weather report parsed from a FM 15/16 report
#[derive(Clone, Debug)]
pub struct MetarReport {
    pub country: CCCC,
    pub origin: DayHourMinute,
    pub wind: Option<WindSummary>,
    pub kind: MetarReportKind,
    pub variable_wind_dir: Option<MetarVariableWindDir>,
    pub visibility: Option<Length>,
    pub minimum_visibility: Option<MetarMinimumVisibility>,
    pub runway_range: Vec<(RunwayDesignator, Length, RunwayTrend)>,
    pub weather: Vec<SignificantWeather>,
    pub clouds: Vec<CloudReport>,
    pub air_dewpoint_temperature: Option<(ThermodynamicTemperature, ThermodynamicTemperature)>,
    pub qnh: Option<Pressure>,
    pub recent_weather: Option<SignificantWeather>,
    pub runway_wind_shear: Option<RunwayWindShear>,
    pub sea: Vec<MetarSeaSurfaceReport>,
    pub runway_status: Vec<RunwayState>,
}

#[derive(Clone, Copy, Debug)]
pub enum MetarSeaSurfaceReport {
    StateOfSea {
        temp: ThermodynamicTemperature,
        state: StateOfTheSea,
    },
    WaveHeight {
        temp: ThermodynamicTemperature,
        height: Length,
    },
}

/// Wind shear report with segment of runway affected
#[derive(Clone, Copy, Debug)]
pub enum RunwayWindShear {
    Within(Length),
    All,
}

/// The trend that a runway's distance is observed to be
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunwayTrend {
    Closer,
    Farther,
    NoChange,
}

/// Directions that variables winds blow between in a METAR report
#[derive(Clone, Copy, Debug)]
pub struct MetarVariableWindDir {
    pub extreme_ccw: Angle,
    pub extreme_cw: Angle,
}

/// Optional METAR report specifying the direction and length of minimum horizontal visibility
#[derive(Clone, Copy, Debug)]
pub struct MetarMinimumVisibility {
    pub visibility: Length,
    pub direction: Compass,
}

/// The kind of report a METAR/SPECI is
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MetarReportKind {
    Auto,
    Cor,
}

/// Reported runway contamination status
#[derive(Clone, Copy, Debug)]
pub struct RunwayState {
    pub runway: RunwayDesignator,
    pub deposits: RunwayDeposits,
    pub level: RunwayContaminationLevel,
    pub depth: RunwayDepositDepth,
    pub friction: RunwaySurfaceFriction,
}

impl EmwinMetarReport {
    pub fn parse(month: NaiveDate) -> impl FnMut(&str) -> ParseResult<&str, Self> {
        move |input: &str| {
            let (input, header) = WMOProductIdentifier::parse(input)?;
            let (input, _) = preceded(multispace1, alt((tag("METAR"), tag("SPECI"))))(input)?;
            let mut input = input;
            let mut metars = vec![];
            while !input.is_empty() {
                let (new_input, metar) = preceded(
                    multispace0,
                    MetarReport::parse,
                )(input)?;

                if let Some(metar) = metar {
                    metars.push(metar);
                }
                
                let (new_input, _) = delimited(
                    multispace0,
                    char('='),
                    multispace0,
                )(new_input)?;

                input = new_input;
            }

            Ok((
                input,
                Self {
                    header,
                    month,
                    metars,
                },
            ))
        }
    }
}

impl MetarReport {
    /// Returns Ok(None) if the report is `NIL`
    pub fn parse(input: &str) -> ParseResult<&str, Option<Self>> {
        let (input, kind) = opt(preceded(space0, tag("COR").map(|_| MetarReportKind::Cor)))(input)?;

        let (input, country): (_, CCCC) = context("Four-letter country code", preceded(space0, fromstr_n(4)))(input)?;
        let (input, origin) = context("METAR origin timestamp", preceded(space0, terminated(yygggg, char('Z'))))(input)?;

        let (input, kind) = match kind {
            Some(kind) => (input, kind),
            None => match opt(preceded(
                space1,
                alt((
                    tag("NIL").map(|_| None),
                    tag("AUTO").map(|_| Some(MetarReportKind::Auto)),
                    tag("COR").map(|_| Some(MetarReportKind::Cor)),
                )),
            ))(input)?
            {
                (input, Some(Some(k))) => (input, k),
                (input, None) => (input, MetarReportKind::Auto),
                (input, Some(None)) => return Ok((input, None)),
            },
        };

        let (input, wind) = context("METAR wind summary", opt(preceded(multispace1, WindSummary::parse)))(input)?;
        let (input, variable_wind_dir) = opt(preceded(multispace0, MetarVariableWindDir::parse))(input)?;

        let (input, visibility) = opt(preceded(multispace0, vvvv))(input)?;

        let (input, minimum_visibility) = opt(preceded(
            multispace0,
            tuple((
                fromstr_n::<'_, f32>(4),
                alt((
                    tag("N").map(|_| Compass::North),
                    tag("NE").map(|_| Compass::NorthEast),
                    tag("E").map(|_| Compass::East),
                    tag("SE").map(|_| Compass::SouthEast),
                    tag("S").map(|_| Compass::South),
                    tag("SW").map(|_| Compass::SouthWest),
                    tag("W").map(|_| Compass::West),
                    tag("NW").map(|_| Compass::NorthWest),
                )),
            ))
            .map(|(len, direction)| MetarMinimumVisibility {
                visibility: Length::new::<meter>(len),
                direction,
            }),
        ))(input)?;

        let (input, runway_range) = multi(preceded(
            space0,
            preceded(
                char('R'),
                separated_pair(
                    RunwayDesignator::parse,
                    char('/'),
                    tuple((
                        fromstr_n(4).map(|v: f32| Length::new::<meter>(v)),
                        map_res(anychar, |c: char| {
                            Ok(match c {
                                'U' => RunwayTrend::Farther,
                                'D' => RunwayTrend::Closer,
                                'N' => RunwayTrend::NoChange,
                                _ => return Err("Unknown runway trend code"),
                            })
                        }),
                    )),
                )
                .map(|(designator, (distance, trend))| (designator, distance, trend)),
            ),
        ))
        .parse(input)?;

        let (input, weather) = multi(preceded(multispace0, SignificantWeather::parse)).parse(input)?;

        let (input, clouds) = multi_opt(
            preceded(
                multispace0,
                CloudReport::parse,
            ),
        ).parse(input)?;

        let (input, air_dewpoint_temperature) = opt(
            preceded(
                multispace0,
                alt((
                    separated_pair(opt(temperature(2)), char('/'), opt(temperature(2))).map(|(a, d)| match (a, d) {
                        (Some(a), Some(d)) => Some((a, d)),
                        _ => None,
                    }),
                    tag("/////").map(|_| None)
                )),
            )
        ).map(Option::flatten).parse(input)?;

        let (input, qnh) = opt(
            preceded(
                multispace0,
                alt((
                    preceded(
                        char('Q'),
                        alt((
                            fromstr_n(4)
                                .map(|v| Pressure::new::<hectopascal>(v))
                                .map(Some),
                            tag("////").map(|_| None),
                        ))
                    ),
                    preceded(
                        char('A'),
                        alt((
                            fromstr_n(4)
                                .map(|v: f32| Pressure::new::<inch_of_mercury>(v / 100.))
                                .map(Some),
                            tag("////").map(|_| None),
                        )),
                    ),
                ))
            )
        ).map(Option::flatten).parse(input)?;

        let (input, recent_weather) = opt(
            preceded(
                multispace0,
                alt((
                    preceded(tag("RE"), SignificantWeather::parse).map(Some),
                    tag("NOSIG").map(|_| None),
                ))
            ),
        ).map(Option::flatten).parse(input)?;

        let (input, runway_wind_shear) = opt(preceded(multispace0, RunwayWindShear::parse))(input)?;

        let (input, sea) = multi(preceded(multispace0, MetarSeaSurfaceReport::parse)).parse(input)?;

        let (input, runway_status) = multi(preceded(multispace0, RunwayState::parse)).parse(input)?;
    
        let (input, _) = opt(
            preceded(
                preceded(multispace0, tag("RMK")),
                take_till(|c| c == '='),
            )
        )(input)?;

        Ok((
            input,
            Some(Self {
                country,
                origin,
                kind,
                wind,
                variable_wind_dir,
                visibility,
                minimum_visibility,
                runway_range,
                weather,
                clouds,
                air_dewpoint_temperature,
                qnh,
                recent_weather,
                runway_wind_shear,
                sea,
                runway_status,
            }),
        ))
    }
}

impl RunwayState {
    pub fn parse(input: &str) -> ParseResult<&str, Self> {
        let (input, (runway, deposits, level, depth, friction)) = preceded(
            char('R'),
            tuple((
                terminated(RunwayDesignator::parse, char('/')),
                RunwayDeposits::parse,
                RunwayContaminationLevel::parse,
                RunwayDepositDepth::parse,
                RunwaySurfaceFriction::parse,
            )),
        )(input)?;

        Ok((
            input,
            Self {
                runway,
                deposits,
                level,
                depth,
                friction,
            },
        ))
    }
}

impl RunwayWindShear {
    pub fn parse(input: &str) -> ParseResult<&str, Self> {
        preceded(
            tuple((tag("WS"), space1)),
            alt((
                tag("ALL RWY").map(|_| Self::All),
                preceded(
                    char('R'),
                    fromstr_n(2).map(|v| Self::Within(Length::new::<meter>(v))),
                ),
            )),
        )(input)
    }
}

impl MetarSeaSurfaceReport {
    pub fn parse(input: &str) -> ParseResult<&str, Self> {
        let (input, temp) = preceded(char('W'), terminated(temperature(2), char('/')))(input)?;

        alt((
            preceded(char('S'), StateOfTheSea::parse)
                .map(move |state| Self::StateOfSea { temp, state }),
            preceded(
                char('H'),
                fromstr_n(3).map(|v: f32| Length::new::<decimeter>(v)),
            )
            .map(move |height| Self::WaveHeight { temp, height }),
        ))(input)
    }
}

impl MetarVariableWindDir {
    pub fn parse(input: &str) -> ParseResult<&str, Self> {
        let (input, extreme_ccw) = fromstr_n::<f32>(3).map(|v| Angle::new::<degree>(v)).parse(input)?;
        let (input, _) = char('V')(input)?;
        let (input, extreme_cw) = fromstr_n::<f32>(3).map(|v| Angle::new::<degree>(v)).parse(input)?;

        Ok((
            input,
            Self {
                extreme_ccw,
                extreme_cw,
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const METAR: &str = include_str!("test/metar.txt");

    #[test]
    pub fn test_metar() {
        let (_, m) = EmwinMetarReport::parse(NaiveDate::from_ymd(1, 1, 1))(METAR)
            .unwrap_or_else(|e| panic!("{}", crate::display_error(e)));

        panic!("{:#?}", m);
    }
}
