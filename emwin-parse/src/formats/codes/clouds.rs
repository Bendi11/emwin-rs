use nom::{combinator::{opt, map_res}, branch::alt, error::context, bytes::complete::take, Parser};
use nom_supreme::tag::complete::tag;
use uom::si::f32::Length;

use crate::{ParseResult, formats::codetbl::parse_1690};

/// Cloud amount NsNsNs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CloudAmount {
    Few,
    Scattered,
    Broken,
    Overcast,
}

/// Report containing a cloud level and observed cloud height
#[derive(Clone, Copy, Debug)]
pub struct CloudReport {
    pub amount: Option<CloudAmount>,
    pub altitude: Length,
}

impl CloudReport {
    pub fn parse(input: &str) -> ParseResult<&str, Option<Self>> {
        let (input, val) = opt(alt((tag("NSC"), tag("SKC"))))(input)?;
        
        if val.is_some() {
            return Ok((input, None));
        }

        let (input, amount) = context("cloud amount code", alt((
                tag("VV").map(|_| None),
                map_res(take(3usize), |s: &str| {
                    Ok(match s {
                        "FEW" => Some(CloudAmount::Few),
                        "SCT" => Some(CloudAmount::Scattered),
                        "BKN" => Some(CloudAmount::Broken),
                        "OVC" => Some(CloudAmount::Overcast),
                        _ => return Err("invalid cloud amount code"),
                    })
                }),
            )),
        )(input)?;

        
        let (input, altitude) = parse_1690(input)?;
        Ok((
            input,
            Some(CloudReport { amount, altitude }),
        ))
    }
}
