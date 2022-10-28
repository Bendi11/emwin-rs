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
    /// ii
    pub enumerator: u8,
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
            subtype: match value.t2 {
                'B' => SatelliteDataSubType::SatelliteOrbitParameters,
                'C' => SatelliteDataSubType::SatelliteCloudInterpretations,
                'H' => SatelliteDataSubType::SatelliteRemoteUpperAirSoundings,
                'R' => SatelliteDataSubType::ClearRadianceObservations,
                'T' => SatelliteDataSubType::SeaSurfaceTemperatures,
                'W' => SatelliteDataSubType::WindsAndCloudsTemperatures,
                'X' => SatelliteDataSubType::Misc,
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2('T', other)),
            },
            area: GeographicalAreaDesignator::try_from(value.a1)?,
            time: ReferenceTimeDesignator::parse_for_dghjopt(value.a2)?,
            enumerator: value.ii,
        })
    }
}
