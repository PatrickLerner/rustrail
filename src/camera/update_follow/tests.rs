use std::time::Duration;

use super::*;
use crate::camera::GameCameraBundle;
use coverage_helper::test;

#[test]
fn update_follow_position() {
    let mut app = App::default();

    let camera_id = app.world_mut().spawn(GameCameraBundle::default()).id();

    let entity = app
        .world_mut()
        .spawn(Transform::from_xyz(10.0, 10.0, 10.0))
        .id();

    app.init_resource::<Time>();
    let mut time = app.world_mut().resource_mut::<Time>();
    time.advance_by(Duration::from_millis(500));

    let mut state = app.world_mut().query::<&mut GameCameraState>();
    let world = app.world_mut();
    let mut state = state.get_mut(world, camera_id).unwrap();
    state.follow = Some(entity);

    {
        let mut state = app.world_mut().query::<&GameCameraState>();
        let state = state.get(&app.world(), camera_id).unwrap();
        assert_eq!(state.center, Vec3::ZERO);
    }

    app.add_systems(Update, system);
    app.update();

    {
        let mut state = app.world_mut().query::<&GameCameraState>();
        let state = state.get(&app.world(), camera_id).unwrap();
        assert_eq!(state.center, Vec3::new(10.0, 10.0, 10.0));
    }
}
