use crate::dt::{area::AreaCode, DataTypeDesignatorParseError, UnparsedProductIdentifier};

/// W
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Warning {
    /// T2
    pub subtype: WarningSubType,
    /// A1A2
    pub area: AreaCode,
    /// ii
    pub enumerator: u8,
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
            area: AreaCode::try_from((value.a1, value.a2))?,
            enumerator: value.ii,
        })
    }
}
