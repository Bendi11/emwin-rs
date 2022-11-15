use crate::dt::{
    area::{GeographicalAreaDesignator, ReferenceTimeDesignator},
    DataTypeDesignatorParseError, UnparsedProductIdentifier,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommonAlertProtocolMessage {
    /// A1
    pub area: GeographicalAreaDesignator,
    /// A2
    pub time: ReferenceTimeDesignator,
}

impl TryFrom<UnparsedProductIdentifier> for CommonAlertProtocolMessage {
    type Error = DataTypeDesignatorParseError;
    fn try_from(value: UnparsedProductIdentifier) -> Result<Self, Self::Error> {
        Ok(Self {
            area: GeographicalAreaDesignator::try_from(value.a1)?,
            time: ReferenceTimeDesignator::parse_for_qxy(value.a2)?,
        })
    }
}
