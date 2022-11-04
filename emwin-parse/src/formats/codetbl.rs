use std::num::ParseFloatError;

use nom::{IResult, combinator::map_res, bytes::complete::take};
use uom::si::{f32::Length, length::meter};


/// Parse altitude levels using code table 1690
pub fn parse_1690(input: &str) -> IResult<&str, Length> {
    map_res(
        take(3usize),
        |s: &str| Ok::<_, ParseFloatError>(Length::new::<meter>(s.parse::<f32>()? * 30f32)),
    )(input)
}
