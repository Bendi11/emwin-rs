use crate::dt::{UnparsedProductIdentifier, DataTypeDesignatorParseError, area::AreaCode};


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Notice {
    pub subtype: NoticeSubType,
    pub area: AreaCode,
}

/// Term T2 definition when T1=Notice
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NoticeSubType {
    Hydrological,
    Marine,
    NuclearEmergencyResponse,
    METNOWIFMA,
    ProductGenerationDelay,
    TESTMSG,
    WarningRelatedOrCancellation,
}

impl TryFrom<UnparsedProductIdentifier> for Notice {
    type Error = DataTypeDesignatorParseError;
    fn try_from(value: UnparsedProductIdentifier) -> Result<Self, Self::Error> {
        Ok(Self {
            subtype: match value.t2 {
                'G' => NoticeT2::Hydrological,
                'H' => NoticeT2::Marine,
                'N' => NoticeT2::NuclearEmergencyResponse,
                'O' => NoticeT2::METNOWIFMA,
                'P' => NoticeT2::ProductGenerationDelay,
                'T' => NoticeT2::TESTMSG,
                'W' => NoticeT2::WarningRelatedOrCancellation,
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2('N', other)),
            },
            area: AreaCode::try_from((value.a1, value.a2))?,
        })
    }
}
