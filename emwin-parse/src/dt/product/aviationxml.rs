use crate::dt::{area::AreaCode, UnparsedProductIdentifier, DataTypeDesignatorParseError};


/// L
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AviationInformationXML {
    /// T2
    pub subtype: AviationInformationXMLSubType,
    /// A1A2
    pub area: AreaCode,
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

impl TryFrom<UnparsedProductIdentifier> for AviationInformationXML {
    type Error = DataTypeDesignatorParseError;
    fn try_from( value: UnparsedProductIdentifier) -> Result<Self, Self::Error> {
        Ok(Self {
            subtype: match value.t2 {
                'A' => AviationInformationXMLSubType::AviationRoutineReportMETAR,
                'C' => AviationInformationXMLSubType::AerodomeForecastTAFVTLT12,
                'K' => AviationInformationXMLSubType::TropicalCycloneAdvisory,
                'N' => AviationInformationXMLSubType::SpaceWeatherAdvisory,
                'P' => AviationInformationXMLSubType::SpecialAviationWeatherReportSPECI,
                'S' => AviationInformationXMLSubType::AviationGeneralWarningSIGMET,
                'T' => AviationInformationXMLSubType::AerodomeForecastTAFVTGE12,
                'U' => AviationInformationXMLSubType::VolcanicAshAdvisory,
                'V' => AviationInformationXMLSubType::AviationVolcanicAshWarningSIGMET,
                'W' => AviationInformationXMLSubType::AIRMET,
                'Y' => AviationInformationXMLSubType::AviationTropicalCycloneWarningSIGMET,
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2(value.t1, other)),
            },
            area: AreaCode::try_from((value.a1, value.a2))?,
        }) 
    }
}
