use nom::{combinator::map_res, Parser, error::context, bytes::complete::take};
use uom::si::{f32::Angle, angle::degree};

use crate::{ParseResult, parse::fromstr};

/// Parse the direction that wind is blowing in `dd` format (code table 0878)
pub fn dd(input: &str) -> ParseResult<&str, Angle> {
    context(
        "wind direction 'dd'",
        fromstr(2) 
            .map(|d: f32| Angle::new::<degree>(d * 10f32))
    )(input)
}

/// Parse a direction the wind is blowing in `ddd` (pg. 181)
pub fn ddd(input: &str) -> ParseResult<&str, Angle> {
    context(
        "wind direction 'ddd'",
        map_res(
            take(3usize),
            |s: &str| match s {
                "VRB" => Ok(0f32),
                _ => s.parse(),
            }
        )
        .map(|v| Angle::new::<degree>(v))
    )(input)
}

/// Parse a wind speed with no units in `ff` format (pg. 184)
pub fn ff(input: &str) -> ParseResult<&str, f32> {
    context(
        "wind speed without units 'ff'",
        fromstr(2), 
    )(input)
}
