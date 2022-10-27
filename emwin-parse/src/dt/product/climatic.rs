use crate::dt::{code::CodeForm, area::AreaCode, UnparsedProductIdentifier, DataTypeDesignatorParseError};


/// C
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClimaticData {
    pub subtype: ClimaticDataSubType,
    pub area: AreaCode,
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
            subtype: match second {
                'A' => ClimaticDataT2::Anomaly,
                'E' => ClimaticDataT2::UpperAirMonthlyMean,
                'H' => ClimaticDataT2::SurfaceMonthlyMean(CodeForm::CLIMATSHIP),
                'O' => ClimaticDataT2::OceanMonthlyMean,
                'S' => ClimaticDataT2::SurfaceMonthlyMean(CodeForm::CLIMAT),
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2('C', other)),
            },
            area: AreaCode::try_from((value.a1, value.a2))?,
        })
    }
}
