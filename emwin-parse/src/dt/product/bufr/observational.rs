use crate::dt::{area::GeographicalAreaDesignator, UnparsedProductIdentifier, DataTypeDesignatorParseError};

/// I
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObservationalDataBinary {
    /// T2
    pub subtype: ObservationalDataBinaryBUFRSubType,
    /// A2
    pub area: GeographicalAreaDesignator,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObservationalBUFRSatellite {
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
pub enum ObservationalBUFROceanic {
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
pub enum ObservationalBUFRPictoral {
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
pub enum ObservationalBUFRSurfaceSeaLevel {
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
pub enum ObservationalBUFRText {
    AdministrativeMessage,
    ServiceMessage,
    RequestData,
    OtherText,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObservationalBUFRUpperAir {
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
    SatelliteData(ObservationalBUFRSatellite),
    /// O
    OceanographicLimnographic(ObservationalBUFROceanic),
    /// P
    Pictorial(ObservationalBUFRPictoral),
    /// S
    SurfaceSeaLevel(ObservationalBUFRSurfaceSeaLevel),
    /// T
    Text(ObservationalBUFRText),
    /// U
    UpperAir(ObservationalBUFRUpperAir),
    /// X
    Other,
}

impl TryFrom<UnparsedProductIdentifier> for ObservationalDataBinary {
    type Error = DataTypeDesignatorParseError;
    fn try_from(value: UnparsedProductIdentifier) -> Result<Self, Self::Error> {
        Ok(Self {
            subtype: match value.t2 {
                'N' => ObservationalDataBinaryBUFRSubType::SatelliteData(match value.a1 {
                    'A' => ObservationalBUFRSatellite::AMSUA,
                    'B' => ObservationalBUFRSatellite::AMSUB,
                    'C' => ObservationalBUFRSatellite::CrIS,
                    'H' => ObservationalBUFRSatellite::HIRS,
                    'I' => ObservationalBUFRSatellite::IRAS,
                    'J' => ObservationalBUFRSatellite::HIRAS,
                    'K' => ObservationalBUFRSatellite::MWHS,
                    'M' => ObservationalBUFRSatellite::MHS,
                    'Q' => ObservationalBUFRSatellite::IASI,
                    'S' => ObservationalBUFRSatellite::ATMS,
                    'T' => ObservationalBUFRSatellite::MWTS,
                    other => return Err(DataTypeDesignatorParseError::UnrecognizedA1(value.t1, value.t2, other)),
                }),
                'O' => ObservationalDataBinaryBUFRSubType::OceanographicLimnographic(match value.a1 {
                    'B' => ObservationalBUFROceanic::BuoyObservations,
                    'I' => ObservationalBUFROceanic::SeaIce,
                    'P' => ObservationalBUFROceanic::SubsurfaceProfilingFloats,
                    'R' => ObservationalBUFROceanic::SeasurfaceObservations,
                    'S' => ObservationalBUFROceanic::SeasurfaceAndBelowSoundings,
                    'T' => ObservationalBUFROceanic::SeasurfaceTemperature,
                    'W' => ObservationalBUFROceanic::SeasurfaceWaves,
                    'X' => ObservationalBUFROceanic::OtherSeaEnvironmental,
                    'Z' => ObservationalBUFROceanic::DeepOceanTsunameter,
                    other => return Err(DataTypeDesignatorParseError::UnrecognizedA1(value.t1, value.t2, other)),
                }),
                'P' => ObservationalDataBinaryBUFRSubType::Pictorial(match value.a1 {
                    'C' => ObservationalBUFRPictoral::RadarCompositeImagery,
                    'I' => ObservationalBUFRPictoral::SatelliteImagery,
                    'R' => ObservationalBUFRPictoral::RadarImagery,
                    'X' => ObservationalBUFRPictoral::NotDefined,
                    other => return Err(DataTypeDesignatorParseError::UnrecognizedA1(value.t1, value.t2, other)),
                }),
                'S' => ObservationalDataBinaryBUFRSubType::SurfaceSeaLevel(match value.a1 {
                    'A' if (30..=59).contains(&value.ii) => ObservationalBUFRSurfaceSeaLevel::NMinuteObservationLandStation,
                    'A' => ObservationalBUFRSurfaceSeaLevel::RoutinelyScheduledLandStation,
                    'B' => ObservationalBUFRSurfaceSeaLevel::RadarReportAB,
                    'C' if (46..=59).contains(&value.ii) => ObservationalBUFRSurfaceSeaLevel::ClimaticObservationMarine,
                    'C' if value.ii == 60 => ObservationalBUFRSurfaceSeaLevel::ClimaticObservationMonthly,
                    'C' => ObservationalBUFRSurfaceSeaLevel::ClimaticObservationLandStation,
                    'D' => ObservationalBUFRSurfaceSeaLevel::RadiologicalObservation,
                    'E' => ObservationalBUFRSurfaceSeaLevel::SurfaceOzoneMeasurement,
                    'F' => ObservationalBUFRSurfaceSeaLevel::AtmosphericsSource,
                    'I' if (46..=59).contains(&value.ii) => 
                        ObservationalBUFRSurfaceSeaLevel::IntermediateSynopticObservation(LandStation::Mobile),
                    'I' => ObservationalBUFRSurfaceSeaLevel::IntermediateSynopticObservation(LandStation::Fixed),
                    'M' if (46..=59).contains(&value.ii) =>
                        ObservationalBUFRSurfaceSeaLevel::MainSynopticObservation(LandStation::Mobile),
                    'M' => ObservationalBUFRSurfaceSeaLevel::MainSynopticObservation(LandStation::Fixed),
                    'N' if (46..=59).contains(&value.ii) =>
                        ObservationalBUFRSurfaceSeaLevel::SynopticObservationNonStandardTime(LandStation::Mobile),
                    'N' => ObservationalBUFRSurfaceSeaLevel::SynopticObservationNonStandardTime(LandStation::Fixed),
                    'R' => ObservationalBUFRSurfaceSeaLevel::Hydrologic,
                    'S' if (20..=39).contains(&value.ii) => ObservationalBUFRSurfaceSeaLevel::OneHourObservationMarineStation,
                    'S' if (40..=59).contains(&value.ii) => ObservationalBUFRSurfaceSeaLevel::NMinuteObservationMarineStation,
                    'S' => ObservationalBUFRSurfaceSeaLevel::SynopticObservationMarineStation,
                    'T' if (20..=39).contains(&value.ii) =>
                        ObservationalBUFRSurfaceSeaLevel::ObservedWaterLevelTimeSeries,
                    'T' => ObservationalBUFRSurfaceSeaLevel::TideGaugeObservation,
                    'V' => ObservationalBUFRSurfaceSeaLevel::SpecialAeronauticalObservation,
                    'W' => ObservationalBUFRSurfaceSeaLevel::AviationRoutineWeatherObservation,
                    'X' => ObservationalBUFRSurfaceSeaLevel::OtherSurfaceData,
                    other => return Err(DataTypeDesignatorParseError::UnrecognizedA1(value.t1, value.t2, other)),
                }),
                'T' => ObservationalDataBinaryBUFRSubType::Text(match value.a1 {
                    'A' => ObservationalBUFRText::AdministrativeMessage,
                    'B' => ObservationalBUFRText::ServiceMessage,
                    'R' => ObservationalBUFRText::RequestData,
                    'X' => ObservationalBUFRText::OtherText,
                    other => return Err(DataTypeDesignatorParseError::UnrecognizedA1(value.t1, value.t2, other)),
                }),
                'U' => ObservationalDataBinaryBUFRSubType::UpperAir(match value.a1 {
                    'A' => ObservationalBUFRUpperAir::SingleLevelAircraftReportAuto,
                    'B' => ObservationalBUFRUpperAir::SingleLevelBalloonReport,
                    'C' => ObservationalBUFRUpperAir::SingleLevelSatelliteDerivedReport,
                    'D' => ObservationalBUFRUpperAir::Dropsonde,
                    'E' => ObservationalBUFRUpperAir::OzoneVerticalSounding,
                    'I' => ObservationalBUFRUpperAir::DispersalTransportAnalysis,
                    'J' if (20..=39).contains(&value.ii) => ObservationalBUFRUpperAir::UpperWindEntireSounding(LandStation::Mobile),
                    'J' if (40..=59).contains(&value.ii) => ObservationalBUFRUpperAir::UpperWindEntireSoundingMarine,
                    'J' => ObservationalBUFRUpperAir::UpperWindEntireSounding(LandStation::Fixed),
                    'K' if (20..=39).contains(&value.ii) => ObservationalBUFRUpperAir::RadioSoundingUpTo100HPA(LandStation::Mobile),
                    'K' if (40..=59).contains(&value.ii) => ObservationalBUFRUpperAir::RadioSoundingUpTo100HPAMarine,
                    'K' => ObservationalBUFRUpperAir::RadioSoundingUpTo100HPA(LandStation::Fixed),
                    'L' => ObservationalBUFRUpperAir::TotalOzone,
                    'M' => ObservationalBUFRUpperAir::ModelDerivedSondes,
                    'N' => ObservationalBUFRUpperAir::Rocketsondes,
                    'O' => ObservationalBUFRUpperAir::AircraftAscendDescendProfile,
                    'P' => ObservationalBUFRUpperAir::Profiler,
                    'Q' => ObservationalBUFRUpperAir::RASSTemperatureProfiler,
                    'R' => ObservationalBUFRUpperAir::Radiance,
                    'S' if (20..=39).contains(&value.ii) => ObservationalBUFRUpperAir::RadiosondesSounding(LandStation::Mobile),
                    'S' if (40..=59).contains(&value.ii) => ObservationalBUFRUpperAir::RadiosondesSoundingMarine,
                    'S' => ObservationalBUFRUpperAir::RadiosondesSounding(LandStation::Fixed),
                    'T' => ObservationalBUFRUpperAir::SatelliteDerivedSondes,
                    'U' => ObservationalBUFRUpperAir::MonthlyStatisticsDataMarine,
                    'W' if (20..=39).contains(&value.ii) => ObservationalBUFRUpperAir::UpperWindUpTo100HPA(LandStation::Mobile),
                    'W' if (40..=59).contains(&value.ii) => ObservationalBUFRUpperAir::UpperWindMarineUpTo100HPAMarine,
                    'W' => ObservationalBUFRUpperAir::UpperWindUpTo100HPA(LandStation::Fixed),
                    'X' => ObservationalBUFRUpperAir::OtherUpperAirReport,
                    other => return Err(DataTypeDesignatorParseError::UnrecognizedA1(value.t1, value.t2, other)),
                }),
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2(value.t1, value.t2)),
            },
            area: GeographicalAreaDesignator::try_from(value.t2)?,
        })
    }
}
