use header::AWIPSProductIdentifer;
use nom::IResult;
use nom_supreme::error::ErrorTree;

pub type ParseError<I> = ErrorTree<I>;
pub type ParseResult<I, O> = IResult<I, O, ParseError<I>>;

pub mod dt;
pub mod formats;
pub mod header;
mod util;
