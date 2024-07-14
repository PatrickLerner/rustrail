use super::*;
use crate::train::{AirPressure, AirPressureDelta, TrainComponent};
use coverage_helper::test;

#[test]
fn applies_value() {
    let mut app = App::new();

    let engine_1 = app.world_mut().spawn(AirPressureDelta(1.0)).id();
    let engine_2 = app.world_mut().spawn(AirPressureDelta(2.0)).id();
    let engine_3 = app.world_mut().spawn(AirPressureDelta(-4.0)).id();

    let train = app
        .world_mut()
        .spawn((
            TrainComposition {
                components: vec![
                    TrainComponent::Engine(engine_1),
                    TrainComponent::Engine(engine_2),
                    TrainComponent::Engine(engine_3),
                ],
            },
            AirPressure(5.0),
        ))
        .id();

    app.add_systems(Update, system::<AirPressureDelta, AirPressure>);
    app.update();

    let mut values = app.world_mut().query::<&AirPressure>();
    let value = values.get(&app.world_mut(), train).unwrap().get();

    assert_eq!(value, 4.0);
}
