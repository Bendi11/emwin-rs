use chrono::Duration;
use uom::si::f32::Length;

use crate::header::CCCC;

use super::codes::wind::WindSummary;


/// A single METAR weather report parsed from a FM 15/16 report
#[derive(Clone, Debug, )]
pub struct MetarReport {
    pub country: CCCC,
    pub origin: Duration,
    pub wind: WindSummary,
    pub kind: MetarReportKind,
    pub visibility: Option<Length>
}

/// The kind of report a METAR/SPECI is
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MetarReportKind {
    Auto,
    Cor,
}
