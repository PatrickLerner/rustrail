use super::*;
use crate::camera::GameCameraBundle;
use coverage_helper::test;

#[coverage(off)]
fn mock_egui_is_unlocked() -> EguiUnlocked {
    EguiUnlocked(true)
}

#[coverage(off)]
fn mock_egui_is_locked() -> EguiUnlocked {
    EguiUnlocked(false)
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

#[test]
fn panning_with_egui_locked() {
    let mut app = App::new();

    let camera = app.world.spawn(GameCameraBundle::default()).id();

    app.add_event::<MouseMotion>();
    app.add_event::<MouseWheel>();

    let mut inputs: ButtonInput<MouseButton> = ButtonInput::default();
    inputs.press(MouseButton::Left);
    app.insert_resource(inputs);

    app.add_systems(Update, mock_egui_is_locked.pipe(system));

    app.world
        .resource_mut::<Events<MouseMotion>>()
        .send(MouseMotion {
            delta: Vec2::new(300.0, 300.0),
        });

    app.update();

    {
        let transform = app
            .world
            .query::<&Transform>()
            .get(&app.world, camera)
            .unwrap();

        assert_eq!(transform.translation.z, 1.0);
        assert_eq!(transform.translation.x, 0.0);
    }
}

#[test]
fn panning() {
    let mut app = App::new();

    let camera = app.world.spawn(GameCameraBundle::default()).id();

    app.add_event::<MouseMotion>();
    app.add_event::<MouseWheel>();

    let mut inputs: ButtonInput<MouseButton> = ButtonInput::default();
    inputs.press(MouseButton::Left);
    app.insert_resource(inputs);

    app.add_systems(Update, mock_egui_is_unlocked.pipe(system));

    app.world
        .resource_mut::<Events<MouseMotion>>()
        .send(MouseMotion {
            delta: Vec2::new(300.0, 300.0),
        });

    app.update();

    {
        let transform = app
            .world
            .query::<&Transform>()
            .get(&app.world, camera)
            .unwrap();

        // we moved forward and left
        // default was 1.0, we moved forward
        assert!(transform.translation.z < 1.0);
        // default was zero, we moved left
        assert!(transform.translation.x < 0.0);
    }
}

#[test]
fn orbit() {
    let mut app = App::new();

    let camera = app.world.spawn(GameCameraBundle::default()).id();

    app.add_event::<MouseMotion>();
    app.add_event::<MouseWheel>();

    let mut inputs: ButtonInput<MouseButton> = ButtonInput::default();
    inputs.press(MouseButton::Right);
    app.insert_resource(inputs);

    app.add_systems(Update, mock_egui_is_unlocked.pipe(system));

    app.world
        .resource_mut::<Events<MouseMotion>>()
        .send(MouseMotion {
            delta: Vec2::new(300.0, 300.0),
        });

    app.update();

    {
        let transform = app
            .world
            .query::<&Transform>()
            .get(&app.world, camera)
            .unwrap();

        // we turn up and left
        // just hard-coded results to fixate current behavior
        assert_eq!(transform.translation.z, 0.75);
        assert_eq!(transform.translation.x, -0.43301272);
    }

    // 360 no-scope
    app.world
        .resource_mut::<Events<MouseMotion>>()
        .send(MouseMotion {
            delta: Vec2::new(3000.0, 3000.0),
        });

    app.update();

    {
        let transform = app
            .world
            .query::<&Transform>()
            .get(&app.world, camera)
            .unwrap();

        // we turn up and left
        // just hard-coded results to fixate current behavior
        assert_eq!(transform.translation.z, 0.7499997);
        assert_eq!(transform.translation.x, 0.43301293);
    }

    app.world
        .resource_mut::<Events<MouseMotion>>()
        .send(MouseMotion {
            delta: Vec2::new(-3000.0, -3000.0),
        });

    app.update();

    {
        let transform = app
            .world
            .query::<&Transform>()
            .get(&app.world, camera)
            .unwrap();

        // we turn up and left
        // just hard-coded results to fixate current behavior
        assert_eq!(transform.translation.z, 0.75000006);
        assert_eq!(transform.translation.x, -0.43301263);
    }

    app.update();

    // test upside down on start

    app.world
        .resource_mut::<Events<MouseMotion>>()
        .send(MouseMotion {
            delta: Vec2::new(0.0, -1700.0),
        });

    app.update();

    app.world
        .resource_mut::<Events<MouseMotion>>()
        .send(MouseMotion {
            delta: Vec2::new(0.0, 100.0),
        });

    app.update();

    {
        let transform = app
            .world
            .query::<&Transform>()
            .get(&app.world, camera)
            .unwrap();

        // we turn up and left
        // just hard-coded results to fixate current behavior
        assert_eq!(transform.translation.z, -0.5566705);
        assert_eq!(transform.translation.x, 0.32139376);
    }
}
