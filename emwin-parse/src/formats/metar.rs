use chrono::Duration;
use nom::{character::{streaming::char, complete::{space1, anychar}}, combinator::{opt, map_res}, branch::alt, sequence::{preceded, terminated, tuple, separated_pair}, Parser, multi::many0};
use nom_supreme::tag::complete::tag;
use uom::si::{f32::{Length, ThermodynamicTemperature, Angle, Pressure}, length::{meter, decimeter}, thermodynamic_temperature::degree_celsius, pressure::hectopascal};

use crate::{header::CCCC, ParseResult, parse::{time::yygggg, fromstr}};

use super::{codes::{wind::WindSummary, weather::SignificantWeather, visibility::vvvv, clouds::CloudReport, sea::StateOfTheSea, temperature, runway::RunwayDeposits}, Compass, RunwayDesignator};


/// A single METAR weather report parsed from a FM 15/16 report
#[derive(Clone, Debug, )]
pub struct MetarReport {
    pub country: CCCC,
    pub origin: Duration,
    pub wind: WindSummary,
    pub kind: MetarReportKind,
    pub variable_wind_dir: Option<MetarVariableWindDir>,
    pub visibility: Option<Length>,
    pub minimum_visibility: Option<MetarMinimumVisibility>,
    pub runway_range: Vec<(RunwayDesignator, Length, RunwayTrend)>,
    pub weather: SignificantWeather,
    pub clouds: Option<CloudReport>,
    pub air_dewpoint_temperature: Option<(ThermodynamicTemperature, ThermodynamicTemperature)>,
    pub qnh: Option<Pressure>,
    pub recent_weather: Option<SignificantWeather>,
    pub runway_wind_shear: Option<RunwayWindShear>,
    pub sea: Option<MetarSeaSurfaceReport>,
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
    }
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
#[derive(Clone, Copy, Debug,)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RunwayState {
    pub runway: RunwayDesignator,
    pub deposits: RunwayDeposits,

}

impl MetarReport {
    /// Returns Ok(None) if the report is `NIL`
    pub fn parse(input: &str) -> ParseResult<&str, Option<Self>> {
        let (input, _) = alt((
            tag("METAR"),
            tag("SPECI"),
        ))(input)?;

        let (input, kind) = opt(
            preceded(
                space1,
                tag("COR").map(|_| MetarReportKind::Cor),
            )
        )(input)?;
        
        let (input, country): (_, CCCC) = preceded(space1, fromstr(4))(input)?;
        let (input, origin) = terminated(yygggg, char('Z'))(input)?;

        let (input, kind) = match kind {
            Some(kind) => (input, kind),
            None => match preceded(space1, alt((
                tag("NIL").map(|_| None),
                tag("AUTO").map(|_| Some(MetarReportKind::Auto)),
            )))(input)? {
                (input, Some(k)) => (input, k),
                (input, None) => return Ok((input, None)),
            }
        };

        let (input, wind) = preceded(space1, WindSummary::parse)(input)?;
        let (input, variable_wind_dir) = opt(
            preceded(
                space1,
                MetarVariableWindDir::parse,
            )
        )(input)?;

        let (input, visibility) = opt(preceded(space1, vvvv))(input)?;
        
        let (input, minimum_visibility) = opt(
            preceded(
                space1,
                tuple((
                    fromstr::<'_, f32>(4),
                    alt((
                        tag("N").map(|_|Compass::North),
                        tag("NE").map(|_| Compass::NorthEast),
                        tag("E").map(|_| Compass::East),
                        tag("SE").map(|_| Compass::SouthEast),
                        tag("S").map(|_| Compass::South),
                        tag("SW").map(|_| Compass::SouthWest),
                        tag("W").map(|_| Compass::West),
                        tag("NW").map(|_| Compass::NorthWest),
                    ))
                )).map(|(len, direction)| MetarMinimumVisibility { visibility: Length::new::<meter>(len), direction })
            )
        )(input)?;

        let (input, runway_range) = many0(preceded(
            space1,
            preceded(
                char('R'),
                separated_pair(
                    RunwayDesignator::parse,
                    char('/'),
                    tuple((
                        fromstr(4).map(|v: f32| Length::new::<meter>(v)),
                        map_res(
                            anychar,
                            |c: char| Ok(match c {
                                'U' => RunwayTrend::Farther,
                                'D' => RunwayTrend::Closer,
                                'N' => RunwayTrend::NoChange,
                                _ => return Err("Unknown runway trend code")
                            })
                        )
                    ))
                )
                .map(|(designator, (distance, trend))| (designator, distance, trend)),
            )
        ))(input)?;

        let (input, weather) = opt(preceded(space1, SignificantWeather::parse))(input)?;
        let (input, clouds) = opt(preceded(space1, CloudReport::parse)).map(Option::flatten).parse(input)?;
        
        let (input, air_temperature) = opt(
            preceded(
                space1,
                separated_pair(
                    temperature(2),
                    char('/'),
                    temperature(2),
                ),
            ),
        )(input)?;

        let (input, qnh) = opt(
            preceded(
                space1,
                preceded(
                    char('Q'),
                    fromstr(4).map(|v| Pressure::new::<hectopascal>(v))
                )
            )
        )(input)?;

        let (input, runway_wind_shear) = opt(preceded(space1, RunwayWindShear::parse))(input)?;
        let (input, sea) = many0(preceded(space1, StateOfTheSea::parse))(input)?;
        
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
                    fromstr(2).map(|v| Self::Within(Length::new::<meter>(v))),
                )
            ))
        )(input)
    }
}

impl MetarSeaSurfaceReport {
    pub fn parse(input: &str) -> ParseResult<&str, Self> {
        let (input, temp) = preceded(
            char('W'),
            terminated(temperature(2), char('/'))
        )(input)?;

        alt((
            preceded(
                char('S'),
                StateOfTheSea::parse,
            ).map(|state| Self::StateOfSea { temp, state }),
            preceded(
                char('H'),
                fromstr(3)
                    .map(|v: f32| Length::new::<decimeter>(v))
            ).map(|height| Self::WaveHeight { temp, height })
        ))(input)
    }
}

impl MetarVariableWindDir {
    pub fn parse(input: &str) -> ParseResult<&str, Self> {
        let (input, extreme_ccw) = fromstr(3)(input)?;
        let (input, _) = char('V')(input)?;
        let (input, extreme_cw) = fromstr(3)(input)?;

        Ok((
            input,
            Self {
                extreme_ccw,
                extreme_cw,
            }
        ))
    }
}
