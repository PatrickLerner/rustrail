use super::*;
use coverage_helper::test;

#[coverage(off)]
fn spawn_engine(app: &mut App, speed: f32) -> Entity {
    app.world_mut()
        .spawn((Engine, ForceAirResistance::default(), Speed(speed)))
        .id()
}

#[test]
fn no_speed_no_resistance() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let engine_id = spawn_engine(&mut app, 0.0);

    app.update();

    assert!(app.world().get::<ForceAirResistance>(engine_id).is_some());
    assert_eq!(
        app.world().get::<ForceAirResistance>(engine_id).unwrap().0,
        0.0
    );
}

#[test]
fn higher_speed_higher_resistance() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let slow_train = spawn_engine(&mut app, 10.0);
    let fast_train = spawn_engine(&mut app, 30.0);

    app.update();

    assert!(app.world().get::<ForceAirResistance>(slow_train).is_some());
    assert!(app.world().get::<ForceAirResistance>(fast_train).is_some());
    assert!(
        app.world().get::<ForceAirResistance>(slow_train).unwrap().0
            < app.world().get::<ForceAirResistance>(fast_train).unwrap().0
    );
}

#[test]
fn negative_positive_speed_equal() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let backwards_train = spawn_engine(&mut app, -30.0);
    let forwards_train = spawn_engine(&mut app, 30.0);

    app.update();

    assert!(app
        .world()
        .get::<ForceAirResistance>(backwards_train)
        .is_some());
    assert!(app
        .world()
        .get::<ForceAirResistance>(forwards_train)
        .is_some());
    assert_eq!(
        app.world()
            .get::<ForceAirResistance>(backwards_train)
            .unwrap()
            .0,
        app.world()
            .get::<ForceAirResistance>(forwards_train)
            .unwrap()
            .0
    );
}
