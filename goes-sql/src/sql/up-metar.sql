CREATE TABLE IF NOT EXISTS weather.metar (
    data_id int UNSIGNED NOT NULL,
    country CHAR(4) NOT NULL,
    origin DATETIME NOT NULL,
    vwind_ex_ccw FLOAT,
    vwind_ex_cw FLOAT,
    visibility FLOAT,
    min_vis FLOAT,
    min_vis_dir ENUM(
        'N',
        'NE',
        'E',
        'SE',
        'S',
        'SW',
        'W',
        'NW'
    ),
    air_temp FLOAT,
    dewpoint_temp FLOAT,
    qnh FLOAT,
    runway_wind_shear_within FLOAT,
    CHECK(
        (vwind_ex_cw IS NULL OR vwind_ex_ccw IS NOT NULL) AND
        (vwind_ex_ccw IS NULL OR vwind_ex_cw IS NOT NULL) AND
        (
            (min_vis IS NULL OR min_vis_dir IS NOT NULL) AND
            (min_vis_dir IS NULL OR min_vis IS NOT NULL)
        ) AND
        (
            (air_temp IS NULL OR dewpoint_temp IS NOT NULL) AND
            (dewpoint_temp IS NULL OR air_temp IS NOT NULL)
        )
    ),
    CONSTRAINT `fk_metar_data`
        FOREIGN KEY (data_id) REFERENCES weather.data(id)
        ON DELETE CASCADE
        ON UPDATE RESTRICT
);

CREATE TABLE IF NOT EXISTS weather.metar_runway_state (
    data_id int UNSIGNED NOT NULL,
    runway SMALLINT NOT NULL,
    direction ENUM(
        'LEFT',
        'CENTER',
        'RIGHT'
    ),
    deposits ENUM(
        'CLEAR',
        'DAMP',
        'WET',
        'RIME_FROST',
        'DRY_SNOW',
        'WET_SNOW',
        'SLUSH',
        'ICE',
        'COMPACTED_SNOW',
        'FROZEN_RUTS',
        'NOT_REPORTED'
    ) NOT NULL,
    contamination_from FLOAT,
    contamination_to FLOAT,
    deposits_depth_status ENUM(
        'REPORTED',
        'INOPERABLE',
        'NOT_REPORTED'
    ) NOT NULL,
    deposits_depth FLOAT,
    braking_action_status ENUM(
        'COEFFICIENT',
        'POOR',
        'MEDIUM_POOR',
        'MEDIUM',
        'MEDIUM_GOOD',
        'GOOD',
        'UNRELIABLE',
        'NOT_REPORTED'
    ) NOT NULL,
    friction_coefficient FLOAT,
    CHECK(
        (deposits_depth_status!='REPORTED' OR deposits_depth IS NOT NULL) AND
        (braking_action_status!='COEFFICIENT' OR friction_coefficient IS NOT NULL)
    )

);

CREATE TABLE IF NOT EXISTS weather.metar_state_of_sea (
    data_id int UNSIGNED NOT NULL,
    temperature FLOAT NOT NULL,
    state ENUM(
        'GLASSY',
        'RIPPLED',
        'WAVELETS',
        'SLIGHT',
        'MODERATE',
        'ROUGH',
        'VERY_ROUGH',
        'HIGH',
        'VERY_HIGH',
        'PHENOMENAL'
    ) NOT NULL,
    CONSTRAINT `fk_metar_state_of_sea_data`
        FOREIGN KEY (data_id) REFERENCES weather.data(id)
        ON DELETE CASCADE
        ON UPDATE RESTRICT
);

CREATE TABLE IF NOT EXISTS weather.metar_wave_height (
    data_id int UNSIGNED NOT NULL,
    temperature FLOAT NOT NULL,
    height FLOAT NOT NULL,
    CONSTRAINT `fk_metar_wave_height_data`
        FOREIGN KEY (data_id) REFERENCES weather.data(id)
        ON DELETE CASCADE
        ON UPDATE RESTRICT
);

CREATE TABLE IF NOT EXISTS weather.metar_runway (
    data_id int UNSIGNED NOT NULL,
    runway SMALLINT NOT NULL,
    direction ENUM(
        'LEFT',
        'CENTER',
        'RIGHT'
    ),
    len FLOAT NOT NULL,
    trend ENUM(
        'CLOSER',
        'FARTHER',
        'NO_CHANGE'
    ),
    CONSTRAINT `fk_metar_runway_data`
        FOREIGN KEY (data_id) REFERENCES weather.data(id)
        ON UPDATE RESTRICT
        ON DELETE CASCADE
);
