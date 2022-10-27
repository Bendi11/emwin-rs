use crate::dt::{area::AreaCode, UnparsedProductIdentifier, DataTypeDesignatorParseError};


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Analysis {
    pub subtype: AnalysisSubType,
    pub area: AreaCode,
}

/// Term T2 definition when T1=Analysis
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnalysisSubType {
    Cyclone,
    Hydrological,
    Thickness,
    Ice,
    Ozone,
    Radar,
    Surface,
    UpperAir,
    WeatherSummary,
    Misc,
}

impl TryFrom<UnparsedProductIdentifier> for Analysis {
    type Error = DataTypeDesignatorParseError;
    fn try_from(value: UnparsedProductIdentifier) -> Result<Self, Self::Error> {
        Ok(Self {
            subtype: match value.t2 {
                'C' => AnalysisSubType::Cyclone,
                'G' => AnalysisSubType::Hydrological,
                'H' => AnalysisSubType::Thickness,
                'I' => AnalysisSubType::Ice,
                'O' => AnalysisSubType::Ozone,
                'R' => AnalysisSubType::Radar,
                'S' => AnalysisSubType::Surface,
                'U' => AnalysisSubType::UpperAir,
                'W' => AnalysisSubType::WeatherSummary,
                'X' => AnalysisSubType::Misc,
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2('A', other)),
            },
            area: AreaCode::try_from((value.a1, value.a2))?
        })
    }
}
