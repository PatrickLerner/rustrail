use super::*;
use crate::train::{ForceBraking, TrainComponent};
use coverage_helper::test;

#[test]
fn applies_value() {
    let mut app = App::new();

    let engine_1 = app.world.spawn(ForceBraking(10.0)).id();
    let engine_2 = app.world.spawn(ForceBraking(2.0)).id();
    let engine_3 = app.world.spawn(ForceBraking(4.0)).id();

    let train = app
        .world
        .spawn((
            TrainComposition {
                components: vec![
                    TrainComponent::Engine(engine_1),
                    TrainComponent::Engine(engine_2),
                    TrainComponent::Engine(engine_3),
                ],
            },
            ForceBraking(0.0),
        ))
        .id();

    app.add_systems(Update, system::<ForceBraking>);
    app.update();

    let mut values = app.world.query::<&ForceBraking>();
    let value = values.get(&app.world, train).unwrap().get();

    assert_eq!(value, 10.0);
}
