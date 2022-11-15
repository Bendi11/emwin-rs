use crate::dt::{
    area::AreaCode, code::CodeForm, DataTypeDesignatorParseError, UnparsedProductIdentifier,
};

/// C
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClimaticData {
    /// T2
    pub subtype: ClimaticDataSubType,
    /// A1A2
    pub area: AreaCode,
    /// ii
    pub enumerator: u8,
}

/// Term T2 definition when T1=ClimaticData
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClimaticDataSubType {
    Anomaly,
    UpperAirMonthlyMean,
    SurfaceMonthlyMean(CodeForm),
    OceanMonthlyMean,
}

impl TryFrom<UnparsedProductIdentifier> for ClimaticData {
    type Error = DataTypeDesignatorParseError;
    fn try_from(value: UnparsedProductIdentifier) -> Result<Self, Self::Error> {
        Ok(Self {
            subtype: match value.t2 {
                'A' => ClimaticDataSubType::Anomaly,
                'E' => ClimaticDataSubType::UpperAirMonthlyMean,
                'H' => ClimaticDataSubType::SurfaceMonthlyMean(CodeForm::CLIMATSHIP),
                'O' => ClimaticDataSubType::OceanMonthlyMean,
                'S' => ClimaticDataSubType::SurfaceMonthlyMean(CodeForm::CLIMAT),
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2('C', other)),
            },
            area: AreaCode::try_from((value.a1, value.a2))?,
            enumerator: value.ii,
        })
    }
}
