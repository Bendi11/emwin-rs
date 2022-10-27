use crate::dt::{UnparsedProductIdentifier, DataTypeDesignatorParseError, area::AreaCode};



#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Surface {
    pub subtype: SurfaceSubType,
    pub area: AreaCode,
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

impl TryFrom<UnparsedProductIdentifier> for Surface {
    type Error = DataTypeDesignatorParseError;
    fn try_from(value: UnparsedProductIdentifier) -> Result<Self, Self::Error> {
        Ok(Self {
            subtype: match value.t2 {
                'A' => SurfaceT2::AviationRoutineReport,
                'B' => SurfaceT2::RadarReportA,
                'C' => SurfaceT2::RadarReportB,
                'D' => SurfaceT2::RadarReportAB,
                'E' => SurfaceT2::Seismic,
                'F' => SurfaceT2::AtmosphericReport,
                'G' => SurfaceT2::RadiologicalDataReport,
                'H' => SurfaceT2::DCPStationReport,
                'I' => SurfaceT2::IntermediateSynopticHour,
                'M' => SurfaceT2::MainSynopticHour,
                'N' => SurfaceT2::NonstandardSynopticHour,
                'O' => SurfaceT2::OceanographicData,
                'P' => SurfaceT2::SpecialAviationWeatherReport,
                'R' => SurfaceT2::HydrologicalRiverReport,
                'S' => SurfaceT2::DriftingBuoyReport,
                'T' => SurfaceT2::SeaIce,
                'U' => SurfaceT2::SnowDepth,
                'V' => SurfaceT2::LakeIce,
                'W' => SurfaceT2::WaveInformation,
                'X' => SurfaceT2::Misc,
                'Y' => SurfaceT2::SeismicWaveformData,
                'Z' => SurfaceT2::SeaLevelDeepOceanTsunamiData,
                other => return Err(DataTypeDesignatorParseError::UnrecognizedT2('S', other)),
            },
            area: AreaCode::try_from((value.a1, value.a2))?,
        })
    }
}
