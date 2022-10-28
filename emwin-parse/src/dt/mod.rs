use std::{str::FromStr, num::ParseIntError};
pub use self::product::*;
use self::{
    code::CodeForm,
    area::{AreaCode, AreaCodeParseError, GeographicalAreaDesignator, ReferenceTimeDesignator, GeographicalAreaDesignatorParseError, ReferenceTimeDesignatorParseError}, analysis::Analysis, addressedmsg::AddressedMessage, climatic::ClimaticData, gridpoint::GridPointInformation, satelliteimagery::SatelliteImagery, forecast::Forecast, bufr::{observational::ObservationalDataBinary, forecast::ForecastDataBinary}, aviationxml::AviationInformationXML, notice::Notice, oceanographic::OceanographicInformation, pictoral::PictoralInformation, pictoral_regional::RegionalPictoralInformation, surface::Surface, satellite::SatelliteData, upperair::UpperAirData, warning::Warning, cap::CommonAlertProtocolMessage,
};

pub mod product;

pub mod code;
pub mod area;
#[cfg(test)]
mod test;

/// A product identifier containing only raw characters used for parsing product identifiers
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnparsedProductIdentifier {
    pub t1: char,
    pub t2: char,
    pub a1: char,
    pub a2: char,
    pub ii: u8,
}

/// A data type designator consisting of two alphanumeric characters
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataTypeDesignator {
    /// A
    Analysis(Analysis),
    /// B
    AddressedMessage(AddressedMessage),
    /// C
    ClimaticData(ClimaticData),
    /// D, G, H, O
    GridPointInformation(GridPointInformation),
    /// E
    SatelliteImagery(SatelliteImagery),
    /// F
    Forecast(Forecast),
    /// I
    ObservationalDataBinaryBUFR(ObservationalDataBinary),
    /// J
    ForecastBinaryBUFR(ForecastDataBinary),
    /// K
    CREX,
    /// L
    AviationInformationXML(AviationInformationXML),
    /// N
    Notice(Notice),
    /// O
    OceanographicInformation(OceanographicInformation),
    /// P
    PictoralInformationBinary(PictoralInformation),
    /// Q
    PictoralInformationRegionalBinary(RegionalPictoralInformation),
    /// S
    SurfaceData(Surface),
    /// T
    SatelliteData(SatelliteData),
    /// U
    UpperAirData(UpperAirData),
    /// V
    NationalData,
    /// W
    Warning(Warning),
    /// X
    CommonAlertProtocolMessage(CommonAlertProtocolMessage),
}

impl FromStr for DataTypeDesignator {
    type Err = DataTypeDesignatorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 6 {
            return Err(DataTypeDesignatorParseError::Length)
        }
        
        let mut iter = s.chars();

        let ident = UnparsedProductIdentifier {
            t1: iter.next().unwrap(),
            t2: iter.next().unwrap(),
            a1: iter.next().unwrap(),
            a2: iter.next().unwrap(),
            ii: s[4..6].parse()?,
        };

        Ok(match first {
            'A' => Self::Analysis(Analysis::try_from(ident)),
            'B' => Self::AddressedMessage(AddressedMessage::try_from(ident)),
            'C' => Self::ClimaticData(ClimaticData::try_from(ident)),
            'F' => Self::Forecast(Forecast::try_from(ident)),
            'N' => Self::Notice(Notice::try_from(ident)),
            'S' => Self::SurfaceData(SurfaceData::try_from(ident)),
            'T' => Self::SatelliteData(),
            'U' => Self::UpperAirData(),
            'W' => Self::Warning(),
            'D' | 'G' | 'H' | 'Y' => Self::GridPointInformation(),
            'I' => Self::ObservationalDataBinaryBUFR(),
            'J' => Self::ForecastBinaryBUFR(),
            'O' => Self::OceanographicInformation(),
            'E' => Self::SatelliteImagery(),
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
    #[error("Invalid numeral ii: {0}")]
    InvalidNumeral(#[from] ParseIntError),
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
    #[error("error parsing geo area designator: {0}")]
    InvalidGeographicalAreaDesignator(#[from] GeographicalAreaDesignatorParseError),
    #[error("error parsing reference time designator: {0}")]
    InvalidReferenceTimeDesignator(#[from] ReferenceTimeDesignatorParseError),
}


impl DataTypeDesignator {

}
