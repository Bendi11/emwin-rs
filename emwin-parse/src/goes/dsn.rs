use std::str::FromStr;

use nom::{branch::alt, Parser, sequence::{preceded, terminated}, character::streaming::char, combinator::map_res};
use nom_supreme::tag::complete::tag;

use crate::{ParseResult, parse::fromstr};

/// A channel number between 01 and 16
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Channel(u8);

/// Instrument that an image was taken with, represented as an enum with one variant in case more
/// satellites with more instruments are ever put into orbit
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum Instrument {
    AdvancedBaselineImager,
}

/// Either level 1b or 2+ processing level for [DataShortName] with product acronym
#[derive(Clone, Copy, Debug)]
pub enum ProductAcronym {
    /// Always radiance data
    L1b(Channel),
    L2(L2Acronym),
}

/// Product acronyms for [level 2+](ProductAcronym::L2)
#[derive(Clone, Copy, Debug)]
pub enum L2Acronym {
    CloudTopHeight,
    CloudTopTemperature,
    ClearSkyMasks,
    CloudTopPhase,
    AerosolOpticalDepth,
    CloudMoistureImagery(Channel),
    MultibandCloudMoistureImagery,
    CloudOpticalDepth,
    CloudParticleSizeDistribution,
    CloudTopPressure,
    DerivedMotionWinds(Channel),
    DerivedMotionWindsBand8,
    DerivedStabilityIndices,
    DownwardShortwaveSurface,
    FireHotCharacterization,
    SnowCover,
    LandSkinTemperature,
    LegacyVerticalMoistureProfile,
    LegacyVerticalTemperatureProfile,
    RainfallRate,
    ReflectedShortwave,
    SeaSkinTemperature,
    TotalPrecipitableWater,
}

#[derive(Clone, Copy, Debug)]
pub enum ABISector {
    FullDisk,
    CONUS,
    Mesoscale1,
    Mesoscale2,
}

#[derive(Clone, Copy, Debug)]
pub enum ABIMode {
    Mode3,
    Mode4,
    Mode6,
}

/// Structure representing a full DSN part of a GOES-R series filename
#[derive(Clone, Copy, Debug)]
pub struct DataShortName {
    pub instrument: Instrument,
    pub acronym: ProductAcronym,
    pub sector: ABISector,
    pub mode: ABIMode,
}

impl Channel {
    /// Panics if `ch` is not within [1, 16]
    pub fn new(ch: u8) -> Self {
        assert!((1..=16).contains(&ch), "Channel::new called with invalid channel {}", ch);
        Self(ch)
    }
}

impl DataShortName {
    pub fn parse(input: &str) -> ParseResult<&str, Self> {
        let (input, instrument) = Instrument::parse(input)?;
        let (input, acronym) = ProductAcronym::parse(input)?;
        let (input, sector) = ABISector::parse(input)?;
        let (input, mode): (_, ABIMode) = fromstr(3)(input)?;

        let channel = preceded(char('C'), map_res(fromstr::<u8>(2), |v| Channel::try_from(v)));

        let (input, acronym) = match acronym {
            ProductAcronym::L2(L2Acronym::DerivedMotionWinds(_)) => 
                    channel
                        .map(|c| ProductAcronym::L2(L2Acronym::DerivedMotionWinds(c)))
                        .parse(input)?,
            ProductAcronym::L1b(_) => channel
                .map(|c| ProductAcronym::L1b(c))
                .parse(input)?,
            ProductAcronym::L2(L2Acronym::CloudMoistureImagery(_)) => channel
                .map(|c| ProductAcronym::L2(L2Acronym::CloudMoistureImagery(c)))
                .parse(input)?,
            other => (input, other),
        };

        Ok((
            input,
            Self {
                instrument,
                acronym,
                sector,
                mode,
            }
        ))
    }
}

impl ProductAcronym {
    fn parse(input: &str) -> ParseResult<&str, Self> {
        alt((
            terminated(
                tag("-L1b")
                    .map(|_| Self::L1b(Channel(1))),
                tag("-Rad"),
            ),
            preceded(
                tag("-L2"),
                alt((
                    alt((
                        tag("ACHA").map(|_| L2Acronym::CloudTopHeight),
                        tag("ACHT").map(|_| L2Acronym::CloudTopTemperature),
                        tag("ACM").map(|_| L2Acronym::ClearSkyMasks),
                        tag("ACTP").map(|_| L2Acronym::CloudTopPhase),
                        tag("ADP").map(|_| L2Acronym::AerosolOpticalDepth),
                        tag("CMIP").map(|_| L2Acronym::CloudMoistureImagery(Channel(1))),
                        tag("MCMIP").map(|_| L2Acronym::MultibandCloudMoistureImagery),
                        tag("COD").map(|_| L2Acronym::CloudOpticalDepth),
                        tag("CPS").map(|_| L2Acronym::CloudParticleSizeDistribution),
                        tag("CTP").map(|_| L2Acronym::CloudTopTemperature),
                        tag("DMW").map(|_| L2Acronym::DerivedMotionWinds(Channel(1))),
                    )),
                    alt((
                        tag("DMWV").map(|_| L2Acronym::DerivedMotionWindsBand8),
                        tag("DSI").map(|_| L2Acronym::DerivedStabilityIndices),
                        tag("DSR").map(|_| L2Acronym::DownwardShortwaveSurface),
                        tag("FDC").map(|_| L2Acronym::FireHotCharacterization),
                        tag("FSC").map(|_| L2Acronym::SnowCover),
                        tag("LST").map(|_| L2Acronym::LandSkinTemperature),
                        tag("LVMP").map(|_| L2Acronym::LegacyVerticalMoistureProfile),
                        tag("LVTP").map(|_| L2Acronym::LegacyVerticalTemperatureProfile),
                        tag("RRQPE").map(|_| L2Acronym::RainfallRate),
                        tag("RSR").map(|_| L2Acronym::ReflectedShortwave),
                        tag("SST").map(|_| L2Acronym::SeaSkinTemperature),
                        tag("TPW").map(|_| L2Acronym::TotalPrecipitableWater),
                    )),
                ))
                .map(|acronym| Self::L2(acronym))
            )
        ))(input)
    }
}

impl ABISector {
    fn parse(input: &str) -> ParseResult<&str, Self> {
        alt((
            tag("F").map(|_| Self::FullDisk),
            tag("C").map(|_| Self::CONUS),
            tag("M1").map(|_| Self::Mesoscale1),
            tag("M2").map(|_| Self::Mesoscale2),
        )).parse(input)
    }
}

impl Instrument {
    fn parse(input: &str) -> ParseResult<&str, Self> {
        tag("ABI").map(|_| Self::AdvancedBaselineImager).parse(input)
    }
}

#[derive(Clone, Copy, Debug, thiserror::Error)]
#[error("Invalid GOES ABI mode")]
pub struct InvalidABIMode;

impl FromStr for ABIMode {
    type Err = InvalidABIMode;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "-M3" => Self::Mode3,
            "-M4" => Self::Mode4,
            "-M6" => Self::Mode6,
            _ => return Err(InvalidABIMode),
        })
    }
}

#[derive(Clone, Copy, Debug, thiserror::Error)]
#[error("Channel identifier is not in the range 01-16")]
pub struct InvalidChannel;

impl TryFrom<u8> for Channel {
    type Error = InvalidChannel;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match (1..=16).contains(&value) {
            true => Ok(Self(value)),
            false => Err(InvalidChannel),
        }    
    }
}

impl AsRef<u8> for Channel {
    fn as_ref(&self) -> &u8 {
        &self.0
    }
}
