
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OceanographicInformation {
    pub subtype: OceanographicSubType,
}

/// Term T2 definitions when T1=OceanographicInformation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OceanographicSubType {
    Depth,
    IceConcentration,
    IceThickness,
    IceDrift,
    IceGrowth,
    IceConvergenceDivergence,
    TemperatureAnomaly,
    DepthAnomaly,
    Salinity,
    Temperature,
    CurrentComponent,
    TemperatureWarming,
    Mixed,
}
