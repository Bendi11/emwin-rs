CREATE DATABASE IF NOT EXISTS weather;


CREATE TABLE IF NOT EXISTS weather.data (
    id int UNSIGNED NOT NULL PRIMARY KEY AUTO_INCREMENT
);

CREATE TABLE IF NOT EXISTS weather.cloud_report (
    data_id int UNSIGNED NOT NULL,
    amount ENUM(
        'FEW',
        'SCATTERED',
        'BROKEN',
        'OVERCAST'
    ),
    altitude FLOAT NOT NULL,
    CONSTRAINT `fk_cloud_report_data`
        FOREIGN KEY (data_id) REFERENCES weather.data (id)
        ON DELETE CASCADE
        ON UPDATE RESTRICT
);

CREATE TABLE IF NOT EXISTS weather.wind_summary (
    data_id int UNSIGNED NOT NULL,
    angle FLOAT NOT NULL,
    speed FLOAT NOT NULL,
    max_speed FLOAT,
    CONSTRAINT `taf_wind_data_id`
        FOREIGN KEY (data_id) REFERENCES weather.data (id)
        ON DELETE CASCADE
        ON UPDATE RESTRICT
);

CREATE TABLE IF NOT EXISTS weather.significant_weather (
    data_id int UNSIGNED NOT NULL,
    intensity ENUM('LIGHT', 'MODERATE', 'HEAVY', 'VICINITY') NOT NULL,
    descriptor ENUM(
        'SHALLOW',
        'PATCHES',
        'PARTIAL',
        'LOW_DRIFTING',
        'BLOWING',
        'SHOWERS',
        'THUNDERSTORM',
        'SUPERCOOLED'
    ),
    precipitation SET(
        'DRIZZLE',
        'RAIN',
        'SNOW',
        'SNOWGRAIN',
        'ICEPELLET',
        'HAIL',
        'SMALLHAIL',
        'UNKNOWN'
    ) NOT NULL,
    phenomena ENUM (
        'MIST',
        'FOG',
        'SMOKE',
        'ASH',
        'DUST',
        'SAND',
        'HAZE',
        'DUST_SANDSWIRLS',
        'SQUALLS',
        'FUNNEL_CLOUD',
        'SANDSTORM',
        'DUSTSTORM'
    ),
    CONSTRAINT `fk_significant_weather_data`
        FOREIGN KEY (data_id) REFERENCES weather.data (id)
        ON DELETE CASCADE
        ON UPDATE RESTRICT
);

CREATE TABLE IF NOT EXISTS weather.taf_item (
    id int UNSIGNED NOT NULL PRIMARY KEY AUTO_INCREMENT,
    month DATETIME NOT NULL,
    country CHAR(4) NOT NULL,
    origin_off int UNSIGNED NOT NULL,
    from_off int UNSIGNED NOT NULL,
    to_off int UNSIGNED NOT NULL,
    visibility FLOAT,
    data_id int UNSIGNED NOT NULL UNIQUE,
    CONSTRAINT `fk_taf_item_data`
        FOREIGN KEY (data_id) REFERENCES weather.data (id)
        ON DELETE CASCADE
        ON UPDATE RESTRICT
);


CREATE TABLE IF NOT EXISTS weather.taf_group (
    item_id int UNSIGNED NOT NULL,
    data_id int UNSIGNED NOT NULL,
    kind ENUM('TIMED', 'CHANGE', 'TEMP', 'PROB') NOT NULL,
    from_off int UNSIGNED NOT NULL,
    to_off int UNSIGNED,
    probability FLOAT,
    visibility FLOAT,
    CHECK (
        (kind='TIMED'  AND to_off IS     NULL AND probability IS     NULL) OR
        (kind='CHANGE' AND to_off IS NOT NULL AND probability IS     NULL) OR
        (kind='TEMP'   AND to_off IS NOT NULL AND probability IS NOT NULL) OR
        (kind='PROB'   AND to_off IS NOT NULL AND probability IS NOT NULL)
    ),
    CONSTRAINT `fk_taf_group_item`
        FOREIGN KEY (item_id) REFERENCES weather.taf_item (id)
        ON DELETE CASCADE
        ON UPDATE RESTRICT,
    CONSTRAINT `fk_taf_group_data`
        FOREIGN KEY (data_id) REFERENCES weather.data (id)
        ON DELETE CASCADE
        ON UPDATE RESTRICT
);

CREATE DATABASE IF NOT EXISTS goesimg;
CREATE TABLE IF NOT EXISTS goesimg.files (
    id int UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
    env ENUM(
        'OP_REALTIME',
        'OP_TEST',
        'TEST_REALTIME',
        'TEST_DATA',
        'TEST_PLAYBACK',
        'TEST_SIMULATED'
    ) NOT NULL,
    instrument ENUM(
        'ADVANCED_BASELINE_IMAGER'
    ) NOT NULL,
    acronym ENUM(
        'L1b',
        'CLOUD_TOP_HEIGHT',
        'CLOUD_TOP_TEMPERATURE',
        'CLEAR_SKY_MASK',
        'CLOUD_TOP_PHASE',
        'AEROSOL_OPTICAL_DEPTH',
        'CLOUD_MOISTURE_IMAGERY',
        'MULTIBAND_CLOUD_MOISTURE_IMAGERY',
        'CLOUD_OPTICAL_DEPTH',
        'CLOUD_PARTICLE_SIZE_DISTRIBUTION',
        'CLOUD_TOP_PRESSURE',
        'DERIVED_MOTION_WIND',
        'DERIVED_MOTION_WIND_BAND8',
        'DERIVED_STABILITY_INDEX',
        'DOWNWARD_SHORTWAVE_SURFACE',
        'FIRE_HOT_CHARACTERIZATION',
        'SNOW_COVER',
        'LAND_SKIN_TEMPERATURE',
        'LEGACY_VERTICAL_MOISTURE_PROFILE',
        'LEGACY_VERTICAL_TEMPERATURE_PROFILE',
        'RAINFALL_RATE',
        'REFLECTED_SHORTWAVE',
        'SEA_SKIN_TEMPERATURE',
        'TOTAL_PRECIPITABLE_WATER'
    ),
    channel TINYINT,
    sector ENUM(
        'FULL_DISK',
        'CONUS',
        'MESOSCALE1',
        'MESOSCALE2'
    ) NOT NULL,
    abi_mode ENUM(
        '3',
        '4',
        '6'
    ) NOT NULL,
    satellite ENUM(
        'GOES16',
        'GOES17',
        'GOES18',
        'GOES19'
    ) NOT NULL,
    start_dt DATETIME NOT NULL,
    end_dt DATETIME NOT NULL,
    file_name VARCHAR(255) NOT NULL,
    CHECK (
        (channel IS NULL OR (channel>=1 AND CHANNEL <=16)) AND
        (
            (
                acronym!='CLOUD_MOISTURE_IMAGERY' OR
                acronym!='DERIVED_MOTION_WIND' OR
                acronym!='L1b'
            ) OR
            channel IS NOT NULL
        )
    ),
);
