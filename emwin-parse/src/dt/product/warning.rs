use crate::dt::{UnparsedProductIdentifier, DataTypeDesignatorParseError};


/// W
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Warning {
    pub subtype: WarningSubType,
}

/// Term T2 definitions when T1=Warning
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WarningSubType {
    AIRMET,
    TropicalCycloneSIGMET,
    Tsunami,
    Tornado,
    HydrologicalRiverFlood,
    MarineCoastalFlood,
    Other,
    HumanitarianActivities,
    SIGMET,
    TropicalCycloneTyphoonHurricane,
    SevereThunderstorm,
    VolcanicAshCloudsSIGMET,
    WarningsWeatherSummary,
}

impl TryFrom<UnparsedProductIdentifier> for Warning {
    type Error = DataTypeDesignatorParseError;
    fn try_from(value: UnparsedProductIdentifier) -> Result<Self, Self::Error> {
        Ok(Self {
            subtype: match value.t2 {
                'A' => WarningSubType::AIRMET,
                'C' => WarningSubType::TropicalCycloneSIGMET,
                'E' => WarningSubType::Tsunami,
                'F' => WarningSubType::Tornado,
                'G' => WarningSubType::HydrologicalRiverFlood,
                'H' => WarningSubType::MarineCoastalFlood,
                'O' => WarningSubType::Other,
                'R' => WarningSubType::HumanitarianActivities,
                'S' => WarningSubType::SIGMET,
                'T' => WarningSubType::TropicalCycloneTyphoonHurricane,
                'U' => WarningSubType::SevereThunderstorm,
                'V' => WarningSubType::VolcanicAshCloudsSIGMET,
                'W' => WarningSubType::WarningsWeatherSummary,
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2('W', other)),
            },
        })
    }
}
