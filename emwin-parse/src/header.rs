use std::{str::FromStr, num::ParseIntError};

use chrono::{NaiveDateTime, NaiveTime};

use crate::dt::{DataTypeDesignator, DataTypeDesignatorParseError};

/// A full WMO product identifier with data type designator, country code, and timezone
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WMOProductIdentifier {
    pub dataype: DataTypeDesignator,
}

/// A full AWIPS product identifier containing a WMO abbreviated heading and AFOS PIL
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AWIPSProductIdentifer {
    pub wmo_abbreviated_heading: DataTypeDesignator,
}

/// Four-letter country code
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CCCC {
    pub code: [char ; 4],
}

/// A parsed EMWIN filename
#[derive(Clone, Debug)]
pub struct GoesFileName {
    pub wmo_product_id: DataTypeDesignator,
    pub country: CCCC,
    pub last_modify: NaiveTime,
    pub creation_timestamp: NaiveDateTime,
    pub sequence: u32,
    pub priority: u8,
}

fn expect<I: Iterator<Item = (usize, char)>>(iter: &mut I, ch: char) -> Result<(), GoesFileNameParseError> {
    let next = iter.next().ok_or_else(|| GoesFileNameParseError::Length)?.1;
    if ch != next {
        Err(GoesFileNameParseError::Unexpected(next, ch))
    } else {
        Ok(())
    }
}

fn expect_str<I: Iterator<Item = (usize, char)>>(iter: &mut I, st: &str) -> Result<(), GoesFileNameParseError> {
    for ch in st.chars() {
        expect(iter, ch)?;
    }

    Ok(())
}

trait Expect {
    type Output;
    fn require(self) -> Result<Self::Output, GoesFileNameParseError>;
}

impl<T> Expect for Option<T> {
    type Output = T;
    fn require(self) -> Result<Self::Output, GoesFileNameParseError> {
        self.ok_or_else(|| GoesFileNameParseError::Length)
    }
}

impl FromStr for GoesFileName {
    type Err = GoesFileNameParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.char_indices().peekable();
        
        match chars.next().require()?.1 {
            'A' | 'Z' => (),
            other => return Err(GoesFileNameParseError::Unexpected(other, 'A')),
        }

        expect(&mut chars, '_')?;

        for _ in 0..6 {
            chars.next().require()?;
        }

        let wmo_product_id: DataTypeDesignator = s[2..][..6].parse()?;

        let mut country = ['\0', '\0', '\0', '\0'];
        for i in 0..4 {
            country[i] = chars.next().require()?.1;
        }

        for _ in 0..6 {
            chars.next().require()?;
        }
        
        let last_modify = NaiveTime::parse_from_str(&s[12..][..6], "%d%H%M")?;
        
        //skip [BBB]
        match chars.next().require()?.1 {
            '_' => (),
            _ => {
                for _ in 0..3 {
                    chars.next().require()?;
                }
            }
        }

        expect_str(&mut chars, "C_KWIN_")?;
        let ts_idx = chars.peek().require()?.0;
        
        for _ in 0..14 {
            chars.next().require()?;
        }

        let creation_timestamp = NaiveDateTime::parse_from_str(&s[ts_idx..][..14], "%Y%m%d%H%M%S")?;
        let seq_idx = ts_idx + 15;

        expect(&mut chars, '_')?;
        for _ in 0..6 {
            chars.next().require()?;
        }

        let sequence: u32 = s[seq_idx..][..6].parse()?;
        expect(&mut chars, '-')?;
        let priority = chars.next().require()?.1;
        let priority = priority.to_digit(10).ok_or_else(|| GoesFileNameParseError::Priority(priority))? as u8;

        Ok(Self {
            wmo_product_id,
            country: CCCC { code: country },
            last_modify,
            creation_timestamp,
            sequence,
            priority,
        })
    }
}


#[derive(Clone, Debug, thiserror::Error)]
pub enum GoesFileNameParseError {
    #[error("Failed to parse WMO product ID from: {0}")]
    WMOParse(#[from] DataTypeDesignatorParseError),
    #[error("Failed to parse file creation timestamp: {0}")]
    DateTime(#[from] chrono::ParseError),
    #[error("Failed to parse sequence number: {0}")]
    SeqNum(#[from] ParseIntError),
    #[error("Priority {0} is not a digit or out of priority range")]
    Priority(char),
    #[error("Unexpected character {0}, expecting {1} in filename")]
    Unexpected(char, char),
    #[error("Goes filename is not the correct length")]
    Length,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_filename() {
        let filename: GoesFileName = "A_FXUS65KABQ121804AAB_C_KWIN_20160112180901_008996-2-AFDABQNM.TXT".parse().unwrap();
        assert!(filename.priority == 2);
        assert!(filename.sequence == 8996);
        assert!(matches!(filename.wmo_product_id, DataTypeDesignator::Forecast(_)));
    }
}
