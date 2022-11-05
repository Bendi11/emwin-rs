use nom::Parser;

use crate::{ParseResult, ParseError};

/// Combinator used to provide a recovery strategy for tolerant parsers
/// `recovery` is a parser that should consume characters until the expected end of the parsed
/// text, like [take_while](nom::bytes::complete::take_while)
pub fn recover<I, O1, O2, P, S>(mut parser: P, mut recovery: S) -> impl FnMut(I) -> ParseResult<I, Option<O1>>
where
    I: Copy,
    P: Parser<I, O1, ParseError<I>>,
    S: Parser<I, O2, ParseError<I>>,
{
    move |input| match parser.parse(input) {
        Ok((r, o)) => Ok((r, Some(o))),
        Err(nom::Err::Error(_)) => Ok((
            recovery.parse(input)?.0,
            None
        )),
        Err(e) => return Err(e),
    }
}

#[cfg(test)]
mod test {
    use nom::bytes::complete::take_until;

    use crate::parse::fromstr;

    use super::*;

    const TEST_FAIL: &str = "123z1 test";

    #[test]
    fn test_recovery() {
        let mut parser = recover(
            fromstr::<'_, f32>(5),
            take_until(" "),
        );

        let (rest, num) = parser(TEST_FAIL).unwrap();
        assert!(num.is_none());
        assert_eq!(rest, " test");
    }
}
