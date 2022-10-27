use crate::dt::area::ReferenceTimeDesignator;


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
