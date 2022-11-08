use chrono::Duration;
use nom::{bytes::complete::take, combinator::map_res, error::context, sequence::tuple, Parser};

use crate::ParseResult;

use super::fromstr;


/// Parse a time in DDHHMM format
pub fn yygggg(input: &str) -> ParseResult<&str, Duration> {
    context(
        "time in YYGGgg format",
        tuple((
            fromstr(2),
            fromstr(2),
            fromstr(2)
        ))
        .map(|(d, h, m)| 
            Duration::days(d) +
            Duration::hours(h) +
            Duration::minutes(m)
        )
    )(input)
}

/// Parse a duration in YYGG (days-hours) format
pub fn yygg(input: &str) -> ParseResult<&str, Duration> {
    fn parsenum(input: &str) -> ParseResult<&str, i64> {
        map_res(take(2usize), |s: &str| s.parse::<i64>())(input)
    }

    let (input, (days, hours)) = tuple((parsenum, parsenum))(input)?;

    Ok((
        input,
        Duration::days(days) + Duration::hours(hours),
    ))
}
