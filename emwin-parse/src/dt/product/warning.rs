

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
