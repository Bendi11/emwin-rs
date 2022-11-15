use crate::dt::{
    area::{GeographicalAreaDesignator, ReferenceTimeDesignator},
    level::SeaLevelDesignator,
    DataTypeDesignatorParseError, UnparsedProductIdentifier,
};

/// O
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OceanographicInformation {
    /// T2
    pub subtype: OceanographicSubType,
    /// A1
    pub area: GeographicalAreaDesignator,
    /// A2
    pub time: ReferenceTimeDesignator,
    /// ii
    pub level: SeaLevelDesignator,
}

/// Term T2 definitions when T1=OceanographicInformation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OceanographicSubType {
    Depth,
    IceConcentration,
    IceThickness,
    IceDrift,
    IceGrowth,
    IceConvergenceDivergence,
    TemperatureAnomaly,
    DepthAnomaly,
    Salinity,
    Temperature,
    CurrentComponent,
    TemperatureWarming,
    Mixed,
}

impl TryFrom<UnparsedProductIdentifier> for OceanographicInformation {
    type Error = DataTypeDesignatorParseError;
    fn try_from(value: UnparsedProductIdentifier) -> Result<Self, Self::Error> {
        Ok(Self {
            subtype: match value.t2 {
                'D' => OceanographicSubType::Depth,
                'E' => OceanographicSubType::IceConcentration,
                'F' => OceanographicSubType::IceThickness,
                'G' => OceanographicSubType::IceDrift,
                'H' => OceanographicSubType::IceGrowth,
                'I' => OceanographicSubType::IceConvergenceDivergence,
                'Q' => OceanographicSubType::TemperatureAnomaly,
                'R' => OceanographicSubType::DepthAnomaly,
                'S' => OceanographicSubType::Salinity,
                'T' => OceanographicSubType::Temperature,
                'U' | 'V' => OceanographicSubType::CurrentComponent,
                'W' => OceanographicSubType::TemperatureWarming,
                'X' => OceanographicSubType::Mixed,
                other => {
                    return Err(DataTypeDesignatorParseError::UnrecognizedT2(
                        value.t1, other,
                    ))
                }
            },
            area: GeographicalAreaDesignator::try_from(value.a1)?,
            time: ReferenceTimeDesignator::parse_for_dghjopt(value.a2)?,
            level: SeaLevelDesignator::try_from(value.ii)?,
        })
    }
}
