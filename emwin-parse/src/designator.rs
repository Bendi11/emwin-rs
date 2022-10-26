
/// Term T2 definition when T1=Analysis
pub enum AnalysisT2 {
    Cyclone,
    Hydrological,
    Thickness,
    Ice,
    Ozone,
    Radar,
    Surface,
    UpperAir,
    WeatherSummary,
    Misc,
}

/// Term T2 definition when T1=ClimaticData
pub enum ClimaticDataT2 {
    Anomaly,
    UpperAirMonthlyMean,
    SurfaceMonthlyMean,
    OceanMonthlyMean,
}

/// Term T2 definition when T1=Forecast
pub enum ForecastT2 {
    AviationGAMETAdvisories,
    UpperWindsTemps,
    AerodomeVTLT12,
    RadiologicalTrajectoryDose,
    Extended,
    Shipping,
    Hydrological,
    UpperAirThickness,
    Iceberg,
    RadioWarningService,
    TropicalCycloneAdvisory,
    Local,
    TemperatureExtreme,
    SpaceWeatherAdvisory,
    Guidance,
    Public,
    OtherShipping,
    AviationRoute,
    Surface,
    AerodomeVTGE12,
    UpperAir,
    VolcanicAshAdvisory,
    WinterSports,
    Misc,
    ShippingArea
}

/// Term T2 definition when T1=Notice
pub enum NoticeT2 {
    Hydrological,
    Marine,
    NuclearEmergencyResponse,
    METNOWIFMA,
    ProductGenerationDelay,
    TESTMSG,
    WarningRelatedOrCancellation,
}

/// Term T2 definition when T1=Surface
pub enum SurfaceT2 {
    AviationRoutineReport,
    RadarReportA,
    RadarReportB,
    RadarReportAB,
    Seismic,
    AtmosphericReport,
    RadiologicalDataReport,
    DCPStationReport,
    IntermediateSynopticHour,
    MainSynopticHour,
    NonstandardSynopticHour,
    OceanographicData,
    SpecialAviationWeatherReport,
    HydrologicalRiverReport,
    DriftingBuoyReport,
    SeaIce,
    SnowDepth,
    LakeIce,
    WaveInformation,
    Misc,
    SeismicWaveformData,
    SeaLevelDeepOceanTsunamiData,
}

/// Term T2 definition when T1=satellite
pub enum SatelliteT2 {
    SatelliteOrbitParameters,
    SatelliteCloudInterpretations,
    SatelliteRemoteUpperAirSoundings,
    ClearRadianceObservations,
    SeaSurfaceTemperatures,
    WindsAndCloudsTemperatures,
    Misc,
}

/// Term T2 definitions when T1=UpperAirData
pub enum UpperT2 {
    AircraftReport,
    UpperLevelPressureTemperatureHumidityWindD,
    UpperLevelPressureTemperatureHumidityWindCD,
    UpperWindB,
    UpperWindC,
    UpperWindAB,
    UpperLevelPressureTemperatureHumidityWindB,
    UpperLevelPressureTemperatureHumidityWindC,
    UpperLevelPressureTemperatureHumidityWindAB,
    RocketsondeReport,
    UpperWindA,
    UpperWindD,
    UpperLevelPressureTemperatureHumidityWindA,
    Misc,
    UpperWindCD,
    UpperLevelPressureTemperatureHumidityWindABCD,
}

/// Term T2 definitions when T1=Warning
pub enum WarningT2 {
    AIRMET,
    TropicalCycloneSIGMET,
    Tsunami,
    Tornado,
    HydrologicalRiverFlood,
    MarineCoastalFlood,
    Other,
    HumanitarianActivities,
    SIGMET,
    TropicalCycloneTyphoonHurricane,
    SevereThunderstorm,
    VolcanicAshCloudsSIGMET,
    WarningsWeatherSummary,
}

/// A data type designator consisting of two alphanumeric characters
pub enum DataTypeDesignator {
    Analysis(AnalysisT2),
    AddressedMessage,
    ClimaticData(ClimaticDataT2),
    GridPointInformation,
    SatelliteImagery,
    Forecast(ForecastT2),
    ObservationalDataBinary,
    ForecastBinary,
    CREX,
    AviationInformationXML,
    Notice(NoticeT2),
    OceanographicInformation,
    PictoralInformationBinary,
    PictoralInformationRegionalBinary,
    SurfaceData(SurfaceT2),
    SatelliteData(SatelliteT2),
    UpperAirData(UpperT2),
    NationalData,
    Warning(WarningT2),
    CommonAlertProtocolMessage
}
