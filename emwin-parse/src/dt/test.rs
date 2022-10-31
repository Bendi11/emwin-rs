use super::*;

#[test]
fn test_parse() {
    let weather_roundup: DataTypeDesignator = "UDCA00".parse().unwrap();
    eprintln!("{:#?}", weather_roundup);
    assert!(matches!(
        weather_roundup,
        DataTypeDesignator::Analysis(Analysis {
            subtype: AnalysisSubType::WeatherSummary,
            area: _,
            enumerator: _,
        })
    ));
}
