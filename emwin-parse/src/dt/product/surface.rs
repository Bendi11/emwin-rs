use crate::dt::{area::AreaCode, DataTypeDesignatorParseError, UnparsedProductIdentifier};

/// S
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceData {
    /// T2
    pub subtype: SurfaceSubType,
    /// A1A2
    pub area: AreaCode,
    /// ii
    pub enumerator: u8,
}

/// Term T2 definition when T1=Surface
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceSubType {
    AviationRoutineReport,
    RadarReportA,
    RadarReportB,
    RadarReportAB,
    Seismic,
    AtmosphericReport,
    RadiologicalDataReport,
    DCPStationReport,
    IntermediateSynopticHour,
    MainSynopticHour,
    NonstandardSynopticHour,
    OceanographicData,
    SpecialAviationWeatherReport,
    HydrologicalRiverReport,
    DriftingBuoyReport,
    SeaIce,
    SnowDepth,
    LakeIce,
    WaveInformation,
    Misc,
    SeismicWaveformData,
    SeaLevelDeepOceanTsunamiData,
}

impl TryFrom<UnparsedProductIdentifier> for SurfaceData {
    type Error = DataTypeDesignatorParseError;
    fn try_from(value: UnparsedProductIdentifier) -> Result<Self, Self::Error> {
        Ok(Self {
            subtype: match value.t2 {
                'A' => SurfaceSubType::AviationRoutineReport,
                'B' => SurfaceSubType::RadarReportA,
                'C' => SurfaceSubType::RadarReportB,
                'D' => SurfaceSubType::RadarReportAB,
                'E' => SurfaceSubType::Seismic,
                'F' => SurfaceSubType::AtmosphericReport,
                'G' => SurfaceSubType::RadiologicalDataReport,
                'H' => SurfaceSubType::DCPStationReport,
                'I' => SurfaceSubType::IntermediateSynopticHour,
                'M' => SurfaceSubType::MainSynopticHour,
                'N' => SurfaceSubType::NonstandardSynopticHour,
                'O' => SurfaceSubType::OceanographicData,
                'P' => SurfaceSubType::SpecialAviationWeatherReport,
                'R' => SurfaceSubType::HydrologicalRiverReport,
                'S' => SurfaceSubType::DriftingBuoyReport,
                'T' => SurfaceSubType::SeaIce,
                'U' => SurfaceSubType::SnowDepth,
                'V' => SurfaceSubType::LakeIce,
                'W' => SurfaceSubType::WaveInformation,
                'X' => SurfaceSubType::Misc,
                'Y' => SurfaceSubType::SeismicWaveformData,
                'Z' => SurfaceSubType::SeaLevelDeepOceanTsunamiData,
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2('S', other)),
            },
            area: AreaCode::try_from((value.a1, value.a2))?,
            enumerator: value.ii,
        })
    }
}
