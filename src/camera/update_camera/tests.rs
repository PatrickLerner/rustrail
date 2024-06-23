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
        let transform = app
            .world
            .query::<&Transform>()
            .get(&app.world, camera)
            .unwrap();

        // we move back by radius and radius defaults to 1.0
        assert_eq!(transform.translation, Vec3::new(0.0, 0.0, 1.0));
    }

    // move mouse but no clicks changes nothing
    app.world
        .resource_mut::<Events<MouseMotion>>()
        .send(MouseMotion {
            delta: Vec2::new(10.0, 10.0),
        });

    app.world
        .resource_mut::<Events<MouseMotion>>()
        .send(MouseMotion {
            delta: Vec2::new(3.0, 12.0),
        });

    app.update();

    {
        let transform = app
            .world
            .query::<&Transform>()
            .get(&app.world, camera)
            .unwrap();

        // we move back by radius and radius defaults to 1.0
        assert_eq!(transform.translation, Vec3::new(0.0, 0.0, 1.0));
    }
}

#[test]
fn zoom_in_out() {
    let mut app = App::new();

    let camera = app.world.spawn(GameCameraBundle::default()).id();

    app.add_event::<MouseMotion>();
    app.add_event::<MouseWheel>();

    let inputs: ButtonInput<MouseButton> = ButtonInput::default();
    app.insert_resource(inputs);

    app.add_systems(Update, mock_egui_is_unlocked.pipe(system));

    // scrolling up
    app.world
        .resource_mut::<Events<MouseWheel>>()
        .send(MouseWheel {
            unit: MouseScrollUnit::Pixel,
            x: 0.0,
            y: 200.0,
            window: Entity::from_raw(0),
        });

    app.update();

    {
        let transform = app
            .world
            .query::<&Transform>()
            .get(&app.world, camera)
            .unwrap();

        // we move back by radius and radius defaults to 1.0, so it must be smaller
        assert!(transform.translation.z < 1.0);

        let state = app
            .world
            .query::<&GameCameraState>()
            .get(&app.world, camera)
            .unwrap();

        // it zoomed in
        assert!(state.radius < 1.0);
    }

    // scrolling down
    app.world
        .resource_mut::<Events<MouseWheel>>()
        .send(MouseWheel {
            unit: MouseScrollUnit::Pixel,
            x: 0.0,
            y: -200.0,
            window: Entity::from_raw(0),
        });

    app.update();

    {
        let transform = app
            .world
            .query::<&Transform>()
            .get(&app.world, camera)
            .unwrap();

        // we move back by radius and radius defaults to 1.0, so it must be now at one again
        assert_eq!(transform.translation.z, 1.0);

        let state = app
            .world
            .query::<&GameCameraState>()
            .get(&app.world, camera)
            .unwrap();

        // it zoomed out again
        assert_eq!(state.radius, 1.0);
    }

    // scrolling down (mouse wheel)
    app.world
        .resource_mut::<Events<MouseWheel>>()
        .send(MouseWheel {
            unit: MouseScrollUnit::Line,
            x: 0.0,
            y: -20.0,
            window: Entity::from_raw(0),
        });

    app.update();

    {
        let transform = app
            .world
            .query::<&Transform>()
            .get(&app.world, camera)
            .unwrap();

        // we move back by radius and radius defaults to 1.0, so it must be now be larger
        assert!(transform.translation.z > 1.0);

        let state = app
            .world
            .query::<&GameCameraState>()
            .get(&app.world, camera)
            .unwrap();

        // it zoomed out even more
        assert!(state.radius > 1.0);
    }
}
