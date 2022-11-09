
CREATE TABLE IF NOT EXISTS weather.data (
    id int UNSIGNED NOT NULL PRIMARY KEY AUTO_INCREMENT
);

CREATE TABLE IF NOT EXISTS weather.cloud_report (
    data_id int UNSIGNED NOT NULL,
    amount ENUM(
        'FEW',
        'SCATTERED',
        'BROKEN',
        'OVERCAST',
    ),
    altitude FLOAT NOT NULL,
    CONSTRAINT `fk_cloud_report_data`
        FOREIGN KEY (data_id) REFERENCES weather.data (id)
        ON DELETE CASCADE
        ON UPDATE RESTRICT
);


CREATE TABLE IF NOT EXISTS weather.taf_visibility (
    data_id int UNSIGNED NOT NULL,
    horizontal_visibility FLOAT,
    CONSTRAINT `fk_taf_visibility_data`
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
    kind ENUM('REPORT', 'AMENDMENT', 'CORRECTION') NOT NULL,
    country CHAR(4) NOT NULL,
    origin DATETIME NOT NULL,
    from_dt DATETIME NOT NULL,
    to_dt DATETIME NOT NULL,
    data_id int UNSIGNED NOT NULL,
    CONSTRAINT `fk_taf_item_data`
        FOREIGN KEY (data_id) REFERENCES weather.data (id)
        ON DELETE CASCADE
        ON UPDATE RESTRICT
);


CREATE TABLE IF NOT EXISTS weather.taf_group (
    item_id int UNSIGNED NOT NULL,
    data_id int UNSIGNED NOT NULL,
    kind ENUM('TIMED', 'CHANGE', 'TEMP', 'PROB') NOT NULL,
    from_dt DATETIME NOT NULL,
    to_dt DATETIME,
    probability FLOAT,
    CHECK (
        (kind='TIMED'  AND to_dt IS     NULL AND probability IS     NULL) OR
        (kind='CHANGE' AND to_dt IS NOT NULL AND probability IS     NULL) OR
        (kind='TEMP'   AND to_dt IS NOT NULL AND probability IS NOT NULL) OR
        (kind='PROB'   AND to_dt IS NOT NULL AND probability IS NOT NULL)
    ),
    CONSTRAINT `fk_taf_group_taf_item` 
        FOREIGN KEY (item_id) REFERENCES weather.taf_item (id)
        ON DELETE CASCADE
        ON UPDATE RESTRICT,
    CONSTRAINT `fk_taf_group_data`
        FOREIGN KEY (data_id) REFERENCES weather.data (id)
        ON DELETE CASCADE
        ON UPDATE RESTRICT
);
