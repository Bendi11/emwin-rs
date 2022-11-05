//! Utility functions for parsing with error recovery

use std::str::FromStr;

use nom::{combinator::map_res, bytes::complete::take, error::FromExternalError};

use crate::ParseResult;

pub mod time;

/// Parse a value of type `T` using `T`'s [FromStr] implementation by taking `n` characters from
/// the input string
pub fn fromstr<'a, T>(n: usize) -> impl FnMut(&'a str) -> ParseResult<&'a str, T>
where
    T: FromStr,
    crate::ParseError<&'a str>: FromExternalError<&'a str, <T as FromStr>::Err>,
{
    map_res(
        take(n),
        <T as FromStr>::from_str
    )
}
