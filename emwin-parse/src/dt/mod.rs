use std::{str::FromStr, num::ParseIntError};
pub use self::product::*;
use self::{
    area::{AreaCodeParseError, GeographicalAreaDesignatorParseError, ReferenceTimeDesignatorParseError}, analysis::Analysis, addressedmsg::AddressedMessage, climatic::ClimaticData, gridpoint::GridPointInformation, satelliteimagery::SatelliteImagery, forecast::Forecast, bufr::{observational::ObservationalDataBinary, forecast::ForecastDataBinary}, aviationxml::AviationInformationXML, notice::Notice, oceanographic::OceanographicInformation, pictoral::PictoralInformation, pictoral_regional::RegionalPictoralInformation, surface::SurfaceData, satellite::SatelliteData, upperair::UpperAirData, warning::Warning, cap::CommonAlertProtocolMessage, level::{InvalidAirLevelDesignator, InvalidSeaLevelDesignator},
};

pub mod product;

pub mod code;
pub mod area;
pub mod level;
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
    CREX(CREX),
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
    SurfaceData(SurfaceData),
    /// T
    SatelliteData(SatelliteData),
    /// U
    UpperAirData(UpperAirData),
    /// V
    National(National),
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

        Ok(match ident.t1 {
            'A' => Self::Analysis(Analysis::try_from(ident)?),
            'B' => Self::AddressedMessage(AddressedMessage::try_from(ident)?),
            'C' => Self::ClimaticData(ClimaticData::try_from(ident)?),
            'F' => Self::Forecast(Forecast::try_from(ident)?),
            'N' => Self::Notice(Notice::try_from(ident)?),
            'S' => Self::SurfaceData(SurfaceData::try_from(ident)?),
            'T' => Self::SatelliteData(SatelliteData::try_from(ident)?),
            'U' => Self::UpperAirData(UpperAirData::try_from(ident)?),
            'W' => Self::Warning(Warning::try_from(ident)?),
            'D' | 'G' | 'H' | 'Y' => Self::GridPointInformation(GridPointInformation::try_from(ident)?),
            'I' => Self::ObservationalDataBinaryBUFR(ObservationalDataBinary::try_from(ident)?),
            'J' => Self::ForecastBinaryBUFR(ForecastDataBinary::try_from(ident)?),
            'O' => Self::OceanographicInformation(OceanographicInformation::try_from(ident)?),
            'E' => Self::SatelliteImagery(SatelliteImagery::try_from(ident)?),
            'P' => Self::PictoralInformationBinary(PictoralInformation::try_from(ident)?),
            'Q' => Self::PictoralInformationRegionalBinary(RegionalPictoralInformation::try_from(ident)?),
            'L' => Self::AviationInformationXML(AviationInformationXML::try_from(ident)?),
            'V' => Self::National(National::try_from(ident)?),
            'K' => Self::CREX(CREX::try_from(ident)?),
            'X' => Self::CommonAlertProtocolMessage(CommonAlertProtocolMessage::try_from(ident)?),
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
    #[error("error parsing air level designator: {0}")]
    InvalidAirLevel(#[from] InvalidAirLevelDesignator),
    #[error("error parsing sea level designator: {0}")]
    InvalidSeaLevel(#[from] InvalidSeaLevelDesignator),
}


impl DataTypeDesignator {

}
