use super::*;
use coverage_helper::test;

#[test]
fn plugin() {
    let mut app = App::default();
    app.add_plugins(TrainPhysicsPlugin);
    assert!(app.is_plugin_added::<TrainPhysicsPlugin>());
}
