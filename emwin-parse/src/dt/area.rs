
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
/// Area code for D, G, H, O, P, Q, T, X or Y or A2 for I and J
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

impl TryFrom<char> for GeographicalAreaDesignator {
    type Error = GeographicalAreaDesignatorParseError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'A' => Self::ZeroToNinetyWest(GeographicalAreaDesignatorHemisphere::NorthernHemisphere),
            'B' => Self::NinetyToOneEightyWest(GeographicalAreaDesignatorHemisphere::NorthernHemisphere),
            'C' => Self::OneEightytoNinetyEast(GeographicalAreaDesignatorHemisphere::NorthernHemisphere),
            'D' => Self::NinetyToZeroEast(GeographicalAreaDesignatorHemisphere::NorthernHemisphere),
            'E' => Self::ZeroToNinetyWest(GeographicalAreaDesignatorHemisphere::TropicalBelt),
            'F' => Self::NinetyToOneEightyWest(GeographicalAreaDesignatorHemisphere::TropicalBelt),
            'G' => Self::OneEightytoNinetyEast(GeographicalAreaDesignatorHemisphere::TropicalBelt),
            'H' => Self::NinetyToZeroEast(GeographicalAreaDesignatorHemisphere::TropicalBelt),
            'I' => Self::ZeroToNinetyWest(GeographicalAreaDesignatorHemisphere::SouthernHemisphere),
            'J' => Self::NinetyToOneEightyWest(GeographicalAreaDesignatorHemisphere::SouthernHemisphere),
            'K' => Self::OneEightytoNinetyEast(GeographicalAreaDesignatorHemisphere::SouthernHemisphere),
            'L' => Self::NinetyToZeroEast(GeographicalAreaDesignatorHemisphere::SouthernHemisphere),
            'N' => Self::Hemisphere(GeographicalAreaDesignatorHemisphere::NorthernHemisphere),
            'S' => Self::Hemisphere(GeographicalAreaDesignatorHemisphere::SouthernHemisphere),
            'T' => Self::FortyFiveToOneEightyWestNorthernHemisphere,
            'X' => Self::Global,
            other => return Err(GeographicalAreaDesignatorParseError::Invalid(other)),
        })
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum GeographicalAreaDesignatorParseError {
    #[error("Unrecognized geographical area designator {0}")]
    Invalid(char),
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum AreaCodeParseError {
    #[error("Unrecognized area code {0}{1}")]
    Invalid(char, char),
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum ReferenceTimeDesignatorParseError {
    #[error("Unrecognized reference time designator {0}")]
    Invalid(char),
}
