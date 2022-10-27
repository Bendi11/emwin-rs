use std::str::FromStr;
pub use self::t2::*;
use self::{code::CodeForm, area::{AreaCode, AreaCodeParseError, GeographicalAreaDesignator, ReferenceTimeDesignator}, bufr::BUFRDataType};

pub mod t2;
pub mod code;
pub mod area;
pub mod bufr;
#[cfg(test)]
mod test;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnalysisDataDesignator {
    pub subtype: AnalysisT2,
    pub area: AreaCode,
}

/// The type of message an [AddressedMessage] is
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddressedMessageType {
    Administrative,
    Service,
    GTSRequest,
    RequestToDB,
    GTSOrDBResponse,
}

/// format @ WMO-No. 386 p.103 attachment II-6
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AddressedMessage {
    /// T2
    pub binary: bool,
    /// A1A2
    pub kind: AddressedMessageType,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridPointInformation {
    /// T2
    pub subtype: GRIDT2,
    /// A1
    pub area: GeographicalAreaDesignator,
    /// A2
    pub time: ReferenceTimeDesignator,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PictoralInformation {
    /// T2
    pub subtype: PictoralInformationT2,
    /// A1
    pub area: GeographicalAreaDesignator,
    /// A2
    pub time: ReferenceTimeDesignator,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RegionalPictoralInformation {
    /// T2
    pub subtype: PictoralInformationT2,
    /// A1
    pub area: GeographicalAreaDesignator,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SatelliteData {
    /// T2
    pub subtype: SatelliteT2,
    /// A1
    pub area: GeographicalAreaDesignator,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObservationalDataBinary {
    /// T2
    pub subtype: BUFRDataType,
    /// A2
    pub area: GeographicalAreaDesignator,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ForecastDataBinary {
    /// T2
    pub subtype: BUFRDataType,
    /// A2
    pub time: ReferenceTimeDesignator,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommonAlertProtocolMessage {
    pub time: ReferenceTimeDesignator,
}

/// A data type designator consisting of two alphanumeric characters
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataTypeDesignator {
    /// A
    Analysis(AnalysisDataDesignator),
    /// B
    AddressedMessage(AddressedMessage),
    /// C
    ClimaticData(ClimaticDataT2),
    /// D, G, H, O
    GridPointInformation(GridPointInformation),
    /// E
    SatelliteImagery(SatelliteImageryT2),
    /// F
    Forecast(ForecastT2),
    /// I
    ObservationalDataBinaryBUFR(ObservationalDataBinary),
    /// J
    ForecastBinaryBUFR(ForecastDataBinary),
    /// K
    CREX,
    /// L
    AviationInformationXML(AviationInformationXMLT2),
    /// N
    Notice(NoticeT2),
    /// O
    OceanographicInformation(OceanographicT2),
    /// P
    PictoralInformationBinary(PictoralInformation),
    /// Q
    PictoralInformationRegionalBinary(RegionalPictoralInformation),
    /// S
    SurfaceData(SurfaceT2),
    /// T
    SatelliteData(SatelliteT2),
    /// U
    UpperAirData(UpperT2),
    /// V
    NationalData,
    /// W
    Warning(WarningT2),
    /// X
    CommonAlertProtocolMessage(CommonAlertProtocolMessage),
}

impl FromStr for DataTypeDesignator {
    type Err = DataTypeDesignatorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 4 {
            return Err(DataTypeDesignatorParseError::Length)
        }
        
        let mut iter = s.chars();
        
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();

        let a1 = iter.next().unwrap();
        let a2 = iter.next().unwrap();
        drop(iter);

        Ok(match first {
            'A' => Self::Analysis(AnalysisDataDesignator {
                subtype: match second {
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
                },
                area: AreaCode::try_from((a1, a2))?,
            }),
            'B' => Self::AddressedMessage(AddressedMessage {
                binary: match second {
                    'M' => false,
                    'I' => true,
                    other => return Err(DataTypeDesignatorParseError::UnrecognizedT2('B', other)),
                },
                kind: match (a1, a2) {
                    ('A', 'A') => AddressedMessageType::Administrative,
                    ('B', 'B') => AddressedMessageType::Service,
                    ('R', 'R') => AddressedMessageType::GTSRequest,
                    ('R', 'Q') => AddressedMessageType::RequestToDB,
                    ('D', 'A') => AddressedMessageType::GTSOrDBResponse,
                    _ => return Err(DataTypeDesignatorParseError::UnrecognizedA2(first, second, a1, a2)),
                }
            }),
            'C' => Self::ClimaticData(match second {
                'A' => ClimaticDataT2::Anomaly,
                'E' => ClimaticDataT2::UpperAirMonthlyMean,
                'H' => ClimaticDataT2::SurfaceMonthlyMean(CodeForm::CLIMATSHIP),
                'O' => ClimaticDataT2::OceanMonthlyMean,
                'S' => ClimaticDataT2::SurfaceMonthlyMean(CodeForm::CLIMAT),
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2('C', other)),
            }),
            'F' => Self::Forecast(match second {
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
            'N' => Self::Notice(match second {
                'G' => NoticeT2::Hydrological,
                'H' => NoticeT2::Marine,
                'N' => NoticeT2::NuclearEmergencyResponse,
                'O' => NoticeT2::METNOWIFMA,
                'P' => NoticeT2::ProductGenerationDelay,
                'T' => NoticeT2::TESTMSG,
                'W' => NoticeT2::WarningRelatedOrCancellation,
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2('N', other)),
            }),
            'S' => Self::SurfaceData(match second {
                'A' => SurfaceT2::AviationRoutineReport,
                'B' => SurfaceT2::RadarReportA,
                'C' => SurfaceT2::RadarReportB,
                'D' => SurfaceT2::RadarReportAB,
                'E' => SurfaceT2::Seismic,
                'F' => SurfaceT2::AtmosphericReport,
                'G' => SurfaceT2::RadiologicalDataReport,
                'H' => SurfaceT2::DCPStationReport,
                'I' => SurfaceT2::IntermediateSynopticHour,
                'M' => SurfaceT2::MainSynopticHour,
                'N' => SurfaceT2::NonstandardSynopticHour,
                'O' => SurfaceT2::OceanographicData,
                'P' => SurfaceT2::SpecialAviationWeatherReport,
                'R' => SurfaceT2::HydrologicalRiverReport,
                'S' => SurfaceT2::DriftingBuoyReport,
                'T' => SurfaceT2::SeaIce,
                'U' => SurfaceT2::SnowDepth,
                'V' => SurfaceT2::LakeIce,
                'W' => SurfaceT2::WaveInformation,
                'X' => SurfaceT2::Misc,
                'Y' => SurfaceT2::SeismicWaveformData,
                'Z' => SurfaceT2::SeaLevelDeepOceanTsunamiData,
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2('S', other)),
            }),
            'T' => Self::SatelliteData(match second {
                'B' => SatelliteT2::SatelliteOrbitParameters,
                'C' => SatelliteT2::SatelliteCloudInterpretations,
                'H' => SatelliteT2::SatelliteRemoteUpperAirSoundings,
                'R' => SatelliteT2::ClearRadianceObservations,
                'T' => SatelliteT2::SeaSurfaceTemperatures,
                'W' => SatelliteT2::WindsAndCloudsTemperatures,
                'X' => SatelliteT2::Misc,
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2('T', other)),
            }),
            'U' => Self::UpperAirData(match second {
                'A' => UpperT2::AircraftReport(CodeForm::ICAO),
                'D' => UpperT2::AircraftReport(CodeForm::AMDAR),
                'E' => UpperT2::UpperLevelPressureTemperatureHumidityWindD,
                'F' => UpperT2::UpperLevelPressureTemperatureHumidityWindCD,
                'G' => UpperT2::UpperWindB,
                'H' => UpperT2::UpperWindC,
                'I' => UpperT2::UpperWindAB,
                'K' => UpperT2::UpperLevelPressureTemperatureHumidityWindB,
                'L' => UpperT2::UpperLevelPressureTemperatureHumidityWindC,
                'M' => UpperT2::UpperLevelPressureTemperatureHumidityWindAB,
                'N' => UpperT2::RocketsondeReport,
                'P' => UpperT2::UpperWindA,
                'Q' => UpperT2::UpperWindD,
                'R' => UpperT2::AircraftReport(CodeForm::RECCO),
                'S' => UpperT2::UpperLevelPressureTemperatureHumidityWindA,
                'T' => UpperT2::AircraftReport(CodeForm::CODAR),
                'X' => UpperT2::Misc,
                'Y' => UpperT2::UpperWindCD,
                'Z' => UpperT2::UpperLevelPressureTemperatureHumidityWindABCD,
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2('U', other)),
            }),
            'W' => Self::Warning(match second {
                'A' => WarningT2::AIRMET,
                'C' => WarningT2::TropicalCycloneSIGMET,
                'E' => WarningT2::Tsunami,
                'F' => WarningT2::Tornado,
                'G' => WarningT2::HydrologicalRiverFlood,
                'H' => WarningT2::MarineCoastalFlood,
                'O' => WarningT2::Other,
                'R' => WarningT2::HumanitarianActivities,
                'S' => WarningT2::SIGMET,
                'T' => WarningT2::TropicalCycloneTyphoonHurricane,
                'U' => WarningT2::SevereThunderstorm,
                'V' => WarningT2::VolcanicAshCloudsSIGMET,
                'W' => WarningT2::WarningsWeatherSummary,
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2('W', other)),
            }),
            'D' | 'G' | 'H' | 'Y' => Self::GridPointInformation(match second {
                'A' => GRIDT2::RadarData,
                'B' => GRIDT2::Cloud,
                'C' => GRIDT2::Vorticity,
                'D' => GRIDT2::Thickness,
                'E' => GRIDT2::Precipitation,
                'G' => GRIDT2::Divergence,
                'H' => GRIDT2::Height,
                'J' => GRIDT2::WaveHeight,
                'K' => GRIDT2::SwellHeight,
                'M' => GRIDT2::NationalUse,
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2(first, other)),
            }),
            'I' | 'J' => {
                let second = match second {
                    'N' => BUFRDataType::SatelliteData,
                    'O' => BUFRDataType::OceanographicLimnographic,
                    'P' => BUFRDataType::Pictorial,
                    'S' => BUFRDataType::SurfaceSeaLevel,
                    'T' => BUFRDataType::Text,
                    'U' => BUFRDataType::UpperAir,
                    'X' => BUFRDataType::Other,
                    other => return Err(DataTypeDesignatorParseError::UnrecognizedT2(first, other)),
                };
                match first {
                    'I' => Self::ObservationalDataBinaryBUFR(second),
                    'J' => Self::ForecastBinaryBUFR(second),
                    _ => unreachable!(),
                }
            },
            'O' => Self::OceanographicInformation(match second {
                'D' => OceanographicT2::Depth,
                'E' => OceanographicT2::IceConcentration,
                'F' => OceanographicT2::IceThickness,
                'G' => OceanographicT2::IceDrift,
                'H' => OceanographicT2::IceGrowth,
                'I' => OceanographicT2::IceConvergenceDivergence,
                'Q' => OceanographicT2::TemperatureAnomaly,
                'R' => OceanographicT2::DepthAnomaly,
                'S' => OceanographicT2::Salinity,
                'T' => OceanographicT2::Temperature,
                'U' | 'V' => OceanographicT2::CurrentComponent,
                'W' => OceanographicT2::TemperatureWarming,
                'X' => OceanographicT2::Mixed,
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2(first, other)),
            }),
            'E' => Self::SatelliteImagery(match second {
                'C' => SatelliteImageryT2::CloudTopTemperature,
                'F' => SatelliteImageryT2::Fog,
                'I' => SatelliteImageryT2::Infared,
                'S' => SatelliteImageryT2::SurfaceTemperature,
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2(first, other)),
            }),
            'P' | 'Q' => {
                let second = match second {
                    'A' => PictoralInformationT2::RadarData,
                    'B' => PictoralInformationT2::Cloud,
                    'C' => PictoralInformationT2::ClearAirTurbulence,
                    'D' => PictoralInformationT2::Thickness,
                    'E' => PictoralInformationT2::Precipitation,
                    'F' => PictoralInformationT2::AerologicalDiagrams,
                    'G' => PictoralInformationT2::SignificantWeather,
                    'H' => PictoralInformationT2::Height,
                    'I' => PictoralInformationT2::IceFlow,
                    'J' => PictoralInformationT2::WaveHeight,
                    'K' => PictoralInformationT2::SwellHeight,
                    'L' => PictoralInformationT2::PlainLanguage,
                    'M' => PictoralInformationT2::NationalUse,
                    'N' => PictoralInformationT2::Radiation,
                    'O' => PictoralInformationT2::VerticalVelocity,
                    'P' => PictoralInformationT2::Pressure,
                    'Q' => PictoralInformationT2::WetBulbPotentialTemperature,
                    'R' => PictoralInformationT2::RelativeHumidity,
                    'S' => PictoralInformationT2::SnowCover,
                    'T' => PictoralInformationT2::Temperature,
                    'U' => PictoralInformationT2::EastwardWindComponent,
                    'V' => PictoralInformationT2::NorthwardWindComponent,
                    'W' => PictoralInformationT2::Wind,
                    'X' => PictoralInformationT2::LiftedIndex,
                    'Y' => PictoralInformationT2::ObservationalPlottedChart,
                    other => return Err(DataTypeDesignatorParseError::UnrecognizedT2(first, other)),
                };
                match first {
                    'P' => Self::PictoralInformationBinary(second),
                    'Q' => Self::PictoralInformationRegionalBinary(second),
                    _ => unreachable!(),
                }
            },
            'L' => Self::AviationInformationXML(match second {
                'A' => AviationInformationXMLT2::AviationRoutineReportMETAR,
                'C' => AviationInformationXMLT2::AerodomeForecastTAFVTLT12,
                'K' => AviationInformationXMLT2::TropicalCycloneAdvisory,
                'N' => AviationInformationXMLT2::SpaceWeatherAdvisory,
                'P' => AviationInformationXMLT2::SpecialAviationWeatherReportSPECI,
                'S' => AviationInformationXMLT2::AviationGeneralWarningSIGMET,
                'T' => AviationInformationXMLT2::AerodomeForecastTAFVTGE12,
                'U' => AviationInformationXMLT2::VolcanicAshAdvisory,
                'V' => AviationInformationXMLT2::AviationVolcanicAshWarningSIGMET,
                'W' => AviationInformationXMLT2::AIRMET,
                'Y' => AviationInformationXMLT2::AviationTropicalCycloneWarningSIGMET,
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2(first, other)),
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
    #[error("Unrecognized data type designator term 3 {0}{1}{2}")]
    UnrecognizedA1(char, char, char),
    #[error("Unrecognized data type designator term 4 {0}{1}{2}{3}")]
    UnrecognizedA2(char, char, char, char),
    #[error("Invalid area code: {0}")]
    InvalidAreaCode(#[from] AreaCodeParseError),
}


impl DataTypeDesignator {

}
