use crate::dt::area::{GeographicalAreaDesignator, ReferenceTimeDesignator};


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridPointInformation {
    /// T2
    pub subtype: GRIDSubType,
    /// A1
    pub area: GeographicalAreaDesignator,
    /// A2
    pub time: ReferenceTimeDesignator,
}

/// Term T2 definitions when T1=GridPointInformation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GRIDSubType {
    RadarData,
    Cloud,
    Vorticity,
    Thickness,
    Precipitation,
    Divergence,
    Height,
    WaveHeight,
    SwellHeight,
    NationalUse,
}
