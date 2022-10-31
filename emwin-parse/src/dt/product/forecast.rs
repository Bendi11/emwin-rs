use crate::dt::{area::AreaCode, DataTypeDesignatorParseError, UnparsedProductIdentifier};

/// F
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Forecast {
    /// T2
    pub subtype: ForecastSubType,
    /// A1A2
    pub area: AreaCode,
    /// ii
    pub enumerator: u8,
}

/// Term T2 definition when T1=Forecast
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForecastSubType {
    AviationGAMETAdvisories,
    UpperWindsTemps,
    AerodomeVTLT12,
    RadiologicalTrajectoryDose,
    Extended,
    Shipping,
    Hydrological,
    UpperAirThickness,
    Iceberg,
    RadioWarningService,
    TropicalCycloneAdvisory,
    Local,
    TemperatureExtreme,
    SpaceWeatherAdvisory,
    Guidance,
    Public,
    OtherShipping,
    AviationRoute,
    Surface,
    AerodomeVTGE12,
    UpperAir,
    VolcanicAshAdvisory,
    WinterSports,
    Misc,
    ShippingArea,
}

impl TryFrom<UnparsedProductIdentifier> for Forecast {
    type Error = DataTypeDesignatorParseError;
    fn try_from(value: UnparsedProductIdentifier) -> Result<Self, Self::Error> {
        Ok(Self {
            subtype: match value.t2 {
                'A' => ForecastSubType::AviationGAMETAdvisories,
                'B' => ForecastSubType::UpperWindsTemps,
                'C' => ForecastSubType::AerodomeVTLT12,
                'D' => ForecastSubType::RadiologicalTrajectoryDose,
                'E' => ForecastSubType::Extended,
                'F' => ForecastSubType::Shipping,
                'G' => ForecastSubType::Hydrological,
                'H' => ForecastSubType::UpperAirThickness,
                'I' => ForecastSubType::Iceberg,
                'J' => ForecastSubType::RadioWarningService,
                'K' => ForecastSubType::TropicalCycloneAdvisory,
                'L' => ForecastSubType::Local,
                'M' => ForecastSubType::TemperatureExtreme,
                'N' => ForecastSubType::SpaceWeatherAdvisory,
                'O' => ForecastSubType::Guidance,
                'P' => ForecastSubType::Public,
                'Q' => ForecastSubType::OtherShipping,
                'R' => ForecastSubType::AviationRoute,
                'S' => ForecastSubType::Surface,
                'T' => ForecastSubType::AerodomeVTGE12,
                'U' => ForecastSubType::UpperAir,
                'V' => ForecastSubType::VolcanicAshAdvisory,
                'W' => ForecastSubType::WinterSports,
                'X' => ForecastSubType::Misc,
                'Z' => ForecastSubType::ShippingArea,
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2('F', other)),
            },
            area: AreaCode::try_from((value.a1, value.a2))?,
            enumerator: value.ii,
        })
    }
}
