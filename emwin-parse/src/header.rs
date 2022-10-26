use crate::dt::DataTypeDesignator;


/// A full AWIPS product identifier containing a WMO abbreviated heading and AFOS PIL
pub struct AWIPSProductIdentifer {
    pub wmo_abbreviated_heading: DataTypeDesignator,
}
