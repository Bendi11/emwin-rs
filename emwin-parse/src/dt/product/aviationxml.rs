
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AviationInformationXML {
    pub subtype: AviationInformationXMLSubType,
}

/// Term T2 definitions when T1=AviationInformationXML
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AviationInformationXMLSubType {
    AviationRoutineReportMETAR,
    AerodomeForecastTAFVTLT12,
    TropicalCycloneAdvisory,
    SpaceWeatherAdvisory,
    SpecialAviationWeatherReportSPECI,
    AviationGeneralWarningSIGMET,
    AerodomeForecastTAFVTGE12,
    VolcanicAshAdvisory,
    AviationVolcanicAshWarningSIGMET,
    AIRMET,
    AviationTropicalCycloneWarningSIGMET,
}
