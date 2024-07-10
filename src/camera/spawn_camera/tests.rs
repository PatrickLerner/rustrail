use super::*;
use coverage_helper::test;

#[test]
fn spawn_light() {
    let mut app = App::new();

    let mut camera = app.world_mut().query::<&Camera>();
    assert_eq!(camera.iter(&app.world()).len(), 0);

    app.add_systems(Update, system);
    app.update();

    assert_eq!(camera.iter(&app.world()).len(), 1);
}
