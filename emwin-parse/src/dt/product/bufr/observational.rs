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
            },
            area: GeographicalAreaDesignator::try_from(value.t2)?,
        })
    }
}
