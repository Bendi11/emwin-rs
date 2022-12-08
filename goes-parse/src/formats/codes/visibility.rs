use nom::{
    character::{
        complete::{digit1, space0},
        streaming::char,
    },
    combinator::{map_res, opt, complete},
    error::context,
    sequence::{preceded, tuple, separated_pair}, Parser, branch::alt,
};
use nom_supreme::tag::complete::tag;
use uom::si::{
    f32::Length,
    length::{meter, mile},
};

use crate::ParseResult;

/// Parse a surface horizontal visibility in `VVVV` format (pg. 227)
pub fn vvvv(input: &str) -> ParseResult<&str, Length> {
    fn fraction(input: &str) -> ParseResult<&str, f32> {
        separated_pair(
            map_res(digit1, |s: &str| s.parse::<f32>()),
            char('/'),
            map_res(digit1, |s: &str| s.parse::<f32>()),
        )
            .map(|(num, den)| num / den)
            .parse(input)
    }
    
    let (input, ignore) = complete(opt(tag("P6SM")))(input)?;
    if ignore.is_some() {
        return Ok((input, Length::new::<mile>(6f32)))
    }

    let (input, ignore) = complete(opt(tag("M1/4SM")))(input)?;
    if ignore.is_some() {
        return Ok((input, Length::new::<mile>(0f32)))
    }

    let (input, first) = complete(context(
        "horizontal visibility first term",
        alt((
            fraction,
            tuple((
                map_res(digit1, |s: &str| s.parse::<f32>()),
                opt(preceded(
                    space0,
                    fraction,
                ))
            )).map(|(whole, frac)| whole + frac.unwrap_or(0f32)),
        )) 
    ))(input)?;

    let (input, miles) = complete(opt(tag("SM")))(input)?;
    Ok(match miles.is_some() {
        true => (input, Length::new::<mile>(first)),
        false => (input, Length::new::<meter>(first)),
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_vvvv() {
        //assert_eq!(vvvv("15SM").unwrap().1, Length::new::<mile>(15f32),);
        vvvv("2 1/2SM").unwrap_or_else(|e| panic!("{}", crate::display_error(e)));
    }
}
