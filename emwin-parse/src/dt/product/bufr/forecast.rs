use crate::dt::area::ReferenceTimeDesignator;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ForecastDataBinary {
    /// T2
    pub subtype: ObservationalDataBinaryBUFRSubType,
    /// A2
    pub time: ReferenceTimeDesignator,
}
