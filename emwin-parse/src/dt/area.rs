
/// From WMO No. 386 P. 88
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AreaCode(char, char);

/// A hemisphere for use in a [GeographicalAreaDesignator]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeographicalAreaDesignatorHemisphere {
    NorthernHemisphere,
    TropicalBelt,
    SouthernHemisphere,
}

/// From WMO No. 386 P. 92
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeographicalAreaDesignator {
    ZeroToNinetyWest(GeographicalAreaDesignatorHemisphere),
    NinetyToOneEightyWest(GeographicalAreaDesignatorHemisphere),
    OneEightytoNinetyEast(GeographicalAreaDesignatorHemisphere),
    NinetyToZeroEast(GeographicalAreaDesignatorHemisphere),
    Hemisphere(GeographicalAreaDesignatorHemisphere),
    FortyFiveToOneEightyWestNorthernHemisphere,
    Global,
}

/// Reference times to be used for forecasts
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReferenceTimeDesignator(u16);

impl ReferenceTimeDesignator {
    /// Get the number of hours in this designator
    pub const fn hours(&self) -> u16 {
        self.0
    }
    
    /// Create a new `ReferenceTimeDesignator` from a given number of hours
    pub const fn from_hours(hours: u16) -> Self {
        Self(hours)
    }
    
    /// Create a new `ReferenceTimeDesignator` from a given number of days
    pub const fn from_days(days: u16) -> Self {
        Self(days * 24)
    }
}

impl TryFrom<(char, char)> for AreaCode {
    type Error = AreaCodeParseError;

    fn try_from(value: (char, char)) -> Result<Self, Self::Error> {
        Ok(Self(value.0, value.1)) 
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum AreaCodeParseError {
    #[error("Unrecognized area code {0}{1}")]
    Invalid(char, char),
}
