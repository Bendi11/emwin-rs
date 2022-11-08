use std::num::ParseIntError;

use nom::{
    bytes::complete::take, character::complete::anychar, combinator::map_res, error::context,
};
use uom::si::{
    f32::Length,
    length::{centimeter, millimeter},
};

use crate::ParseResult;

/// Runway deposits as specified in code table 0919
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub enum RunwayContaminationLevel {
    Percent { from: f32, to: f32 },
    NotReported,
}

/// Runway deposit depth as specified by code table 1079
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub enum RunwayDepositDepth {
    Depth(Length),
    Inoperable,
    NotReported,
}

/// Expected runway surface friction as specified by code table 0366
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub enum RunwaySurfaceFriction {
    Coefficient(f32),
    BrakingAction(RunwaySurfaceBrakingAction),
    Unreliable,
    NotReported,
}

#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub enum RunwaySurfaceBrakingAction {
    Poor,
    MediumPoor,
    Medium,
    MediumGood,
    Good,
}

impl RunwayDeposits {
    pub fn parse(input: &str) -> ParseResult<&str, Self> {
        map_res(anychar, |c: char| {
            Ok(match c {
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
        })(input)
    }
}

impl RunwayContaminationLevel {
    pub fn parse(input: &str) -> ParseResult<&str, Self> {
        map_res(anychar, |c: char| {
            Ok(match c {
                '1' => Self::Percent {
                    from: 0.0f32,
                    to: 0.10f32,
                },
                '2' => Self::Percent {
                    from: 0.11f32,
                    to: 0.25f32,
                },
                '5' => Self::Percent {
                    from: 0.26f32,
                    to: 0.50f32,
                },
                '9' => Self::Percent {
                    from: 0.51f32,
                    to: 0.100f32,
                },
                '3' | '4' | '6' | '8' | '/' => Self::NotReported,
                _ => return Err("Unknown runway contamination level"),
            })
        })(input)
    }
}

impl RunwayDepositDepth {
    pub fn parse(input: &str) -> ParseResult<&str, Self> {
        map_res(take(2usize), |s: &str| {
            Ok::<_, ParseIntError>(match s {
                "//" => Self::NotReported,
                _ => {
                    let val = s.parse::<u8>()?;
                    if (0..=90).contains(&val) {
                        Self::Depth(Length::new::<millimeter>(val as f32))
                    } else if val == 91 {
                        Self::NotReported
                    } else {
                        Self::Depth(Length::new::<centimeter>((val as f32 - 90f32) * 5f32))
                    }
                }
            })
        })(input)
    }
}

impl RunwaySurfaceFriction {
    pub fn parse(input: &str) -> ParseResult<&str, Self> {
        context(
            "Runway surface friction codes",
            map_res(take(2usize), |s: &str| {
                Ok::<_, ParseIntError>(match s {
                    "//" => Self::NotReported,
                    _ => {
                        let val = s.parse::<u8>()?;
                        if (0..=90).contains(&val) {
                            Self::Coefficient(val as f32 / 100f32)
                        } else if (91..=95).contains(&val) {
                            Self::BrakingAction(match val {
                                91 => RunwaySurfaceBrakingAction::Poor,
                                92 => RunwaySurfaceBrakingAction::MediumPoor,
                                93 => RunwaySurfaceBrakingAction::Medium,
                                94 => RunwaySurfaceBrakingAction::MediumGood,
                                95 => RunwaySurfaceBrakingAction::Good,
                                _ => unreachable!(),
                            })
                        } else if (96..=98).contains(&val) {
                            Self::NotReported
                        } else {
                            Self::Unreliable
                        }
                    }
                })
            }),
        )(input)
    }
}
