use super::*;
use crate::camera::GameCameraBundle;
use coverage_helper::test;

#[coverage(off)]
fn mock_egui_is_unlocked() -> EguiUnlocked {
    EguiUnlocked(true)
}

#[test]
fn no_movement() {
    let mut app = App::new();

    let camera = app.world.spawn(GameCameraBundle::default()).id();

    app.add_event::<MouseMotion>();
    app.add_event::<MouseWheel>();

    let inputs: ButtonInput<MouseButton> = ButtonInput::default();
    app.insert_resource(inputs);

    app.add_systems(Update, mock_egui_is_unlocked.pipe(system));
    app.update();

    {
        let mut transform = app.world.query::<&Transform>();
        let transform = transform.get(&app.world, camera);
        assert!(transform.is_ok());
        let transform = transform.unwrap();
        // we move back by radius and radius defaults to 1.0
        assert_eq!(transform.translation, Vec3::new(0.0, 0.0, 1.0));
    }
}
