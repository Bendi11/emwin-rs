use nom::{character::complete::anychar, combinator::map_res, error::context};

use crate::ParseResult;

/// Sea state, parsed from code table 3700
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub enum StateOfTheSea {
    Glassy,
    Rippled,
    Wavelets,
    Slight,
    Moderate,
    Rough,
    VeryRough,
    High,
    VeryHigh,
    Phenomenal,
}

impl StateOfTheSea {
    pub fn parse(input: &str) -> ParseResult<&str, Self> {
        context(
            "State of the sea S'",
            map_res(anychar, |c: char| {
                Ok(match c {
                    '0' => Self::Glassy,
                    '1' => Self::Rippled,
                    '2' => Self::Wavelets,
                    '3' => Self::Slight,
                    '4' => Self::Moderate,
                    '5' => Self::Rough,
                    '6' => Self::VeryRough,
                    '7' => Self::High,
                    '8' => Self::VeryHigh,
                    '9' => Self::Phenomenal,
                    _ => return Err("Unrecognized state of the sea code"),
                })
            }),
        )(input)
    }
}
