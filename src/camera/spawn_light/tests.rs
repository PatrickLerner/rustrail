use super::*;
use coverage_helper::test;

#[test]
fn spawn_light() {
    let mut app = App::new();

    let light = app.world.get_resource::<AmbientLight>();
    assert!(light.is_none());

    app.add_systems(Update, system);
    app.update();

    let light = app.world.get_resource::<AmbientLight>();
    assert!(light.is_some());
    let light = light.unwrap();
    assert_eq!(light.brightness, 255.0);
}
