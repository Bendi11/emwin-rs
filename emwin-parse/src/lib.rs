use header::AWIPSProductIdentifer;
use nom::IResult;
use nom_supreme::error::ErrorTree;

pub type ParseError<I> = ErrorTree<I>;
pub type ParseResult<I, O> = IResult<I, O, ParseError<I>>;

pub mod dt;
pub mod formats;
pub mod header;
mod util;

fn display_error(e: nom::Err<ParseError<&str>>) -> String {
    match e {
        nom::Err::Error(e) | nom::Err::Failure(e) => e.map_locations(|s| match s.find('\n') {
            Some(idx) => &s[..idx],
            None => s,
        }).to_string(),
        e => e.to_string(),
    }
}
