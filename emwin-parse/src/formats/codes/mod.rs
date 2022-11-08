use nom::{Parser, sequence::{preceded, tuple}, combinator::opt, character::streaming::char};
use uom::si::{f32::ThermodynamicTemperature, thermodynamic_temperature::degree_celsius};

use crate::{ParseError, parse::fromstr};

pub mod visibility;
pub mod weather;
pub mod wind;
pub mod clouds;
pub mod sea;
pub mod runway;

/// Parse a temperature in degrees C with optional preceding `M` character indicating minus
pub fn temperature<'a>(len: usize) -> impl Parser<&'a str, ThermodynamicTemperature, ParseError<&'a str>> {
    tuple((
        opt(char('M').map(|_| -1f32)).map(|v| v.unwrap_or(1f32)),
        fromstr::<'_, f32>(len),
    )).map(|(m, t)| ThermodynamicTemperature::new::<degree_celsius>(t * m))
}
