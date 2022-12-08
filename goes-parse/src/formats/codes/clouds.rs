use nom::{
    branch::alt,
    bytes::complete::take,
    combinator::{map_res, opt},
    error::context,
    Parser,
};
use nom_supreme::tag::complete::tag;
use uom::si::f32::Length;

use crate::ParseResult;

use super::parse_1690;

/// Cloud covering specifying the significant cloud shape
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CloudCover {
    Cumulonimbus,
    ToweringCumulonimbus,
}

/// Cloud amount NsNsNs
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CloudAmount {
    Few,
    Scattered,
    Broken,
    Overcast,
}

/// Report containing a cloud level and observed cloud height
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct CloudReport {
    pub amount: Option<CloudAmount>,
    pub altitude: Length,
    pub cover: Option<CloudCover>,
}

impl CloudReport {
    pub fn parse(input: &str) -> ParseResult<&str, Option<Self>> {
        let (input, val) = opt(alt((tag("NSC"), tag("SKC"), tag("CLR"), tag("NCD"))))(input)?;

        if val.is_some() {
            return Ok((input, None));
        }

        let (input, amount) = context(
            "cloud amount code",
            alt((
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

        let (input, cover) = opt(
            alt((
                tag("CB").map(|_| Some(CloudCover::Cumulonimbus)),
                tag("TCU").map(|_| Some(CloudCover::ToweringCumulonimbus)),
                tag("///").map(|_| None)
            ))
        ).map(Option::flatten).parse(input)?;

        Ok((input, Some(CloudReport { amount, altitude, cover })))
    }
}

#[cfg(test)]
mod test {
    use nom::{sequence::preceded, character::complete::multispace0};

    use crate::parse::multi_opt;

    use super::*;

    #[test]
    fn test_clouds() {
        let (_, c) = multi_opt(preceded(multispace0, CloudReport::parse)).parse(" BKN003 OVC009 ").unwrap();

        assert_eq!(c.len(), 2);
    }
}
