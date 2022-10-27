
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFRSatelliteData {
    AMSUA,
    AMSUB,
    CrIS,
    HIRS,
    IRAS,
    HIRAS,
    MWHS,
    MHS,
    IASI,
    ATMS,
    MWTS,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFROceanicData {
    BuoyObservations,
    SeaIce,
    SubsurfaceProfilingFloats,
    SeasurfaceObservations,
    SeasurfaceAndBelowSoundings,
    SeasurfaceTemperature,
    SeasurfaceWaves,
    OtherSeaEnvironmental,
    DeepOceanTsunameter,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFRPictoralData {
    RadarCompositeImagery,
    SatelliteImagery,
    RadarImagery,
    NotDefined,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFRSurfaceSeaLevelData {
    RoutinelyScheduledLandStation(u16),
    NMinuteObservationLandStation(u16),
}

/// Term T2 definitions when T1=ObservationalDataBinaryBUFR or ForecastBinaryBUFR
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFRDataType {
    /// N
    SatelliteData(BUFRSatelliteData),
    /// O
    OceanographicLimnographic(BUFROceanicData),
    /// P
    Pictorial(BUFRPictoralData),
    /// S
    SurfaceSeaLevel,
    /// T
    Text,
    /// U
    UpperAir,
    /// X
    Other,
}
