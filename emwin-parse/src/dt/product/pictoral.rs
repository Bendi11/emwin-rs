use crate::dt::area::{GeographicalAreaDesignator, ReferenceTimeDesignator};


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PictoralInformation {
    /// T2
    pub subtype: PictoralInformationSubType,
    /// A1
    pub area: GeographicalAreaDesignator,
    /// A2
    pub time: ReferenceTimeDesignator,
}

/// Term T2 definitions when T1=PictoralInformationBinary or PictoralInformationRegionalBinary
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PictoralInformationSubType {
    RadarData,
    Cloud,
    ClearAirTurbulence,
    Thickness,
    Precipitation,
    AerologicalDiagrams,
    SignificantWeather,
    Height,
    IceFlow,
    WaveHeight,
    SwellHeight,
    PlainLanguage,
    NationalUse,
    Radiation,
    VerticalVelocity,
    Pressure,
    WetBulbPotentialTemperature,
    RelativeHumidity,
    SnowCover,
    Temperature,
    EastwardWindComponent,
    NorthwardWindComponent,
    Wind,
    LiftedIndex,
    ObservationalPlottedChart,
}
