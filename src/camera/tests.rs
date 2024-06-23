use super::*;

#[test]
fn plugin() {
    let mut app = App::default();
    app.add_plugins(CameraPlugin);
    assert!(app.is_plugin_added::<CameraPlugin>());
}
