use coverage_helper::test;
use super::*;

#[test]
fn initiate_resources() {
    let mut app = App::new();
    app.add_systems(Update, system);

    app.update();

    assert!(app.world.contains_resource::<HeightMap>());
    assert!(app.world.contains_resource::<OriginOffset>());
}
