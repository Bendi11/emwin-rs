use chrono::{NaiveTime, Duration};
use nom::{combinator::map_res, bytes::complete::take, sequence::tuple, error::context};

use crate::ParseResult;


/// chrono format string for dates in YYGGgg format
const TIME_YYGGGG: &str = "%d%H%M";

/// Parse a time in DDHHMM format
pub fn yygggg(input: &str) -> ParseResult<&str, NaiveTime> {
    context(
        "time in YYGGgg format",
        map_res(
            take(6usize),
            |s: &str| NaiveTime::parse_from_str(s, TIME_YYGGGG),
        )
    )(input)
}

/// Parse a duration in YYGG (days-hours) format
pub fn yygg(input: &str) -> ParseResult<&str, NaiveTime> {
    fn parsenum(input: &str) -> ParseResult<&str, i64> {
        map_res(take(2usize), |s: &str| s.parse::<i64>())(input)
    }

    let (input, (days, hours)) = tuple((parsenum, parsenum))(input)?;

    Ok((
        input,
        NaiveTime::from_hms(0, 0, 0) + Duration::days(days) + Duration::hours(hours),
    ))
}
