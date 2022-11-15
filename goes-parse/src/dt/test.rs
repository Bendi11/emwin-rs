use super::*;

#[test]
fn test_parse() {
    let weather_roundup: DataTypeDesignator = "ASMX00".parse().unwrap();
    assert!(matches!(
        weather_roundup,
        DataTypeDesignator::Analysis(Analysis {
            subtype: AnalysisSubType::Surface,
            area: _,
            enumerator: _,
        })
    ));
}
