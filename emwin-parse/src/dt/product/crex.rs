use crate::dt::{UnparsedProductIdentifier, DataTypeDesignatorParseError, area::GeographicalAreaDesignator};

use super::{ObservationalBUFROceanic, ForecastBUFRSurfaceData, ObservationalBUFRSurfaceSeaLevel, ForecastBUFRTextData, ObservationalBUFRUpperAir};


/// K
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CREX {
    /// T2A1
    pub subtype: CREXSubType,
    /// A2
    pub area: GeographicalAreaDesignator,
    /// ii
    pub enumerator: u8,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CREXPictoral {
    SeaIce,
    SeaSurfaceAndBelowSoundings,
    SeaSurfaceTemperature,
    SeaSurfaceWaves,
    OtherSeaEnvironmental,
}


/// Term T2 definitions when T1=ObservationalDataBinaryBUFR or ForecastBinaryBUFR
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CREXSubType {
    /// F
    Forecast(ForecastBUFRSurfaceData),
    /// O
    OceanographicLimnographic(ObservationalBUFROceanic),
    /// P
    Pictorial(CREXPictoral),
    /// S
    SurfaceSeaLevel(ObservationalBUFRSurfaceSeaLevel),
    /// T
    Text(ForecastBUFRTextData),
    /// U
    UpperAir(ObservationalBUFRUpperAir),
    /// V
    SIGW(ForecastBUFRSurfaceData),
    /// X
    Other,
}

impl TryFrom<UnparsedProductIdentifier> for CREX {
    type Error = DataTypeDesignatorParseError;
    fn try_from(value: UnparsedProductIdentifier) -> Result<Self, Self::Error> {
        Ok(Self {
            subtype: match value.t2 {
                'F' => CREXSubType::Forecast(ForecastBUFRSurfaceData::try_from(value)?),
                'O' => CREXSubType::OceanographicLimnographic(ObservationalBUFROceanic::try_from(value)?),
                'P' => CREXSubType::Pictorial(match value.a1 {
                    'I' => CREXPictoral::SeaIce,
                    'S' => CREXPictoral::SeaSurfaceAndBelowSoundings,
                    'T' => CREXPictoral::SeaSurfaceTemperature,
                    'W' => CREXPictoral::SeaSurfaceWaves,
                    'X' => CREXPictoral::OtherSeaEnvironmental,
                    other => return Err(DataTypeDesignatorParseError::UnrecognizedA1(value.t1, value.t2, other)),
                }),
                'S' => CREXSubType::SurfaceSeaLevel(ObservationalBUFRSurfaceSeaLevel::try_from(value)?),
                'T' => CREXSubType::Text(ForecastBUFRTextData::try_from(value)?),
                'U' => CREXSubType::UpperAir(ObservationalBUFRUpperAir::try_from(value)?),
                'V' => CREXSubType::SIGW(ForecastBUFRSurfaceData::try_from(value)?),
                'X' => CREXSubType::Other,
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2(value.t1, other)),
            },
            area: GeographicalAreaDesignator::try_from(value.a2)?,
            enumerator: value.ii,
        })
    }
}
