use super::*;
use coverage_helper::test;
use std::time::Duration;

#[test]
fn positive_speed() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let train_id = app.world_mut().spawn((Speed(30.0), Distance(10.0))).id();

    app.init_resource::<Time>();
    let mut time = app.world_mut().resource_mut::<Time>();
    time.advance_by(Duration::from_millis(500));

    app.update();

    assert!(app.world_mut().get::<Distance>(train_id).is_some());
    assert_eq!(app.world_mut().get::<Distance>(train_id).unwrap().0, 25.0);
}

#[test]
fn negative_speed() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let train_id = app.world_mut().spawn((Speed(-30.0), Distance(10.0))).id();

    app.init_resource::<Time>();
    let mut time = app.world_mut().resource_mut::<Time>();
    time.advance_by(Duration::from_millis(500));

    app.update();

    assert!(app.world_mut().get::<Distance>(train_id).is_some());
    assert_eq!(app.world_mut().get::<Distance>(train_id).unwrap().0, -5.0);
}
