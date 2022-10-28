use crate::dt::{area::AreaCode, UnparsedProductIdentifier, DataTypeDesignatorParseError};

/// V
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct National {
    /// A1A2
    pub area: AreaCode,
    /// ii
    pub enumerator: u8,
}

impl TryFrom<UnparsedProductIdentifier> for National {
    type Error = DataTypeDesignatorParseError;
    fn try_from(value: UnparsedProductIdentifier) -> Result<Self, Self::Error> {
        Ok(Self {
            area: AreaCode::try_from((value.a1, value.a2))?,
            enumerator: value.ii,
        })
    }
}
