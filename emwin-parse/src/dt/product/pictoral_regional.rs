use crate::dt::area::GeographicalAreaDesignator;

use super::pictoral::PictoralInformationSubType;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RegionalPictoralInformation {
    /// T2
    pub subtype: PictoralInformationSubType,
    /// A1
    pub area: GeographicalAreaDesignator,
}
