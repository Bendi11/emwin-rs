use chrono::{Duration, NaiveTime};
use nom::{IResult, sequence::tuple, combinator::map_res, bytes::complete::take};

/// chrono format string for dates in YYGGgg format
pub const TIME_YYGGGG: &str = "%d%H%M";

/// Parse a duration in YYGG (days-hours) format
pub fn parse_yygg(input: &str) -> IResult<&str, NaiveTime> {
    fn parsenum(input: &str) -> IResult<&str, i64> {
        map_res(
            take(2usize),
            |s: &str| s.parse::<i64>(),
        )(input)
    }

    let (input, (days, hours)) = tuple((parsenum, parsenum))(input)?;

    Ok((input, NaiveTime::from_hms(0, 0, 0) + Duration::days(days) + Duration::hours(hours)))
}
