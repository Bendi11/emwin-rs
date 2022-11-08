use nom::{character::complete::anychar, combinator::map_res};

use crate::ParseResult;


/// Runway deposits as specified in code table 0919
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunwayDeposits {
    Clear,
    Damp,
    Wet,
    RimeFrost,
    DrySnow,
    WetSnow,
    Slush,
    Ice,
    CompactedSnow,
    FrozenRuts,
    NotReported,
}

/// Runway contamination level as specified in code table 0519
#[derive(Clone, Copy, Debug,)]
pub enum RunwayContaminationLevel {
    Percent(f32),
    NotReported,
}

impl RunwayDeposits {
    pub fn parse(input: &str) -> ParseResult<&str, Self> {
        map_res(
            anychar,
            |c: char| Ok(match c {
                '0' => Self::Clear,
                '1' => Self::Damp,
                '2' => Self::Wet,
                '3' => Self::RimeFrost,
                '4' => Self::DrySnow,
                '5' => Self::WetSnow,
                '6' => Self::Slush,
                '7' => Self::Ice,
                '8' => Self::CompactedSnow,
                '9' => Self::FrozenRuts,
                '/' => Self::NotReported,
                _ => return Err("Unknown runway deposit code"),
            })
        )(input)
    }
}

impl RunwayContaminationLevel {

}
