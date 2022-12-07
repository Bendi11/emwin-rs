use std::ops::Deref;

use crate::dt::{
    area::AreaCode, code::CodeForm, DataTypeDesignatorParseError, UnparsedProductIdentifier,
};

/// U
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UpperAirData {
    /// T2
    pub subtype: UpperAirDataSubType,
    /// A1A2
    pub area: AreaCode,
    /// ii
    pub enumerator: u8,
}

/// Subset of [CodeForm] that contains variants that can be parsed to an
/// [UpperAirDataSubType::AircraftReport]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AircraftReportCodeForm {
    ICAO,
    AMDAR,
    RECCO,
    CODAR,
}

/// Term T2 definitions when T1=UpperAirData
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpperAirDataSubType {
    AircraftReport(AircraftReportCodeForm),
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

impl TryFrom<UnparsedProductIdentifier> for UpperAirData {
    type Error = DataTypeDesignatorParseError;
    fn try_from(value: UnparsedProductIdentifier) -> Result<Self, Self::Error> {
        Ok(Self {
            subtype: match value.t2 {
                'A' => UpperAirDataSubType::AircraftReport(AircraftReportCodeForm::ICAO),
                'D' => UpperAirDataSubType::AircraftReport(AircraftReportCodeForm::AMDAR),
                'E' => UpperAirDataSubType::UpperLevelPressureTemperatureHumidityWindD,
                'F' => UpperAirDataSubType::UpperLevelPressureTemperatureHumidityWindCD,
                'G' => UpperAirDataSubType::UpperWindB,
                'H' => UpperAirDataSubType::UpperWindC,
                'I' => UpperAirDataSubType::UpperWindAB,
                'K' => UpperAirDataSubType::UpperLevelPressureTemperatureHumidityWindB,
                'L' => UpperAirDataSubType::UpperLevelPressureTemperatureHumidityWindC,
                'M' => UpperAirDataSubType::UpperLevelPressureTemperatureHumidityWindAB,
                'N' => UpperAirDataSubType::RocketsondeReport,
                'P' => UpperAirDataSubType::UpperWindA,
                'Q' => UpperAirDataSubType::UpperWindD,
                'R' => UpperAirDataSubType::AircraftReport(AircraftReportCodeForm::RECCO),
                'S' => UpperAirDataSubType::UpperLevelPressureTemperatureHumidityWindA,
                'T' => UpperAirDataSubType::AircraftReport(AircraftReportCodeForm::CODAR),
                'X' => UpperAirDataSubType::Misc,
                'Y' => UpperAirDataSubType::UpperWindCD,
                'Z' => UpperAirDataSubType::UpperLevelPressureTemperatureHumidityWindABCD,
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2('U', other)),
            },
            area: AreaCode::try_from((value.a1, value.a2))?,
            enumerator: value.ii,
        })
    }
}

impl TryFrom<CodeForm> for AircraftReportCodeForm {
    type Error = ();

    fn try_from(value: CodeForm) -> Result<Self, Self::Error> {
        match value {
            CodeForm::ICAO => Ok(Self::ICAO),
            CodeForm::AMDAR => Ok(Self::AMDAR),
            CodeForm::RECCO => Ok(Self::RECCO),
            CodeForm::CODAR => Ok(Self::CODAR),
            _ => Err(()),
        }
    }
}

impl Deref for AircraftReportCodeForm {
    type Target = CodeForm;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::ICAO => &CodeForm::ICAO,
            Self::AMDAR => &CodeForm::AMDAR,
            Self::RECCO => &CodeForm::RECCO,
            Self::CODAR => &CodeForm::CODAR,
        }
    }
}
