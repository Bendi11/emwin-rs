use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::take,
    character::streaming::char,
    combinator::{map_res, opt},
    Parser,
};

use crate::{parse::fromstr, ParseResult};

pub mod amdar;
pub mod codes;
pub mod metar;
pub mod rwr;
pub mod taf;

/// A runway designator containing runway number and approach direction
#[derive(Clone, Copy, Debug)]
pub struct RunwayDesignator {
    pub num: u8,
    pub dir: Option<RunwayDesignatorDirection>,
}

/// Letter appended to a two-digit runway designator indicating direction of approach for parallel
/// runways
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunwayDesignatorDirection {
    Left,
    Center,
    Right,
}

/// Eight directions of a compass needle used for rough geographic angles
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compass {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

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

impl RunwayDesignator {
    /// Parse a runway designator from the DrDr{L,C,R} format
    pub fn parse(input: &str) -> ParseResult<&str, Self> {
        let (input, num) = fromstr(2)(input)?;
        let (input, dir) = opt(alt((
            char('L').map(|_| RunwayDesignatorDirection::Left),
            char('C').map(|_| RunwayDesignatorDirection::Center),
            char('R').map(|_| RunwayDesignatorDirection::Right),
        )))(input)?;

        Ok((input, Self { num, dir }))
    }
}
