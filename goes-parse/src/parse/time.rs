use nom::{bytes::complete::take, combinator::map_res, error::context, sequence::tuple, Parser};

use crate::ParseResult;

use super::fromstr_n;

/// A structure holding naive days, hours, and minutes into the month
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DayHourMinute {
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
}

impl DayHourMinute {
    pub fn offset<D: chrono::Datelike + chrono::Timelike>(&self, month: D) -> Option<D> {
        month
            .with_day(self.day as u32)?
            .with_hour(self.hour as u32)?
            .with_minute(self.minute as u32)
    }
}

/// Parse a time in DDHHMM format
pub fn yygggg(input: &str) -> ParseResult<&str, DayHourMinute> {
    context(
        "time in YYGGgg format",
        tuple((
            context("YY", fromstr_n(2)),
            context("GG", fromstr_n(2)),
            context("gg", fromstr_n(2)),
        ))
        .map(|(day, hour, minute)| DayHourMinute { day, hour, minute }),
    )(input)
}

/// Parse a duration in YYGG (days-hours) format
pub fn yygg(input: &str) -> ParseResult<&str, DayHourMinute> {
    fn parsenum(input: &str) -> ParseResult<&str, u8> {
        map_res(take(2usize), |s: &str| s.parse::<u8>())(input)
    }

    let (input, (day, hour)) = tuple((parsenum, parsenum))(input)?;

    Ok((
        input,
        DayHourMinute {
            day,
            hour,
            minute: 0,
        },
    ))
}
