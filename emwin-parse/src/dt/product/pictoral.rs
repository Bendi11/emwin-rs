use crate::dt::{area::{GeographicalAreaDesignator, ReferenceTimeDesignator}, DataTypeDesignatorParseError, UnparsedProductIdentifier, level::AirLevelDesignator};

/// P
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PictoralInformation {
    /// T2
    pub subtype: PictoralInformationSubType,
    /// A1
    pub area: GeographicalAreaDesignator,
    /// A2
    pub time: ReferenceTimeDesignator,
    /// ii
    pub level: AirLevelDesignator,
}

/// Term T2 definitions when T1=PictoralInformationBinary or PictoralInformationRegionalBinary
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PictoralInformationSubType {
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

impl PictoralInformationSubType {
    pub fn parse_t2(t1: char, t2: char) -> Result<Self, DataTypeDesignatorParseError> {
        Ok(match t2 {
            'A' => Self::RadarData,
            'B' => Self::Cloud,
            'C' => Self::ClearAirTurbulence,
            'D' => Self::Thickness,
            'E' => Self::Precipitation,
            'F' => Self::AerologicalDiagrams,
            'G' => Self::SignificantWeather,
            'H' => Self::Height,
            'I' => Self::IceFlow,
            'J' => Self::WaveHeight,
            'K' => Self::SwellHeight,
            'L' => Self::PlainLanguage,
            'M' => Self::NationalUse,
            'N' => Self::Radiation,
            'O' => Self::VerticalVelocity,
            'P' => Self::Pressure,
            'Q' => Self::WetBulbPotentialTemperature,
            'R' => Self::RelativeHumidity,
            'S' => Self::SnowCover,
            'T' => Self::Temperature,
            'U' => Self::EastwardWindComponent,
            'V' => Self::NorthwardWindComponent,
            'W' => Self::Wind,
            'X' => Self::LiftedIndex,
            'Y' => Self::ObservationalPlottedChart,
            other => return Err(DataTypeDesignatorParseError::UnrecognizedT2(t1, other)),
        })
    }
}

impl TryFrom<UnparsedProductIdentifier> for PictoralInformation {
    type Error = DataTypeDesignatorParseError;
    fn try_from(value: UnparsedProductIdentifier) -> Result<Self, Self::Error> {
        Ok(Self {
            subtype: PictoralInformationSubType::parse_t2(value.t1, value.t2)?,
            area: GeographicalAreaDesignator::try_from(value.a1)?,
            time: ReferenceTimeDesignator::parse_for_dghjopt(value.a2)?,
            level: AirLevelDesignator::try_from(value.ii)?,
        })
    }
}
