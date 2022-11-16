use std::path::Path;

use goes_parse::goes::{GoesFileName, SystemEnvironment, dsn::{Instrument, ProductAcronym, L2Acronym, ABISector, ABIMode}, Satellite};
use sqlx::Row;

use crate::GoesSqlContext;


impl GoesSqlContext {
    pub async fn insert_goes(&self, filename: GoesFileName, path: impl AsRef<Path>) -> Result<u64, sqlx::Error> {
        let id = sqlx::query(
r#"
INSERT INTO goesimg.files (env, instrument, acronym, channel, sector, abi_mode, satellite, start_dt, end_dt, file_name)
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING id;
"#
        )
        .bind(match filename.env {
            SystemEnvironment::OperationalRealTime => "OP_REALTIME",
            SystemEnvironment::OperationalTest => "OP_TEST",
            SystemEnvironment::TestRealTime => "TEST_REALTIME",
            SystemEnvironment::TestData => "TEST_DATA",
            SystemEnvironment::TestPlayback => "TEST_PLAYBACK",
            SystemEnvironment::TestSimulated => "TEST_SIMULATED",
        })
        .bind(match filename.dsn.instrument {
            Instrument::AdvancedBaselineImager => "ADVANCED_BASELINE_IMAGER",
            other => panic!("Unknown GOES instrument {:?}", other),
        })
        .bind(match filename.dsn.acronym {
            ProductAcronym::L1b(_) => "L1b",
            ProductAcronym::L2(l2) => match l2 {
                L2Acronym::CloudTopHeight => "CLOUD_TOP_HEIGHT",
                L2Acronym::CloudTopTemperature => "CLOUD_TOP_TEMPERATURE",
                L2Acronym::ClearSkyMasks => "CLEAR_SKY_MASKS",
                L2Acronym::CloudTopPhase => "CLOUD_TOP_PHASE",
                L2Acronym::AerosolOpticalDepth => "AEROSOL_OPTICAL_DEPTH",
                L2Acronym::CloudMoistureImagery(_) => "CLOUD_MOISTURE_IMAGERY",
                L2Acronym::MultibandCloudMoistureImagery => "MULTIBAND_CLOUD_MOISTURE_IMAGERY",
                L2Acronym::CloudOpticalDepth => "CLOUD_OPTICAL_DEPTH",
                L2Acronym::CloudParticleSizeDistribution => "CLOUD_PARTICLE_SIZE_DISTRIBUTION",
                L2Acronym::CloudTopPressure => "CLOUD_TOP_PRESSURE",
                L2Acronym::DerivedMotionWinds(_) => "DERIVED_MOTION_WIND",
                L2Acronym::DerivedMotionWindsBand8 => "DERIVED_MOTION_WIND_BAND8",
                L2Acronym::DerivedStabilityIndices => "DERIVED_STABILITY_INDEX",
                L2Acronym::DownwardShortwaveSurface => "DOWNWARD_SHORTWAVE_SURFACE",
                L2Acronym::FireHotCharacterization => "FIRE_HOT_CHARACTERIZATION",
                L2Acronym::SnowCover => "SNOW_COVER",
                L2Acronym::LandSkinTemperature => "LAND_SKIN_TEMPERATURE",
                L2Acronym::LegacyVerticalMoistureProfile => "LEGACY_VERTICAL_MOISTURE_PROFILE",
                L2Acronym::LegacyVerticalTemperatureProfile => "LEGACY_VERTICAL_TEMPERATURE_PROFILE",
                L2Acronym::RainfallRate => "RAINFALL_RATE",
                L2Acronym::ReflectedShortwave => "REFLECTED_SHORTWAVE",
                L2Acronym::SeaSkinTemperature => "SEA_SKIN_TEMPERATURE",
                L2Acronym::TotalPrecipitableWater => "TOTAL_PRECIPITABLE_WATER",
            }
        })
        .bind(filename.dsn.acronym.channel().map(|ch| *ch.as_ref()))
        .bind(match filename.dsn.sector {
            ABISector::FullDisk => "FULL_DISK",
            ABISector::CONUS => "CONUS",
            ABISector::Mesoscale1 => "MESOSCALE1",
            ABISector::Mesoscale2 => "MESOSCALE2",
        })
        .bind(match filename.dsn.mode {
            ABIMode::Mode3 => "3",
            ABIMode::Mode4 => "4",
            ABIMode::Mode6 => "6",
        })
        .bind(match filename.satellite {
            Satellite::Goes16 => "GOES16",
            Satellite::Goes17 => "GOES17",
            Satellite::Goes18 => "GOES18",
            Satellite::Goes19 => "GOES19",
        })
        .bind(filename.start)
        .bind(filename.end)
        .bind(path.as_ref().to_string_lossy())
        .fetch_one(&self.conn)
        .await?
        .try_get(0usize)?;

        Ok(id)
    }
}
