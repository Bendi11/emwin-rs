use std::str::FromStr;

use chrono::{NaiveDateTime, Timelike};
use nom::{character::streaming::char, sequence::{preceded, pair}, Parser, combinator::{map_res, map_opt}, bytes::complete::take};
use nom_supreme::tag::complete::tag;

use crate::{ParseResult, parse::fromstr};

use self::dsn::DataShortName;

pub mod dsn;


/// Two-letter system environment code specifying if a GOES image was received from a test or
/// real-time data transmission
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SystemEnvironment {
    OperationalRealTime,
    OperationalTest,
    TestRealTime,
    TestData,
    TestPlayback,
    TestSimulated,
}

/// Enumeration representing all GOES-R series satellites
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Satellite {
    Goes16,
    Goes17,
    Goes18,
    Goes19,
}

/// GOES-R series file name
#[derive(Clone, Copy, Debug,)]
pub struct GoesFileName {
    pub env: SystemEnvironment,
    pub dsn: DataShortName,
    pub satellite: Satellite,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub creation: NaiveDateTime,
}

impl GoesFileName {
    pub fn parse(input: &str) -> ParseResult<&str, Self> {
        let (input, env) = fromstr::<SystemEnvironment>(2).parse(input)?;
        let (input, dsn) = preceded(char('_'), DataShortName::parse)(input)?;
        let (input, satellite) = preceded(char('_'), fromstr::<Satellite>(3))(input)?;

        let (input, start) = preceded(tag("_s"), Self::timestamp)(input)?;
        let (input, end) = preceded(tag("_e"), Self::timestamp)(input)?;
        let (input, creation) = preceded(tag("_c"), Self::timestamp)(input)?;

        Ok((
            input,
            Self {
                env,
                dsn,
                satellite,
                start,
                end,
                creation,
            }
        ))
    }

    fn timestamp(input: &str) -> ParseResult<&str, NaiveDateTime> {
        map_opt(
            pair(
                map_res(
                    take(13usize),
                    |s| NaiveDateTime::parse_from_str(
                        s,
                        "%Y%j%H%M%S"
                    )
                ),
                fromstr::<u32>(1),
            ),
            |(dt, n)| dt.with_nanosecond(n * 1e+8 as u32),
        )
        .parse(input)
    }
}

#[derive(Clone, Copy, Debug, thiserror::Error)]
#[error("Invalid GOES system environment code")]
pub struct InvalidSystemEnvironment;

impl FromStr for SystemEnvironment {
    type Err = InvalidSystemEnvironment;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "OR" => Self::OperationalRealTime,
            "OT" => Self::OperationalTest,
            "IR" => Self::TestRealTime,
            "IT" => Self::TestData,
            "IP" => Self::TestPlayback,
            "IS" => Self::TestSimulated,
            _ => return Err(InvalidSystemEnvironment),
        })
    }
}

#[derive(Clone, Copy, Debug, thiserror::Error)]
#[error("Invalid GOES-R series satellite identifier")]
pub struct InvalidSatelliteIdentifier;

impl FromStr for Satellite {
    type Err = InvalidSatelliteIdentifier;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "G16" => Self::Goes16,
            "G17" => Self::Goes17,
            "G18" => Self::Goes18,
            "G19" => Self::Goes19,
            _ => return Err(InvalidSatelliteIdentifier),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::goes::dsn::{Instrument, ProductAcronym, Channel, ABISector};

    use super::*;
    
    const GOES1: &str = "OR_ABI-L1b-RadF-M6C13_G17_s20210481330321_e20210481339399_c20210481339454.nc";

    #[test]
    fn test_goesr_fn() {
        let (_, goes1) = GoesFileName::parse(GOES1)
            .unwrap_or_else(|e| panic!("{}", crate::display_error(e))); 
        assert_eq!(goes1.env, SystemEnvironment::OperationalRealTime);
        assert_eq!(
            goes1.dsn,
            DataShortName {
                instrument: Instrument::AdvancedBaselineImager,
                acronym: ProductAcronym::L1b(Channel::new(13)),
                sector: ABISector::FullDisk,
                mode: dsn::ABIMode::Mode6,
            }
        );
        assert_eq!(
            goes1.satellite,
            Satellite::Goes17,
        );
    }
}
