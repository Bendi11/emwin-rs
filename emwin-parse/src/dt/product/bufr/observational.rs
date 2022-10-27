use crate::dt::area::GeographicalAreaDesignator;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObservationalDataBinary {
    /// T2
    pub subtype: ObservationalDataBinaryBUFRSubType,
    /// A2
    pub area: GeographicalAreaDesignator,
}

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
pub enum LandStation {
    Fixed,
    Mobile,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFRSurfaceSeaLevelData {
    RoutinelyScheduledLandStation,
    NMinuteObservationLandStation,
    RadarReportAB,
    ClimaticObservationLandStation,
    ClimaticObservationMarine,
    ClimaticObservationMonthly,
    RadiologicalObservation,
    SurfaceOzoneMeasurement,
    AtmosphericsSource,
    IntermediateSynopticObservation(LandStation),
    MainSynopticObservation(LandStation),
    SynopticObservationNonStandardTime(LandStation),
    Hydrologic,
    SynopticObservationMarineStation,
    OneHourObservationMarineStation,
    NMinuteObservationMarineStation,
    TideGaugeObservation,
    ObservedWaterLevelTimeSeries,
    SpecialAeronauticalObservation,
    AviationRoutineWeatherObservation,
    OtherSurfaceData,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFRTextData {
    AdministrativeMessage,
    ServiceMessage,
    RequestData,
    OtherText,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFRUpperAirData {
    SingleLevelAircraftReportAuto,
    SingleLevelAircraftReportManual,
    SingleLevelBalloonReport,
    SingleLevelSatelliteDerivedReport,
    Dropsonde,
    OzoneVerticalSounding,
    DispersalTransportAnalysis,
    UpperWindEntireSounding(LandStation),
    UpperWindEntireSoundingMarine,
    RadioSoundingUpTo100HPA(LandStation),
    RadioSoundingUpTo100HPAMarine,
    TotalOzone,
    ModelDerivedSondes,
    Rocketsondes,
    AircraftAscendDescendProfile,
    Profiler,
    RASSTemperatureProfiler,
    Radiance,
    RadiosondesSounding(LandStation),
    RadiosondesSoundingMarine,
    SatelliteDerivedSondes,
    MonthlyStatisticsDataMarine,
    UpperWindUpTo100HPA(LandStation),
    UpperWindMarineUpTo100HPAMarine,
    OtherUpperAirReport,
}


/// Term T2 definitions when T1=ObservationalDataBinaryBUFR or ForecastBinaryBUFR
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObservationalDataBinaryBUFRSubType {
    /// N
    SatelliteData(BUFRSatelliteData),
    /// O
    OceanographicLimnographic(BUFROceanicData),
    /// P
    Pictorial(BUFRPictoralData),
    /// S
    SurfaceSeaLevel(BUFRSurfaceSeaLevelData),
    /// T
    Text(BUFRTextData),
    /// U
    UpperAir(BUFRUpperAirData),
    /// X
    Other,
}
