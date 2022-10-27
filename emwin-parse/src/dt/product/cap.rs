use crate::dt::area::ReferenceTimeDesignator;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommonAlertProtocolMessage {
    pub time: ReferenceTimeDesignator,
}
