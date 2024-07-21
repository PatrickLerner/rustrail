use super::*;
use crate::scenario::ScenarioMap;
use coverage_helper::test;

#[test]
fn initiate_resources() {
    let mut app = App::new();

    let scenario = ScenarioData {
        map: ScenarioMap {
            osm_data: "assets/liechtenstein-latest.osm.pbf".to_owned(),
            height_map: "assets/dgm200_utm32s.tif".to_owned(),
            ..default()
        },
        ..default()
    };

    app.insert_resource(scenario);
    app.add_systems(Update, system);

    app.update();

    assert!(app.world().contains_resource::<HeightMap>());
    assert!(app.world().contains_resource::<OriginOffset>());
}
