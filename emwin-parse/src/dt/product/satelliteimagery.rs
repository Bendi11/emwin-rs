use crate::dt::{area::AreaCode, DataTypeDesignatorParseError, UnparsedProductIdentifier};

/// E
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SatelliteImagery {
    pub subtype: SatelliteImagerySubType,
    pub area: AreaCode,
}

/// Term T2 definitions when T1=SatelliteImagery
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SatelliteImagerySubType {
    CloudTopTemperature,
    Fog,
    Infared,
    SurfaceTemperature,
}

impl TryFrom<UnparsedProductIdentifier> for SatelliteImagery {
    type Error = DataTypeDesignatorParseError;
    fn try_from(value: UnparsedProductIdentifier) -> Result<Self, Self::Error> {
        Ok(Self {
            subtype: match value.t2 {
                'C' => SatelliteImagerySubType::CloudTopTemperature,
                'F' => SatelliteImagerySubType::Fog,
                'I' => SatelliteImagerySubType::Infared,
                'S' => SatelliteImagerySubType::SurfaceTemperature,
                other => {
                    return Err(DataTypeDesignatorParseError::UnrecognizedT2(
                        value.t1, other,
                    ))
                }
            },
            area: AreaCode::try_from((value.a1, value.a2))?,
        })
    }
}
