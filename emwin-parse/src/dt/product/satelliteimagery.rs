

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SatelliteImagery {
    pub subtype: SatelliteImagerySubType,
}


/// Term T2 definitions when T1=SatelliteImagery
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SatelliteImagerySubType {
    CloudTopTemperature,
    Fog,
    Infared,
    SurfaceTemperature,
}
