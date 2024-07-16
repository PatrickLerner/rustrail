use super::*;
use coverage_helper::test;

#[coverage(off)]
fn spawn_train(app: &mut App, air_pressure: f32, release_valve: f32) -> Entity {
    app.world_mut()
        .spawn((
            AirPressure(air_pressure),
            AirPressureDelta(0.0),
            BrakeLever {
                release_valve,
                engine_brake: 0.0,
            },
        ))
        .id()
}

#[test]
fn no_brake_pressure_increases() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let train_id = spawn_train(&mut app, MAX_AIR_PRESSURE - 1.0, 0.0);
    app.init_resource::<Time>();

    app.update();

    let air_pressure_delta = app.world().get::<AirPressureDelta>(train_id).unwrap().0;
    assert_eq!(
        air_pressure_delta,
        COMPRESSOR_SPEED * app.world().resource::<Time>().delta_seconds()
    );
}

#[test]
fn brake_applies_pressure_decrease() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let release_valve = 0.5;
    let train_id = spawn_train(&mut app, MAX_AIR_PRESSURE, release_valve);
    app.init_resource::<Time>();

    app.update();

    let air_pressure_delta = app.world().get::<AirPressureDelta>(train_id).unwrap().0;
    let target = MAX_AIR_PRESSURE * (1.0 - release_valve.powi(2));
    let expected_delta = (target - MAX_AIR_PRESSURE)
        .min(COMPRESSOR_SPEED * app.world().resource::<Time>().delta_seconds());
    assert_eq!(air_pressure_delta, expected_delta);
}

#[test]
fn full_brake_pressure_decreases() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let train_id = spawn_train(&mut app, MAX_AIR_PRESSURE, 1.0);
    app.init_resource::<Time>();

    app.update();

    let air_pressure_delta = app.world().get::<AirPressureDelta>(train_id).unwrap().0;
    let target = MAX_AIR_PRESSURE * (1.0 - 1.0);
    let expected_delta = (target - MAX_AIR_PRESSURE)
        .min(COMPRESSOR_SPEED * app.world().resource::<Time>().delta_seconds());
    assert_eq!(air_pressure_delta, expected_delta);
}

#[test]
fn partial_brake_pressure_decreases() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let release_valve = 0.25;
    let train_id = spawn_train(&mut app, MAX_AIR_PRESSURE, release_valve);
    app.init_resource::<Time>();

    app.update();

    let air_pressure_delta = app.world().get::<AirPressureDelta>(train_id).unwrap().0;
    let target = MAX_AIR_PRESSURE * (1.0 - release_valve.powi(2));
    let expected_delta = (target - MAX_AIR_PRESSURE)
        .min(COMPRESSOR_SPEED * app.world().resource::<Time>().delta_seconds());
    assert_eq!(air_pressure_delta, expected_delta);
}
