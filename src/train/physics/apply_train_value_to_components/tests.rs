use super::*;
use crate::train::{ForceBraking, TrainComponent};
use coverage_helper::test;

#[test]
fn applies_value() {
    let mut app = App::new();

    let engine_1 = app.world.spawn(ForceBraking(1.0)).id();
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
            ForceBraking(9.0),
        ))
        .id();

    app.add_systems(Update, system::<ForceBraking>);
    app.update();

    let mut values = app.world.query::<&ForceBraking>();
    for entity in [train, engine_1, engine_2, engine_3] {
        let value = values.get(&app.world, entity).unwrap().get();

        assert_eq!(value, 9.0);
    }
}