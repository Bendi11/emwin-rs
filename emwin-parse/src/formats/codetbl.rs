use nom::{bytes::complete::take, combinator::map_res, error::context, Parser};
use uom::si::{f32::Length, length::meter};

use crate::{ParseResult, parse::fromstr};

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
