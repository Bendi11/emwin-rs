use crate::dt::{area::ReferenceTimeDesignator, UnparsedProductIdentifier, DataTypeDesignatorParseError};


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForecastBUFROceanographicData {
    /// I
    SeaIce,
    /// S
    SeaSurfaceAndBelow,
    /// T
    SeaSurfaceTemperature,
    /// W
    SeaSurfaceWaves,
    /// X
    OtherSeaEnvironmentalData,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForecastBUFRSurfaceData {
    /// A
    SurfaceArea,
    /// D
    Radiological,
    /// M
    Surface,
    /// O
    Maritime,
    /// P
    Amendment,
    /// R
    Hydrologic,
    /// S
    AmendmentTAF,
    /// T
    AerodomeTAF,
    /// X
    OtherSurface,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForecastBUFRTextData {
    /// E
    Tsunami,
    /// H
    HurricaneTyphoonStormWarning,
    /// S
    SevereWeatherSIGMET,
    /// T
    TornadoWarning,
    /// X
    OtherWarning,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForecastBUFRUpperAirData {
    /// A
    SingleLevel,
    /// B
    SIGWXEmbeddedCumulonimbus,
    /// C
    SIGWXClearAirTurbulence,
    /// F
    SIGWXFront,
    /// N
    SIGWXOtherParameters,
    /// O
    SIGWXTurbulence,
    /// S
    Soundings,
    /// T
    SIGWXIcingTropopause,
    /// V
    SIGWXTropicalStormSandstormVolcano,
    /// W
    SIGWXHighLevelWinds,
    /// X
    OtherUpperAir,
}

/// Term T2 definitions when T1=ObservationalDataBinaryBUFR or ForecastBinaryBUFR
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForecastDataBinaryBUFRSubType {
    /// N
    SatelliteData,
    /// O
    OceanographicLimnographic(ForecastBUFROceanographicData),
    /// P
    Pictorial,
    /// S
    SurfaceSeaLevel(ForecastBUFRSurfaceData),
    /// T
    Text(ForecastBUFRTextData),
    /// U
    UpperAir(ForecastBUFRUpperAirData),
    /// X
    Other,
}

/// J
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ForecastDataBinary {
    /// T2
    pub subtype: ForecastDataBinaryBUFRSubType,
    /// A2
    pub time: ReferenceTimeDesignator,
}

impl TryFrom<UnparsedProductIdentifier> for ForecastDataBinary {
    type Error = DataTypeDesignatorParseError;
    fn try_from(value: UnparsedProductIdentifier) -> Result<Self, Self::Error> {
        Ok(Self {
            subtype: match value.t2 {
                'N' => ForecastDataBinaryBUFRSubType::SatelliteData,
                'O' => ForecastDataBinaryBUFRSubType::OceanographicLimnographic(match value.a1 {
                    'I' => ForecastBUFROceanographicData::SeaIce,
                    'S' => ForecastBUFROceanographicData::SeaSurfaceAndBelow,
                    'T' => ForecastBUFROceanographicData::SeaSurfaceTemperature,
                    'W' => ForecastBUFROceanographicData::SeaSurfaceWaves,
                    'X' => ForecastBUFROceanographicData::OtherSeaEnvironmentalData,
                    other => return Err(DataTypeDesignatorParseError::UnrecognizedA1(value.t1, value.t2, other)),
                }),
                'P' => ForecastDataBinaryBUFRSubType::Pictorial,
                'S' => ForecastDataBinaryBUFRSubType::SurfaceSeaLevel(match value.a1 {
                    'A' => ForecastBUFRSurfaceData::SurfaceArea,
                    'D' => ForecastBUFRSurfaceData::Radiological,
                    'M' => ForecastBUFRSurfaceData::Surface,
                    'O' => ForecastBUFRSurfaceData::Maritime,
                    'P' => ForecastBUFRSurfaceData::Amendment,
                    'R' => ForecastBUFRSurfaceData::Hydrologic,
                    'S' => ForecastBUFRSurfaceData::AmendmentTAF,
                    'T' => ForecastBUFRSurfaceData::AerodomeTAF,
                    'X' => ForecastBUFRSurfaceData::OtherSurface,
                    other => return Err(DataTypeDesignatorParseError::UnrecognizedA1(value.t1, value.t2, other)),
                }),
                'T' => ForecastDataBinaryBUFRSubType::Text(match value.a1 {
                    'E' => ForecastBUFRTextData::Tsunami,
                    'H' => ForecastBUFRTextData::HurricaneTyphoonStormWarning,
                    'S' => ForecastBUFRTextData::SevereWeatherSIGMET,
                    'T' => ForecastBUFRTextData::TornadoWarning,
                    'X' => ForecastBUFRTextData::OtherWarning,
                    other => return Err(DataTypeDesignatorParseError::UnrecognizedA1(value.t1, value.t2, other)),
                }),
                'U' => ForecastDataBinaryBUFRSubType::UpperAir(match value.a1 {
                    'A' => ForecastBUFRUpperAirData::SingleLevel,
                    'B' => ForecastBUFRUpperAirData::SIGWXEmbeddedCumulonimbus,
                    'C' => ForecastBUFRUpperAirData::SIGWXClearAirTurbulence,
                    'F' => ForecastBUFRUpperAirData::SIGWXFront,
                    'N' => ForecastBUFRUpperAirData::SIGWXOtherParameters,
                    'O' => ForecastBUFRUpperAirData::SIGWXTurbulence,
                    'S' => ForecastBUFRUpperAirData::Soundings,
                    'T' => ForecastBUFRUpperAirData::SIGWXIcingTropopause,
                    'V' => ForecastBUFRUpperAirData::SIGWXTropicalStormSandstormVolcano,
                    'W' => ForecastBUFRUpperAirData::SIGWXHighLevelWinds,
                    'X' => ForecastBUFRUpperAirData::OtherUpperAir,
                    other => return Err(DataTypeDesignatorParseError::UnrecognizedA1(value.t1, value.t2, other)),
                }),
                'X' => ForecastDataBinaryBUFRSubType::Other,
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2(value.t1, other)),
            },
            time: ReferenceTimeDesignator::parse_for_dghjopt(value.a2)?,
        })
    }
}
