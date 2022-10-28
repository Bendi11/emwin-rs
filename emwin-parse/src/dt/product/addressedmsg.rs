use crate::dt::{UnparsedProductIdentifier, DataTypeDesignatorParseError};


/// format @ WMO-No. 386 p.103 attachment II-6
///
/// B
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AddressedMessage {
    /// T2
    pub binary: bool,
    /// A1A2
    pub kind: AddressedMessageType,
}

/// The type of message an [AddressedMessage] is
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddressedMessageType {
    Administrative,
    Service,
    GTSRequest,
    RequestToDB,
    GTSOrDBResponse,
}

impl TryFrom<UnparsedProductIdentifier> for AddressedMessage {
    type Error = DataTypeDesignatorParseError;
    fn try_from(value: UnparsedProductIdentifier) -> Result<Self, Self::Error> {
        Ok(Self {
            binary: match value.t2 {
                'M' => false,
                'I' => true,
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2('B', other)),
            },
            kind: match (value.a1, value.a2) {
                ('A', 'A') => AddressedMessageType::Administrative,
                ('B', 'B') => AddressedMessageType::Service,
                ('R', 'R') => AddressedMessageType::GTSRequest,
                ('R', 'Q') => AddressedMessageType::RequestToDB,
                ('D', 'A') => AddressedMessageType::GTSOrDBResponse,
                _ => return Err(DataTypeDesignatorParseError::UnrecognizedA2(value.t1, value.t2, value.a1, value.a2)),
            }
        })
    }
}
