use super::*;

#[test]
fn test_parse() {
    let weather_roundup: DataTypeDesignator = "ASCA".parse().unwrap();
    let area = AreaCode::try_from(('C', 'A')).unwrap();
   // assert_eq!(weather_roundup, DataTypeDesignator::Analysis(AnalysisT2::Surface, area));
}
