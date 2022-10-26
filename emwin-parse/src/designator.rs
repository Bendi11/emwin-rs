use std::str::FromStr;

use crate::code::CodeForm;


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
    IceCondensation,
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
    PlainLanguage,
    NationalUse,
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

/// A data type designator consisting of two alphanumeric characters
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataTypeDesignator {
    Analysis(AnalysisT2),
    AddressedMessage,
    ClimaticData(ClimaticDataT2),
    GridPointInformation(GRIDT2),
    SatelliteImagery(SatelliteImageryT2),
    Forecast(ForecastT2),
    ObservationalDataBinaryBUFR(BUFRT2),
    ForecastBinaryBUFR(BUFRT2),
    CREX,
    AviationInformationXML(AviationInformationXMLT2),
    Notice(NoticeT2),
    OceanographicInformation(OceanographicT2),
    PictoralInformationBinary(PictoralInformationT2),
    PictoralInformationRegionalBinary(PictoralInformationT2),
    SurfaceData(SurfaceT2),
    SatelliteData(SatelliteT2),
    UpperAirData(UpperT2),
    NationalData,
    Warning(WarningT2),
    CommonAlertProtocolMessage
}


impl FromStr for DataTypeDesignator {
    type Err = DataTypeDesignatorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(DataTypeDesignatorParseError::Length)
        }
        
        let mut iter = s.chars();

        Ok(match iter.next().unwrap() {
            'A' => Self::Analysis(match iter.next().unwrap() {
                'C' => AnalysisT2::Cyclone,
                'G' => AnalysisT2::Hydrological,
                'H' => AnalysisT2::Thickness,
                'I' => AnalysisT2::Ice,
                'O' => AnalysisT2::Ozone,
                'R' => AnalysisT2::Radar,
                'S' => AnalysisT2::Surface,
                'U' => AnalysisT2::UpperAir,
                'W' => AnalysisT2::WeatherSummary,
                'X' => AnalysisT2::Misc,
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2('A', other)),
            }),
            'C' => Self::ClimaticData(match iter.next().unwrap() {
                'A' => ClimaticDataT2::Anomaly,
                'E' => ClimaticDataT2::UpperAirMonthlyMean,
                'H' => ClimaticDataT2::SurfaceMonthlyMean(CodeForm::CLIMATSHIP),
                'O' => ClimaticDataT2::OceanMonthlyMean,
                'S' => ClimaticDataT2::SurfaceMonthlyMean(CodeForm::CLIMAT),
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2('C', other)),
            }),
            'F' => Self::Forecast(match iter.next().unwrap() {
                'A' => ForecastT2::AviationGAMETAdvisories,
                'B' => ForecastT2::UpperWindsTemps,
                'C' => ForecastT2::AerodomeVTLT12,
                'D' => ForecastT2::RadiologicalTrajectoryDose,
                'E' => ForecastT2::Extended,
                'F' => ForecastT2::Shipping,
                'G' => ForecastT2::Hydrological,
                'H' => ForecastT2::UpperAirThickness,
                'I' => ForecastT2::Iceberg,
                'J' => ForecastT2::RadioWarningService,
                'K' => ForecastT2::TropicalCycloneAdvisory,
                'L' => ForecastT2::Local,
                'M' => ForecastT2::TemperatureExtreme,
                'N' => ForecastT2::SpaceWeatherAdvisory,
                'O' => ForecastT2::Guidance,
                'P' => ForecastT2::Public,
                'Q' => ForecastT2::OtherShipping,
                'R' => ForecastT2::AviationRoute,
                'S' => ForecastT2::Surface,
                'T' => ForecastT2::AerodomeVTGE12,
                'U' => ForecastT2::UpperAir,
                'V' => ForecastT2::VolcanicAshAdvisory,
                'W' => ForecastT2::WinterSports,
                'X' => ForecastT2::Misc,
                'Z' => ForecastT2::ShippingArea,
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2('F', other)),
            }),
        
            other => return Err(DataTypeDesignatorParseError::UnrecognizedT1(other)),
        })
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum DataTypeDesignatorParseError {
    #[error("Data type designator does not contain two characters")]
    Length,
    #[error("Unrecognized data type designator term 1 {0}")]
    UnrecognizedT1(char),
    #[error("Unrecognized data type designator term 2 {0}{1}")]
    UnrecognizedT2(char, char),
}
