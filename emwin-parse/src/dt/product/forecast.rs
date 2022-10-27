use crate::dt::{UnparsedProductIdentifier, DataTypeDesignatorParseError, area::AreaCode};



#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Forecast {
    pub subtype: ForecastSubType,
    pub area: AreaCode,
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
    ShippingArea
}

impl TryFrom<UnparsedProductIdentifier> for Forecast {
    type Error = DataTypeDesignatorParseError;
    fn try_from(value: UnparsedProductIdentifier) -> Result<Self, Self::Error> {
        Ok(Self {
            subtype: match value.t2 {
                'A' => ForecastT2::AviationGAMETAdvisories,
                'B' => ForecastT2::UpperWindsTemps,
                'C' => ForecastT2::AerodomeVTLT12,
                'D' => ForecastT2::RadiologicalTrajectoryDose,
                'E' => ForecastT2::Extended,
                'F' => ForecastT2::Shipping,
                'G' => ForecastT2::Hydrological,
                'H' => ForecastT2::UpperAirThickness,
                'I' => ForecastT2::Iceberg,
                'J' => ForecastT2::RadioWarningService,
                'K' => ForecastT2::TropicalCycloneAdvisory,
                'L' => ForecastT2::Local,
                'M' => ForecastT2::TemperatureExtreme,
                'N' => ForecastT2::SpaceWeatherAdvisory,
                'O' => ForecastT2::Guidance,
                'P' => ForecastT2::Public,
                'Q' => ForecastT2::OtherShipping,
                'R' => ForecastT2::AviationRoute,
                'S' => ForecastT2::Surface,
                'T' => ForecastT2::AerodomeVTGE12,
                'U' => ForecastT2::UpperAir,
                'V' => ForecastT2::VolcanicAshAdvisory,
                'W' => ForecastT2::WinterSports,
                'X' => ForecastT2::Misc,
                'Z' => ForecastT2::ShippingArea,
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2('F', other)),
            },
            area: AreaCode::try_from((value.a1, value.a2))?, 
        })
    }
}
