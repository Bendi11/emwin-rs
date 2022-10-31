use crate::dt::DataTypeDesignator;

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

/// A parsed EMWIN filename
#[derive(Clone, Debug)]
pub struct GoesRFileName {
    pub wmo_product_id: AWIPSProductIdentifer,
}
