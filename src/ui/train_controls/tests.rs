use super::*;
use coverage_helper::test;

#[test]
fn plugin() {
    let mut app = App::default();
    app.add_plugins(TrainControlsPlugin);
    assert!(app.is_plugin_added::<TrainControlsPlugin>());
}
