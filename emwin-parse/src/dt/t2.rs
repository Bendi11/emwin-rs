use super::code::CodeForm;


/// Term T2 definition when T1=Analysis
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClimaticDataT2 {
    Anomaly,
    UpperAirMonthlyMean,
    SurfaceMonthlyMean(CodeForm),
    OceanMonthlyMean,
}

/// Term T2 definition when T1=Forecast
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpperT2 {
    AircraftReport(CodeForm),
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

/// Term T2 definitions when T1=GridPointInformation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GRIDT2 {
    RadarData,
    Cloud,
    Vorticity,
    Thickness,
    Precipitation,
    Divergence,
    Height,
    WaveHeight,
    SwellHeight,
    NationalUse,
}

/// Term T2 definitions when T1=ObservationalDataBinaryBUFR or ForecastBinaryBUFR
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFRT2 {
    SatelliteData,
    OceanographicLimnographic,
    Pictorial,
    SurfaceSeaLevel,
    Text,
    UpperAir,
    Other,
}

/// Term T2 definitions when T1=OceanographicInformation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OceanographicT2 {
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

/// Term T2 definitions when T1=SatelliteImagery
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SatelliteImageryT2 {
    CloudTopTemperature,
    Fog,
    Infared,
    SurfaceTemperature,
}

/// Term T2 definitions when T1=PictoralInformationBinary or PictoralInformationRegionalBinary
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PictoralInformationT2 {
    RadarData,
    Cloud,
    ClearAirTurbulence,
    Thickness,
    Precipitation,
    AerologicalDiagrams,
    SignificantWeather,
    Height,
    IceFlow,
    WaveHeight,
    SwellHeight,
    PlainLanguage,
    NationalUse,
    Radiation,
    VerticalVelocity,
    Pressure,
    WetBulbPotentialTemperature,
    RelativeHumidity,
    SnowCover,
    Temperature,
    EastwardWindComponent,
    NorthwardWindComponent,
    Wind,
    LiftedIndex,
    ObservationalPlottedChart,
}

/// Term T2 definitions when T1=AviationInformationXML
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AviationInformationXMLT2 {
    AviationRoutineReportMETAR,
    AerodomeForecastTAFVTLT12,
    TropicalCycloneAdvisory,
    SpaceWeatherAdvisory,
    SpecialAviationWeatherReportSPECI,
    AviationGeneralWarningSIGMET,
    AerodomeForecastTAFVTGE12,
    VolcanicAshAdvisory,
    AviationVolcanicAshWarningSIGMET,
    AIRMET,
    AviationTropicalCycloneWarningSIGMET,
}


