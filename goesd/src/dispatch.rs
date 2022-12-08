use std::sync::Arc;

use chrono::Datelike;
use goes_parse::{
    display_error,
    dt::{
        product::{Analysis, Forecast},
        surface::SurfaceData,
        upperair::UpperAirData,
        AircraftReportCodeForm, AnalysisSubType, DataTypeDesignator, ForecastSubType,
        SurfaceSubType, UpperAirDataSubType,
    },
    formats::{metar::EmwinMetarReport, rwr::RegionalWeatherRoundup, taf::TAFReport},
    goes::GoesFileName,
    header::GoesEmwinFileName,
};
use goes_sql::GoesSqlContext;
use notify::Event;

use goes_cfg::Config;

pub const fn supported(name: &GoesEmwinFileName) -> bool {
    match name.wmo_product_id {
        DataTypeDesignator::Analysis(Analysis {
            subtype: AnalysisSubType::Surface,
            ..
        })
        | DataTypeDesignator::UpperAirData(UpperAirData {
            subtype: UpperAirDataSubType::AircraftReport(AircraftReportCodeForm::AMDAR),
            ..
        })
        | DataTypeDesignator::Forecast(Forecast {
            subtype: ForecastSubType::AerodomeVTLT12 | ForecastSubType::AerodomeVTGE12,
            ..
        })
        | DataTypeDesignator::SurfaceData(SurfaceData {
            subtype: SurfaceSubType::AviationRoutineReport,
            ..
        }) => true,
        _ => false,
    }
}

pub async fn emwin_dispatch(filename: GoesEmwinFileName, src: &str, ctx: Arc<GoesSqlContext>) {
    let month = filename
        .creation_timestamp
        .date()
        .with_day0(0)
        .expect("First day of month is invalid");

    match filename.wmo_product_id {
        DataTypeDesignator::Analysis(Analysis {
            subtype: AnalysisSubType::Surface,
            ..
        }) => {
            let _ = match RegionalWeatherRoundup::parse(&src) {
                Ok((_, rwr)) => rwr,
                Err(e) => {
                    log::error!("Failed to parse regional weather roundup: {}", e);
                    return;
                }
            };
        }
        DataTypeDesignator::UpperAirData(UpperAirData {
            subtype: UpperAirDataSubType::AircraftReport(AircraftReportCodeForm::AMDAR),
            ..
        }) => {
            /*let report = match AmdarReport::parse(&src) {
                Ok((_, report)) => report,
                Err(e) => {
                    log::error!("Failed to parse AMDAR upper air report: {}", e);
                    config.failure.do_for(&path).await;
                    return;
                }
            };

            config.done.do_for(path).await;*/
        }
        DataTypeDesignator::SurfaceData(SurfaceData {
            subtype: SurfaceSubType::AviationRoutineReport,
            ..
        }) => {
            let reports = match EmwinMetarReport::parse(month)(&src) {
                Ok((_, metar)) => metar,
                Err(e) => {
                    log::error!(
                        "Failed to parse METAR report:\n{}\n{}",
                        src,
                        goes_parse::display_error(e)
                    );
                    return;
                }
            };

            for report in reports.metars {
                if let Err(e) = ctx.insert_metar(reports.month, &report).await {
                    log::error!("Failed to write METAR report to SQL: {}", e);
                }
            }
        }
        DataTypeDesignator::Forecast(Forecast {
            subtype: ForecastSubType::AerodomeVTLT12 | ForecastSubType::AerodomeVTGE12,
            ..
        }) => {
            let forecast = match TAFReport::parse(month)(&src) {
                Ok((_, forecast)) => forecast,
                Err(e) => {
                    log::error!(
                        "Failed to parse TAF report: {}",
                        goes_parse::display_error(e)
                    );
                    return;
                }
            };

            for item in forecast.items {
                if let Err(e) = ctx.insert_taf(forecast.month, &item).await {
                    log::error!("Failed to write TAF forecast to database: {}", e);
                }
            }
        }
        _ => {
            log::trace!("Unknown EMWIN product: {:?}", filename.wmo_product_id);
        }
    }
}

pub async fn img_dispatch(event: Event, ctx: Arc<GoesSqlContext>, config: Arc<Config>) {
    for path in event.paths {
        let file_name = match GoesFileName::parse(&path) {
            Ok((_, f)) => f,
            Err(e) => {
                log::error!(
                    "Failed to parse GOES-R image file name: {}",
                    display_error(e)
                );
                return;
            }
        };

        if let Err(e) = ctx.insert_goes(file_name, &path).await {
            log::error!("Failed to write GOES-R image file to database: {}", e);
            config.failure.do_for(&path).await;
        }
    }
}
