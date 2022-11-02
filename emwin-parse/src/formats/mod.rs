use std::str::FromStr;

use nom::{IResult, combinator::map_res, bytes::complete::take};

pub mod rwr;
pub mod amdar;

/// Parse an angle in degrees minutes (DDMM) format
pub fn parse_degreesminutes(input: &str) -> IResult<&str, f32> {
    let (input, degrees) = map_res(
        take(2usize),
        |s: &str| s.parse::<f32>(),
    )(input)?;

    let (input, minutes) = map_res(
        take(2usize),
        |s: &str| s.parse::<f32>(),
    )(input)?;

    Ok((input, degrees + minutes / 60f32))
}


#[derive(Clone, Copy, Debug,)]
pub struct Latitude {
    pub angle: f32,
    pub dir: LatitudeDir,
}

#[derive(Clone, Copy, Debug,)]
pub struct Longitude {
    pub angle: f32,
    pub dir: LongitudeDir,
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

#[derive(Clone, Copy, Debug)]
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
