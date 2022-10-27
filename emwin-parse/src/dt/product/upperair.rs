use crate::dt::code::CodeForm;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UpperAirData {
    pub subtype: UpperT2,
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
