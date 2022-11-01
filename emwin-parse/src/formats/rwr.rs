//! Parsing for the (unspecified?) Regional Weather Roundup format

use std::str::FromStr;

use nom::{
    bytes::complete::{tag, take, take_while},
    character::complete::{anychar, multispace1, space0, space1, char},
    combinator::{map_res, opt},
    error::ErrorKind,
    multi::many_till,
    sequence::{preceded, terminated},
    IResult,
};

use crate::{dt::area::AreaCode, header::WMOProductIdentifier};

#[derive(Clone, Debug)]
pub struct RegionalWeatherRoundup {
    pub header: WMOProductIdentifier,
    pub area: AreaCode,
    pub reports: Vec<RegionalWeatherRoundupItem>,
}

#[derive(Clone, Debug)]
pub struct RegionalWeatherRoundupItem {
    pub city: String,
    pub sky: RegionalWeatherSkyCondition,
    pub temperature: i32,
    pub dew_point: i32,
    pub relative_humidity: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegionalWeatherSkyCondition {
    NA,
    LightRain,
    Drizzle,
    Cloudy,
    PartlySunny,
    MostlySunny,
    Sunny,
    Fair,
    Clear,
}

impl RegionalWeatherRoundup {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, header) = terminated(WMOProductIdentifier::parse, multispace1)(input)?;

        let (input, area) = preceded(
            tag("RWR"),
            map_res(take(2usize), |code: &str| code.parse::<AreaCode>()),
        )(input)?;

        let mut reports = vec![];
        let mut city = false;

        for line in input.lines().filter(|line| line.len() > 2) {
            match city {
                false => {
                    if line.starts_with("CITY") {
                        city = true
                    }
                }
                true => {
                    if line.starts_with("$$") {
                        city = false
                    } else {
                        match RegionalWeatherRoundupItem::parse(line) {
                            Ok((_, item)) => reports.push(item),
                            Err(e) => {
                                log::warn!("Failed to parse RWR item: {}", e);
                            }
                        }
                    }
                }
            }
        }

        Ok((
            "",
            Self {
                header,
                area,
                reports,
            },
        ))
    }
}

impl RegionalWeatherRoundupItem {
    /// Parse a single weather report from one line
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (city_parts, sky)) = preceded(
            opt(char('*')),
            many_till(
                anychar,
                preceded(space0, RegionalWeatherSkyCondition::parse),
            ),
        )(input)?;

        let city = city_parts.into_iter().collect::<String>();

        let mut num = preceded(
            space1,
            map_res(take_while(|c: char| !c.is_whitespace()), |s: &str| s.parse::<i32>()),
        );

        let (input, temperature) = num(input)?;
        let (input, dew_point) = num(input)?;
        let (input, relative_humidity) = num(input)?;

        Ok((
            input,
            Self {
                city,
                sky,
                temperature,
                dew_point,
                relative_humidity,
            },
        ))
    }
}

impl RegionalWeatherSkyCondition {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, designator) = take_while(|c: char| !c.is_whitespace())(input)?;

        Ok((
            rest,
            Self::from_str(designator).map_err(|_| {
                nom::Err::Error(nom::error::Error {
                    input: designator,
                    code: ErrorKind::Fail,
                })
            })?,
        ))
    }
}

impl FromStr for RegionalWeatherSkyCondition {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "CLOUDY" => Self::Cloudy,
            "PTSUNNY" => Self::PartlySunny,
            "MOSUNNY" => Self::MostlySunny,
            "SUNNY" => Self::Sunny,
            "DRIZZLE" => Self::Drizzle,
            "FAIR" => Self::Fair,
            "CLEAR" => Self::Clear,
            "N/A" => Self::NA,
            _ => return Err(()),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_rwr() {
        let _ = RegionalWeatherRoundup::parse(EX_RWR).unwrap_or_else(|e| panic!("{}", e));
    }

    const EX_RWR: &str = include_str!("test/rwr.txt");
}
