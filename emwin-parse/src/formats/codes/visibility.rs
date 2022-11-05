use std::num::ParseFloatError;

use nom::{error::context, combinator::{opt, map_res}, sequence::{preceded, terminated, tuple}, branch::alt, character::{complete::{digit1, space1}, streaming::char}};
use nom_supreme::tag::complete::tag;
use uom::si::{f32::Length, length::{mile, meter}};

use crate::ParseResult;


/// Parse a surface horizontal visibility in `VVVV` format (pg. 227)
pub fn vvvv(input: &str) -> ParseResult<&str, Option<Length>> {
    let mut vis_sm = context(
        "horizontal visibility",
        terminated(
            tuple((
                map_res(digit1, |s: &str| s.parse::<f32>()),
                opt(preceded(
                    char('/'),
                    map_res(digit1, |s: &str| s.parse::<f32>()),
                )),
            )),
            tag("SM"),
        ),
    );

    enum VisFirst {
        Number(f32),
        SM(f32),
    }

    let (input, vis_first) = opt(context(
        "cloud visibility",
        preceded(
            space1,
            alt((
                map_res(&mut vis_sm, |(first, denominator)| {
                    Ok::<VisFirst, ParseFloatError>(match denominator {
                        Some(d) => VisFirst::SM(first / d),
                        None => VisFirst::SM(first),
                    })
                }),
                map_res(digit1, |s: &str| {
                    Ok::<VisFirst, ParseFloatError>(VisFirst::Number(s.parse::<f32>()?))
                }),
                map_res(tag("P6SM"), |_| {
                    Ok::<_, ParseFloatError>(VisFirst::SM(6f32))
                }),
            )),
        ),
    ))(input)?;

    Ok(match vis_first {
        Some(vis_first) => match vis_first {
            VisFirst::Number(whole) => match opt(vis_sm)(input)? {
                (input, Some((numerator, Some(denominator)))) => (
                    input,
                    Some(Length::new::<mile>(whole + numerator / denominator)),
                ),
                (input, Some((numerator, None))) => {
                    (input, Some(Length::new::<mile>(whole + numerator)))
                }
                (input, None) => (input, Some(Length::new::<meter>(whole))),
            },
            VisFirst::SM(vis) => (input, Some(Length::new::<mile>(vis))),
        },
        None => (input, None),
    })
}
