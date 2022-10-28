use crate::dt::{area::{GeographicalAreaDesignator, ReferenceTimeDesignator}, UnparsedProductIdentifier, DataTypeDesignatorParseError};

use super::pictoral::PictoralInformationSubType;

/// Q
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RegionalPictoralInformation {
    /// T2
    pub subtype: PictoralInformationSubType,
    /// A1
    pub area: GeographicalAreaDesignator,
    /// A2
    pub time: ReferenceTimeDesignator,
}

impl TryFrom<UnparsedProductIdentifier> for RegionalPictoralInformation {
    type Error = DataTypeDesignatorParseError;
    fn try_from(value: UnparsedProductIdentifier) -> Result<Self, Self::Error> {
        Ok(Self {
            subtype: PictoralInformationSubType::parse_t2(value.t1, value.t2)?,
            area: GeographicalAreaDesignator::try_from(value.a1)?,
            time: ReferenceTimeDesignator::parse_for_qxy(value.a2)?,
        })
    }
}
