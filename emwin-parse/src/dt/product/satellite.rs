use crate::dt::{area::{GeographicalAreaDesignator, ReferenceTimeDesignator}, UnparsedProductIdentifier, DataTypeDesignatorParseError};

/// T
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SatelliteData {
    /// T2
    pub subtype: SatelliteDataSubType,
    /// A1
    pub area: GeographicalAreaDesignator,
    /// A2
    pub time: ReferenceTimeDesignator,
}

/// Term T2 definition when T1=satellite
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SatelliteDataSubType {
    SatelliteOrbitParameters,
    SatelliteCloudInterpretations,
    SatelliteRemoteUpperAirSoundings,
    ClearRadianceObservations,
    SeaSurfaceTemperatures,
    WindsAndCloudsTemperatures,
    Misc,
}

impl TryFrom<UnparsedProductIdentifier> for SatelliteData {
    type Error = DataTypeDesignatorParseError;
    fn try_from(value: UnparsedProductIdentifier) -> Result<Self, Self::Error> {
        Ok(Self {
            subtype: match second {
                'B' => SatelliteT2::SatelliteOrbitParameters,
                'C' => SatelliteT2::SatelliteCloudInterpretations,
                'H' => SatelliteT2::SatelliteRemoteUpperAirSoundings,
                'R' => SatelliteT2::ClearRadianceObservations,
                'T' => SatelliteT2::SeaSurfaceTemperatures,
                'W' => SatelliteT2::WindsAndCloudsTemperatures,
                'X' => SatelliteT2::Misc,
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2('T', other)),
            },
        })
    }
}
