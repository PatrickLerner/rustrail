use super::*;
use coverage_helper::test;

#[test]
fn loading_data() {
    let data = ScenarioData::load_from_file("assets/scenarios/rb35.toml");

    assert_eq!(data.info.name, "RB 35: Worms => Bingen");
    assert_eq!(data.map.osm_data, "assets/rheinland-pfalz-latest.osm.pbf");
    assert_eq!(data.stops.len(), 16);
}
