use chrono::Duration;
use nom::{character::{streaming::char, complete::space1}, combinator::opt, branch::alt, sequence::{preceded, terminated}, Parser};
use nom_supreme::tag::complete::tag;
use uom::si::f32::{Length, ThermodynamicTemperature, Angle};

use crate::{header::CCCC, ParseResult, parse::{time::yygggg, fromstr}};

use super::{codes::{wind::WindSummary, weather::SignificantWeather, visibility::vvvv}, taf::CloudAmount};


/// A single METAR weather report parsed from a FM 15/16 report
#[derive(Clone, Debug, )]
pub struct MetarReport {
    pub country: CCCC,
    pub origin: Duration,
    pub wind: WindSummary,
    pub kind: MetarReportKind,
    pub variable_wind_dir: Option<MetarVariableWindDir>,
    pub visibility: Option<Length>,
    pub weather: SignificantWeather,
    pub clouds: CloudAmount,
    pub temperature: ThermodynamicTemperature,
    pub minimum_visibility: Option<MetarMinimumVisibility>,
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
    pub direction: Angle,
}

/// The kind of report a METAR/SPECI is
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MetarReportKind {
    Auto,
    Cor,
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
