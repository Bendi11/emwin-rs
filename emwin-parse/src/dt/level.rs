

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AirLevelDesignator(u8);


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SeaLevelDesignator(u8);

impl TryFrom<u8> for AirLevelDesignator {
    type Error = InvalidAirLevelDesignator;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(Self(value)) 
    }
}

impl TryFrom<u8> for SeaLevelDesignator {
    type Error = InvalidSeaLevelDesignator;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum InvalidAirLevelDesignator {
    #[error("Invalid air level designator {0}")]
    Invalid(u8),
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum InvalidSeaLevelDesignator {
    #[error("Invalid sea level designator {0}")]
    Invalid(u8),
}


