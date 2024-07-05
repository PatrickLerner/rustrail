use super::*;
use coverage_helper::test;

#[derive(PartialEq)]
enum GenTrainMode {
    Driving,
    Breaking,
}

#[coverage(off)]
fn gen_train(app: &mut App, weight: f32, mode: GenTrainMode) -> Entity {
    app.world
        .spawn((
            Acceleration(0.0),
            Speed(30.0),
            ForceDriving(if mode == GenTrainMode::Driving {
                100.0
            } else {
                0.0
            }),
            ForceFriction(10.0),
            ForceAirResistance(10.0),
            ForceBraking(if mode == GenTrainMode::Breaking {
                100.0
            } else {
                0.0
            }),
            Mass(weight),
        ))
        .id()
}

#[test]
fn positive_force_balance() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let train_id = gen_train(&mut app, 7000.0, GenTrainMode::Driving);

    app.update();

    assert!(app.world.get::<Acceleration>(train_id).is_some());
    assert!(app.world.get::<Acceleration>(train_id).unwrap().0 > 0.0);

    // inverse driving force

    let train_id = app
        .world
        .spawn((
            Acceleration(0.0),
            Speed(0.0),
            ForceDriving(-100.0),
            ForceFriction(10.0),
            ForceAirResistance(10.0),
            ForceBraking(0.0),
            Mass(7000.0),
        ))
        .id();

    app.update();

    assert!(app.world.get::<Acceleration>(train_id).is_some());
    assert!(app.world.get::<Acceleration>(train_id).unwrap().0 < 0.0);
}

#[test]
fn negative_force_balance() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let train_id = gen_train(&mut app, 7000.0, GenTrainMode::Breaking);

    app.update();

    assert!(app.world.get::<Acceleration>(train_id).is_some());
    assert!(app.world.get::<Acceleration>(train_id).unwrap().0 < 0.0);

    // inverse speed

    let train_id = app
        .world
        .spawn((
            Acceleration(0.0),
            Speed(-100.0),
            ForceDriving(0.0),
            ForceFriction(10.0),
            ForceAirResistance(10.0),
            ForceBraking(0.0),
            Mass(7000.0),
        ))
        .id();

    app.update();

    assert!(app.world.get::<Acceleration>(train_id).is_some());
    assert!(app.world.get::<Acceleration>(train_id).unwrap().0 > 0.0);
}

#[test]
fn clamp_to_speed_if_deceleration_is_more() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let train_id = app
        .world
        .spawn((
            Acceleration(0.0),
            Speed(5.0),
            ForceDriving(0.0),
            ForceFriction(10000.0),
            ForceAirResistance(10000.0),
            ForceBraking(20000.0),
            Mass(7000.0),
        ))
        .id();

    app.update();

    assert!(app.world.get::<Acceleration>(train_id).is_some());
    assert_eq!(app.world.get::<Acceleration>(train_id).unwrap().0, -5.0);

    // inverse speed

    let train_id = app
        .world
        .spawn((
            Acceleration(0.0),
            Speed(-5.0),
            ForceDriving(0.0),
            ForceFriction(10000.0),
            ForceAirResistance(10000.0),
            ForceBraking(20000.0),
            Mass(7000.0),
        ))
        .id();

    app.update();

    assert!(app.world.get::<Acceleration>(train_id).is_some());
    assert_eq!(app.world.get::<Acceleration>(train_id).unwrap().0, 5.0);
}

#[test]
fn heavier_things_accelerate_less() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let low_weight = gen_train(&mut app, 7000.0, GenTrainMode::Driving);
    let high_weight = gen_train(&mut app, 70000.0, GenTrainMode::Driving);

    app.update();

    assert!(app.world.get::<Acceleration>(low_weight).is_some());
    assert!(app.world.get::<Acceleration>(high_weight).is_some());
    assert!(
        app.world.get::<Acceleration>(high_weight).unwrap().0
            < app.world.get::<Acceleration>(low_weight).unwrap().0
    );
}

#[test]
fn heavier_things_decelerate_less() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let low_weight = gen_train(&mut app, 7000.0, GenTrainMode::Breaking);
    let high_weight = gen_train(&mut app, 70000.0, GenTrainMode::Breaking);

    app.update();

    assert!(app.world.get::<Acceleration>(low_weight).is_some());
    assert!(app.world.get::<Acceleration>(high_weight).is_some());
    assert!(
        app.world.get::<Acceleration>(high_weight).unwrap().0
            > app.world.get::<Acceleration>(low_weight).unwrap().0
    );
}

#[test]
fn no_mass() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let train_id = gen_train(&mut app, 0.0, GenTrainMode::Driving);

    app.update();

    assert!(app.world.get::<Acceleration>(train_id).is_some());
    assert_eq!(app.world.get::<Acceleration>(train_id).unwrap().0, 0.0);
}
