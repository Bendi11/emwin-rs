use nom::{character::streaming::char, combinator::opt, sequence::tuple, Parser};
use uom::si::{f32::ThermodynamicTemperature, thermodynamic_temperature::degree_celsius};

use crate::{parse::fromstr, ParseError};

pub mod clouds;
pub mod runway;
pub mod sea;
pub mod visibility;
pub mod weather;
pub mod wind;

/// Parse a temperature in degrees C with optional preceding `M` character indicating minus
pub fn temperature<'a>(
    len: usize,
) -> impl Parser<&'a str, ThermodynamicTemperature, ParseError<&'a str>> {
    tuple((
        opt(char('M').map(|_| -1f32)).map(|v| v.unwrap_or(1f32)),
        fromstr::<'_, f32>(len),
    ))
    .map(|(m, t)| ThermodynamicTemperature::new::<degree_celsius>(t * m))
}
