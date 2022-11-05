use std::num::ParseFloatError;

use nom::{combinator::map_res, bytes::complete::take};
use uom::si::{f32::Length, length::meter};

use crate::ParseResult;


/// Parse altitude levels using code table 1690
pub fn parse_1690(input: &str) -> ParseResult<&str, Length> {
    map_res(
        take(3usize),
        |s: &str| Ok::<_, ParseFloatError>(Length::new::<meter>(s.parse::<f32>()? * 30f32)),
    )(input)
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
        map_res(
            take(2usize),
            |s: &str| Ok(match s {
                "AT" => Self::At,
                "FM" => Self::From,
                "TL" => Self::Until,
                _ => return Err("Invalid time group")
            })
        )(input)
    }
}
