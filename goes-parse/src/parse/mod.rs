//! Utility functions for parsing with error recovery

use std::str::FromStr;

use nom::{bytes::complete::take, combinator::map_res, error::FromExternalError, Parser};

use crate::ParseResult;

pub mod recover;
pub mod time;

/// Parse 0 or more elements of `parser`
pub fn multi<T, I, E, P>(mut parser: P) -> impl Parser<I, Vec<T>, E>
where
    P: Parser<I, T, E>,
    I: Copy,
{
    move |mut input: I| {
        let mut vec = Vec::new();
        loop {
            match parser.parse(input) {
                Ok((new_input, o)) => {
                    input = new_input;
                    vec.push(o);
                }
                Err(nom::Err::Failure(e)) => break Err(nom::Err::Failure(e)),
                Err(_) => break Ok((input, vec)),
            }
        }
    }
}

/// Attempt to use the given parser returning an `Option<T>` to build a vector of `T`
pub fn multi_opt<T, I, E, P>(mut parser: P) -> impl Parser<I, Vec<T>, E>
where
    P: Parser<I, Option<T>, E>,
    I: Copy,
{
    move |mut input: I| {
        let mut vec = Vec::new();
        loop {
            match parser.parse(input) {
                Ok((new_input, o)) => {
                    input = new_input;
                    match o {
                        Some(o) => vec.push(o),
                        None => (),
                    }
                }
                Err(nom::Err::Failure(e)) => break Err(nom::Err::Failure(e)),
                Err(_) => break Ok((input, vec)),
            }
        }
    }
}

/// Parse a value of type `T` using `T`'s [FromStr] implementation by taking `n` characters from
/// the input string
pub fn fromstr_n<'a, T>(n: usize) -> impl FnMut(&'a str) -> ParseResult<&'a str, T>
where
    T: FromStr,
    crate::ParseError<&'a str>: FromExternalError<&'a str, <T as FromStr>::Err>,
{
    map_res(take(n), <T as FromStr>::from_str)
}

/// Produce a parser that consumes the output slice from `first`, using `T`'s [FromStr]
/// implementation to parse a value of type `T` from the produced output
pub fn fromstr_with<'a, T, P>(first: P) -> impl FnMut(&'a str) -> ParseResult<&'a str, T>
where
    T: FromStr,
    P: Parser<&'a str, &'a str, crate::ParseError<&'a str>>,
    crate::ParseError<&'a str>: FromExternalError<&'a str, <T as FromStr>::Err>,
{
    map_res(first, <T as FromStr>::from_str)
}
