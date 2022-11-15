use std::str::FromStr;

/// A channel number between 01 and 16
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Channel(u8);

/// Two-letter system environment code specifying if a GOES image was received from a test or
/// real-time data transmission
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SystemEnvironment {
    OperationalRealTime,
    OperationalTest,
    TestRealTime,
    TestData,
    TestPlayback,
    TestSimulated,
}

/// Enumeration representing all GOES-R series satellites
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Satellite {
    Goes16,
    Goes17,
    Goes18,
    Goes19,
}

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

#[derive(Clone, Copy, Debug, thiserror::Error)]
#[error("Invalid GOES system environment code")]
pub struct InvalidSystemEnvironment;

impl FromStr for SystemEnvironment {
    type Err = InvalidSystemEnvironment;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "OR" => Self::OperationalRealTime,
            "OT" => Self::OperationalTest,
            "IR" => Self::TestRealTime,
            "IT" => Self::TestData,
            "IP" => Self::TestPlayback,
            "IS" => Self::TestSimulated,
            _ => return Err(InvalidSystemEnvironment),
        })
    }
}

#[derive(Clone, Copy, Debug, thiserror::Error)]
#[error("Invalid GOES-R series satellite identifier")]
pub struct InvalidSatelliteIdentifier;

impl FromStr for Satellite {
    type Err = InvalidSatelliteIdentifier;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "G16" => Self::Goes16,
            "G17" => Self::Goes17,
            "G18" => Self::Goes18,
            "G19" => Self::Goes19,
            _ => return Err(InvalidSatelliteIdentifier),
        })
    }
}

impl AsRef<u8> for Channel {
    fn as_ref(&self) -> &u8 {
        &self.0
    }
}
