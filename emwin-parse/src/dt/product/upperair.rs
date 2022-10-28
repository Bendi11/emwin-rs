use crate::dt::{code::CodeForm, UnparsedProductIdentifier, DataTypeDesignatorParseError, area::AreaCode};

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

/// Term T2 definitions when T1=UpperAirData
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpperAirDataSubType {
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

impl TryFrom<UnparsedProductIdentifier> for UpperAirData {
    type Error = DataTypeDesignatorParseError;
    fn try_from(value: UnparsedProductIdentifier) -> Result<Self, Self::Error> {
        Ok(Self {
            subtype: match value.t2 {
                'A' => UpperAirDataSubType::AircraftReport(CodeForm::ICAO),
                'D' => UpperAirDataSubType::AircraftReport(CodeForm::AMDAR),
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
                'R' => UpperAirDataSubType::AircraftReport(CodeForm::RECCO),
                'S' => UpperAirDataSubType::UpperLevelPressureTemperatureHumidityWindA,
                'T' => UpperAirDataSubType::AircraftReport(CodeForm::CODAR),
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
