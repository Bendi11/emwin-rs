use crate::dt::{area::{GeographicalAreaDesignator, ReferenceTimeDesignator}, UnparsedProductIdentifier, DataTypeDesignatorParseError};

/// D, G, H, Y
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridPointInformation {
    /// T2
    pub subtype: GridPointSubType,
    /// A1
    pub area: GeographicalAreaDesignator,
    /// A2
    pub time: ReferenceTimeDesignator,
}

/// Term T2 definitions when T1=GridPointInformation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GridPointSubType {
    RadarData,
    Cloud,
    Vorticity,
    Thickness,
    Precipitation,
    Divergence,
    Height,
    WaveHeight,
    SwellHeight,
    NationalUse,
}

impl TryFrom<UnparsedProductIdentifier> for GridPointInformation {
    type Error = DataTypeDesignatorParseError;
    fn try_from(value: UnparsedProductIdentifier) -> Result<Self, Self::Error> {
        Ok(Self {
            subtype: match value.t2 {
                'A' => GridPointSubType::RadarData,
                'B' => GridPointSubType::Cloud,
                'C' => GridPointSubType::Vorticity,
                'D' => GridPointSubType::Thickness,
                'E' => GridPointSubType::Precipitation,
                'G' => GridPointSubType::Divergence,
                'H' => GridPointSubType::Height,
                'J' => GridPointSubType::WaveHeight,
                'K' => GridPointSubType::SwellHeight,
                'M' => GridPointSubType::NationalUse,
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2(value.t1, other)),
            },
            area: GeographicalAreaDesignator::try_from(value.a1)?,
            time: match value.t1 {
                'Y' => ReferenceTimeDesignator::parse_for_qxy(value.a2),
                _ => ReferenceTimeDesignator::parse_for_dghjopt(value.a2),
            }?,
        })
    }
}
