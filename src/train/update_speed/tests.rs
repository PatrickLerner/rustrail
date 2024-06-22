use super::*;
use std::time::Duration;

#[test]
fn positive_acceleration() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let train_id = app
        .world
        .spawn((Speed(30.0), MaxSpeed(40.0), Acceleration(5.0)))
        .id();

    app.init_resource::<Time>();
    let mut time = app.world.resource_mut::<Time>();
    time.advance_by(Duration::from_millis(500));

    app.update();

    assert!(app.world.get::<Speed>(train_id).is_some());
    assert_eq!(app.world.get::<Speed>(train_id).unwrap().0, 32.5);
}

#[test]
fn negative_acceleration() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let train_id = app
        .world
        .spawn((Speed(30.0), MaxSpeed(40.0), Acceleration(-5.0)))
        .id();

    app.init_resource::<Time>();
    let mut time = app.world.resource_mut::<Time>();
    time.advance_by(Duration::from_millis(500));

    app.update();

    assert!(app.world.get::<Speed>(train_id).is_some());
    assert_eq!(app.world.get::<Speed>(train_id).unwrap().0, 27.5);
}

#[test]
fn clamp_max_speed() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let train_id = app
        .world
        .spawn((Speed(39.0), MaxSpeed(40.0), Acceleration(5.0)))
        .id();

    app.init_resource::<Time>();
    let mut time = app.world.resource_mut::<Time>();
    time.advance_by(Duration::from_millis(500));

    app.update();

    assert!(app.world.get::<Speed>(train_id).is_some());
    assert_eq!(app.world.get::<Speed>(train_id).unwrap().0, 40.0);
}

#[test]
fn clamp_negative_max_speed() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let train_id = app
        .world
        .spawn((Speed(-39.0), MaxSpeed(40.0), Acceleration(-5.0)))
        .id();

    app.init_resource::<Time>();
    let mut time = app.world.resource_mut::<Time>();
    time.advance_by(Duration::from_millis(500));

    app.update();

    assert!(app.world.get::<Speed>(train_id).is_some());
    assert_eq!(app.world.get::<Speed>(train_id).unwrap().0, -40.0);
}
