use super::*;
use crate::train::Direction;
use coverage_helper::test;

#[coverage(off)]
fn spawn_engine(
    app: &mut App,
    throttle: ThrottleLever,
    brake: BrakeLever,
    speed: f32,
    force_driving: f32,
) -> Entity {
    app.world_mut()
        .spawn((
            Engine,
            MaxPower(1000.0),
            Speed(speed),
            throttle,
            brake,
            ForceDriving(force_driving),
        ))
        .id()
}

#[test]
fn no_throttle_no_force() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let engine_id = spawn_engine(
        &mut app,
        ThrottleLever::default(),
        BrakeLever::default(),
        0.0,
        10.0,
    );

    app.update();

    assert!(app.world().get::<ForceDriving>(engine_id).is_some());
    assert_eq!(app.world().get::<ForceDriving>(engine_id).unwrap().0, 0.0);
}

#[test]
fn forward_throttle_forward_force() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let engine_id = spawn_engine(
        &mut app,
        ThrottleLever {
            percentage: 0.2,
            direction: Direction::Forward,
        },
        BrakeLever::default(),
        0.0,
        0.0,
    );

    app.update();

    assert!(app.world().get::<ForceDriving>(engine_id).is_some());
    assert!(app.world().get::<ForceDriving>(engine_id).unwrap().0 > 0.0);
}

#[test]
fn backward_throttle_backward_force() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let engine_id = spawn_engine(
        &mut app,
        ThrottleLever {
            percentage: 0.2,
            direction: Direction::Backward,
        },
        BrakeLever::default(),
        0.0,
        0.0,
    );

    app.update();

    assert!(app.world().get::<ForceDriving>(engine_id).is_some());
    assert!(app.world().get::<ForceDriving>(engine_id).unwrap().0 < 0.0);
}

#[test]
fn more_throttle_more_force() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let low_throttle = spawn_engine(
        &mut app,
        ThrottleLever {
            percentage: 0.2,
            direction: Direction::Forward,
        },
        BrakeLever::default(),
        0.0,
        0.0,
    );

    let high_throttle = spawn_engine(
        &mut app,
        ThrottleLever {
            percentage: 1.0,
            direction: Direction::Forward,
        },
        BrakeLever::default(),
        0.0,
        0.0,
    );

    app.update();

    assert!(app.world().get::<ForceDriving>(low_throttle).is_some());
    assert!(app.world().get::<ForceDriving>(high_throttle).is_some());
    assert!(
        app.world().get::<ForceDriving>(low_throttle).unwrap().0
            < app.world().get::<ForceDriving>(high_throttle).unwrap().0
    );
}

#[test]
fn more_speed_less_force() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let low_speed = spawn_engine(
        &mut app,
        ThrottleLever {
            percentage: 0.2,
            direction: Direction::Forward,
        },
        BrakeLever::default(),
        0.0,
        0.0,
    );

    let high_speed = spawn_engine(
        &mut app,
        ThrottleLever {
            percentage: 0.2,
            direction: Direction::Forward,
        },
        BrakeLever::default(),
        30.0,
        0.0,
    );

    app.update();

    assert!(app.world().get::<ForceDriving>(low_speed).is_some());
    assert!(app.world().get::<ForceDriving>(high_speed).is_some());
    assert!(
        app.world().get::<ForceDriving>(low_speed).unwrap().0
            > app.world().get::<ForceDriving>(high_speed).unwrap().0
    );
}

#[test]
fn brake_applied_no_force() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let engine_id = spawn_engine(
        &mut app,
        ThrottleLever {
            percentage: 0.5,
            direction: Direction::Forward,
        },
        BrakeLever {
            release_valve: 1.0,
            engine_brake: 0.0,
        },
        10.0,
        123.0,
    );

    app.update();

    assert!(app.world().get::<ForceDriving>(engine_id).is_some());
    assert_eq!(app.world().get::<ForceDriving>(engine_id).unwrap().0, 0.0);
}

#[test]
fn engine_brake_applied_no_force() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let engine_id = spawn_engine(
        &mut app,
        ThrottleLever {
            percentage: 0.5,
            direction: Direction::Forward,
        },
        BrakeLever {
            release_valve: 0.0,
            engine_brake: 1.0,
        },
        10.0,
        123.0,
    );

    app.update();

    assert!(app.world().get::<ForceDriving>(engine_id).is_some());
    assert_eq!(app.world().get::<ForceDriving>(engine_id).unwrap().0, 0.0);
}
