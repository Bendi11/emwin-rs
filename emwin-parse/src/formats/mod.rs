use std::str::FromStr;

use nom::{bytes::complete::take, combinator::map_res};

use crate::ParseResult;

pub mod amdar;
pub mod codes;
pub mod codetbl;
pub mod rwr;
pub mod taf;
pub mod metar;

/// Parse an angle in degrees minutes ({D}MM) format
pub fn parse_degreesminutes<const D: usize>(input: &str) -> ParseResult<&str, f32> {
    let (input, degrees) = map_res(take(D), |s: &str| s.parse::<f32>())(input)?;

    let (input, minutes) = map_res(take(2usize), |s: &str| s.parse::<f32>())(input)?;

    Ok((input, degrees + minutes / 60f32))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LatitudeDir {
    North,
    South,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LongitudeDir {
    East,
    West,
}

#[derive(Clone, Copy, Debug, thiserror::Error)]
#[error("Invalid compass direction {0}, expecting N, E, S, W")]
pub struct InvalidLatLong(char);

impl FromStr for LongitudeDir {
    type Err = InvalidLatLong;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.chars().next().ok_or_else(|| InvalidLatLong(' '))? {
            'E' => Self::East,
            'W' => Self::West,
            other => return Err(InvalidLatLong(other)),
        })
    }
}

impl FromStr for LatitudeDir {
    type Err = InvalidLatLong;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.chars().next().ok_or_else(|| InvalidLatLong(' '))? {
            'N' => Self::North,
            'S' => Self::South,
            other => return Err(InvalidLatLong(other)),
        })
    }
}

impl LatitudeDir {
    pub fn to_north(&self, ang: f32) -> f32 {
        if *self == Self::South {
            ang * -1f32
        } else {
            ang
        }
    }
}

impl LongitudeDir {
    pub fn to_east(&self, ang: f32) -> f32 {
        if *self == Self::West {
            ang * -1f32
        } else {
            ang
        }
    }
}
