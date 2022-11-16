use std::sync::Arc;

use chrono::Datelike;
use goes_parse::{
    dt::{
        code::CodeForm,
        product::{Analysis, Forecast},
        upperair::UpperAirData,
        AnalysisSubType, DataTypeDesignator, ForecastSubType, UpperAirDataSubType,
    },
    formats::{amdar::AmdarReport, rwr::RegionalWeatherRoundup, taf::TAFReport},
    header::GoesEmwinFileName, goes::GoesFileName, display_error,
};
use goes_sql::GoesSqlContext;
use notify::Event;

use crate::config::Config;

pub async fn emwin_dispatch(event: Event, ctx: Arc<GoesSqlContext>, config: Arc<Config>) {
    for path in event.paths {
        match path.file_stem().map(std::ffi::OsStr::to_str).flatten() {
            Some(filename) => {
                let filename: GoesEmwinFileName = match filename.parse() {
                    Ok(f) => f,
                    Err(e) => {
                        log::error!("Failed to parse newly created filename {}: {}", filename, e);
                        config.failure.do_for(&path).await;
                        return;
                    }
                };

                let month = filename
                    .creation_timestamp
                    .date()
                    .with_day0(0)
                    .expect("First day of month is invalid");

                let read = async {
                    match tokio::fs::read_to_string(&path).await {
                        Ok(src) => Some(src),
                        Err(e) => {
                            log::error!("Failed to read file {}: {}", path.display(), e);
                            config.failure.do_for(&path).await;
                            None
                        }
                    }
                };

                match filename.wmo_product_id {
                    DataTypeDesignator::Analysis(Analysis {
                        subtype: AnalysisSubType::Surface,
                        ..
                    }) => {
                        let Some(src) = read.await else { return };
                        let _ = match RegionalWeatherRoundup::parse(&src) {
                            Ok((_, rwr)) => rwr,
                            Err(e) => {
                                log::error!("Failed to parse regional weather roundup: {}", e);
                                config.failure.do_for(&path).await;
                                return;
                            }
                        };

                        config.done.do_for(path).await;
                    }
                    DataTypeDesignator::UpperAirData(UpperAirData {
                        subtype: UpperAirDataSubType::AircraftReport(CodeForm::AMDAR),
                        ..
                    }) => {
                        let Some(src) = read.await else { return };
                        let report = match AmdarReport::parse(&src) {
                            Ok((_, report)) => report,
                            Err(e) => {
                                log::error!("Failed to parse AMDAR upper air report: {}", e);
                                config.failure.do_for(&path).await;
                                return;
                            }
                        };

                        config.done.do_for(path).await;
                    }
                    DataTypeDesignator::Forecast(Forecast {
                        subtype: ForecastSubType::AerodomeVTLT12 | ForecastSubType::AerodomeVTGE12,
                        ..
                    }) => {
                        let Some(src) = read.await else { return };
                        let forecast = match TAFReport::parse(month)(&src) {
                            Ok((_, forecast)) => forecast,
                            Err(e) => {
                                log::error!("Failed to parse TAF report: {}", e);
                                config.failure.do_for(&path).await;
                                return;
                            }
                        };

                        for item in forecast.items {
                            if let Err(e) = ctx.insert_taf(forecast.month, &item).await {
                                log::error!("Failed to write TAF forecast to database: {}", e);
                            }
                        }

                        config.done.do_for(path).await;
                    }
                    _ => {
                        log::trace!("Unknown EMWIN product: {:?}", filename.wmo_product_id);
                        config.unrecognized.do_for(&path).await;
                    }
                }
            }
            None => {
                log::error!(
                    "Newly created file {} contains invalid unicode characters",
                    path.display(),
                );
                config.unrecognized.do_for(&path).await;
            }
        }
    }
}


pub async fn img_dispatch(event: Event, ctx: Arc<GoesSqlContext>, config: Arc<Config>) {
    for path in event.paths {
        match path.file_stem().map(std::ffi::OsStr::to_str).flatten() {
            Some(file_name) => {
                let file_name = match GoesFileName::parse(file_name) {
                    Ok((_, f)) => f,
                    Err(e) => {
                        log::error!("Failed to parse GOES-R image file name: {}", display_error(e));
                        return
                    }
                };

                if let Err(e) = ctx.insert_goes(file_name, &path).await {
                    log::error!("Failed to write GOES-R image file to database: {}", e);
                }

                config.done.do_for(path).await; 
            },
            None => {
                log::error!(
                    "Newly created image file {} contains invalid unicode characters",
                    path.display(),
                );
                config.unrecognized.do_for(&path).await;
            }
        }
    }
}
