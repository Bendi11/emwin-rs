use nom::{character::streaming::char, combinator::{opt, map_res}, sequence::tuple, Parser, error::context, bytes::complete::take};
use uom::si::{f32::{ThermodynamicTemperature, Length}, thermodynamic_temperature::degree_celsius, length::meter};

use crate::{parse::fromstr, ParseError, ParseResult};

pub mod clouds;
pub mod runway;
pub mod sea;
pub mod visibility;
pub mod weather;
pub mod wind;

/// Parse a temperature in degrees C with optional preceding `M` character indicating minus
pub fn temperature<'a>(
    len: usize,
) -> impl Parser<&'a str, ThermodynamicTemperature, ParseError<&'a str>> {
    tuple((
        opt(char('M').map(|_| -1f32)).map(|v| v.unwrap_or(1f32)),
        fromstr::<'_, f32>(len),
    ))
    .map(|(m, t)| ThermodynamicTemperature::new::<degree_celsius>(t * m))
}

/// Parse altitude levels using code table 1690
pub fn parse_1690(input: &str) -> ParseResult<&str, Length> {
    context("altitude (code table 1690)", fromstr(3))
        .map(|v: f32| Length::new::<meter>(v * 30f32))
        .parse(input)
}

/// Time group specified by symbols TT
#[derive(Clone, Copy, Debug)]
pub enum TimeGroup {
    At,
    From,
    Until,
}

impl TimeGroup {
    pub fn parse(input: &str) -> ParseResult<&str, Self> {
        map_res(take(2usize), |s: &str| {
            Ok(match s {
                "AT" => Self::At,
                "FM" => Self::From,
                "TL" => Self::Until,
                _ => return Err("Invalid time group"),
            })
        })(input)
    }
}
