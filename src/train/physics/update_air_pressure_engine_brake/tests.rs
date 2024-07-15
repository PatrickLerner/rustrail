use super::*;
use coverage_helper::test;

#[coverage(off)]
fn spawn_engine(app: &mut App, air_pressure: f32, engine_brake: f32) -> Entity {
    app.world_mut()
        .spawn((
            AirPressure(air_pressure),
            BrakeLever {
                engine_brake,
                ..default()
            },
        ))
        .id()
}

#[test]
fn no_brake_no_change() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let initial_pressure = 5.0;
    let engine_id = spawn_engine(&mut app, initial_pressure, 0.0);

    app.update();

    let air_pressure = app.world().get::<AirPressure>(engine_id).unwrap().0;
    assert_eq!(air_pressure, initial_pressure);
}

#[test]
fn full_brake_zero_pressure() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let initial_pressure = 5.0;
    let engine_id = spawn_engine(&mut app, initial_pressure, 1.0);

    app.update();

    let air_pressure = app.world().get::<AirPressure>(engine_id).unwrap().0;
    assert_eq!(air_pressure, 0.0);
}

#[test]
fn partial_brake_reduces_pressure() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let initial_pressure = 5.0;
    let engine_brake = 0.5;
    let engine_id = spawn_engine(&mut app, initial_pressure, engine_brake);

    app.update();

    let air_pressure = app.world().get::<AirPressure>(engine_id).unwrap().0;
    assert_eq!(air_pressure, initial_pressure * (1.0 - engine_brake));
}

#[test]
fn varying_brake_levels() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let initial_pressure = 5.0;

    let brake_levels = vec![0.0, 0.2, 0.5, 0.8, 1.0];
    for &engine_brake in &brake_levels {
        let engine_id = spawn_engine(&mut app, initial_pressure, engine_brake);

        app.update();

        let air_pressure = app.world().get::<AirPressure>(engine_id).unwrap().0;
        assert_eq!(air_pressure, initial_pressure * (1.0 - engine_brake));
    }
}
