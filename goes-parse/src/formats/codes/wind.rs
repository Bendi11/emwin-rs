use nom::{
    branch::alt,
    bytes::complete::take,
    character::streaming::char,
    combinator::{map_res, opt},
    error::context,
    sequence::preceded,
    Parser,
};
use nom_supreme::tag::complete::tag;
use uom::si::{
    angle::degree,
    f32::{Angle, Velocity},
    velocity::{knot, meter_per_second},
};

use crate::{parse::fromstr, ParseResult};

/// Wind report on direction and speed parsed from dddff**G**f*m*f*m* format
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct WindSummary {
    pub direction: Angle,
    pub speed: Velocity,
    pub max_speed: Option<Velocity>,
}

/// Parse the direction that wind is blowing in `dd` format (code table 0878)
pub fn dd(input: &str) -> ParseResult<&str, Angle> {
    context(
        "wind direction 'dd'",
        alt((
            fromstr(2).map(|d: f32| Angle::new::<degree>(d * 10f32)),
            tag("//").map(|_| Angle::new::<degree>(0f32)),
        )),
    )(input)
}

/// Parse a direction the wind is blowing in `ddd` (pg. 181)
pub fn ddd(input: &str) -> ParseResult<&str, Angle> {
    context(
        "wind direction 'ddd'",
        map_res(take(3usize), |s: &str| match s {
            "VRB" => Ok(0f32),
            "///" => Ok(0f32),
            _ => s.parse(),
        })
        .map(|v| Angle::new::<degree>(v)),
    )(input)
}

/// Parse a wind speed with no units in `ff` format (pg. 184)
pub fn ff(input: &str) -> ParseResult<&str, f32> {
    context("wind speed without units 'ff'", fromstr(2))(input)
}

impl WindSummary {
    pub fn parse(input: &str) -> ParseResult<&str, Self> {
        let (input, direction) = ddd(input)?;
        let (input, speed) = ff(input)?;

        let (input, max_speed) = opt(preceded(char('G'), fromstr(2)))(input)?;

        let (input, (speed, max_speed)) = context(
            "wind speed units",
            alt((
                map_res(tag("KT"), |_| {
                    Ok::<_, &str>((
                        Velocity::new::<knot>(speed),
                        max_speed.map(Velocity::new::<knot>),
                    ))
                }),
                map_res(tag("MPS"), |_| {
                    Ok::<_, &str>((
                        Velocity::new::<meter_per_second>(speed),
                        max_speed.map(Velocity::new::<meter_per_second>),
                    ))
                }),
            )),
        )(input)?;

        Ok((
            input,
            Self {
                direction,
                speed,
                max_speed,
            },
        ))
    }
}
