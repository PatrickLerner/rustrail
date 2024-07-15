use super::*;
use crate::train::Engine;
use coverage_helper::test;

#[coverage(off)]
fn spawn_engine(app: &mut App, air_pressure: AirPressure, mass: f32) -> Entity {
    app.world_mut()
        .spawn((Engine, ForceBraking::default(), Mass(mass), air_pressure))
        .id()
}

#[test]
fn no_brake_no_force() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let engine_id = spawn_engine(&mut app, AirPressure(MAX_AIR_PRESSURE), 7000.0);

    app.update();

    assert!(app.world().get::<ForceBraking>(engine_id).is_some());
    assert_eq!(app.world().get::<ForceBraking>(engine_id).unwrap().0, 0.0);
}

#[test]
fn brake_applies_force() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let engine_id = spawn_engine(&mut app, AirPressure(4.8), 7000.0);

    app.update();

    assert!(app.world().get::<ForceBraking>(engine_id).is_some());
    assert!(app.world().get::<ForceBraking>(engine_id).unwrap().0 > 0.0);
}

#[test]
fn more_brake_more_force() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let low_brake = spawn_engine(&mut app, AirPressure(4.8), 7000.);

    let high_brake = spawn_engine(&mut app, AirPressure(0.0), 7000.);

    app.update();

    assert!(app.world().get::<ForceBraking>(low_brake).is_some());
    assert!(app.world().get::<ForceBraking>(high_brake).is_some());
    assert!(
        app.world().get::<ForceBraking>(low_brake).unwrap().0
            < app.world().get::<ForceBraking>(high_brake).unwrap().0
    );
}

#[test]
fn more_weight_more_brake() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let low_weight = spawn_engine(&mut app, AirPressure(4.8), 7000.);

    let high_weight = spawn_engine(&mut app, AirPressure(4.8), 70000.);

    app.update();

    assert!(app.world().get::<ForceBraking>(low_weight).is_some());
    assert!(app.world().get::<ForceBraking>(high_weight).is_some());
    assert!(
        app.world().get::<ForceBraking>(low_weight).unwrap().0
            < app.world().get::<ForceBraking>(high_weight).unwrap().0
    );
}
