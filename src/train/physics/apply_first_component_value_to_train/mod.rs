#[cfg(test)]
mod tests;

use super::{TrainComposition, WrappedValue};
use bevy::prelude::*;

pub fn system<T: WrappedValue + Component>(
    trains: Query<(Entity, &TrainComposition)>,
    mut values: Query<&mut T>,
) {
    for (train_entity, composition) in trains.iter() {
        let value = {
            let components = composition.entities();

            let component_entity = components.first().expect("train to have a component");

            let value = values
                .get(*component_entity)
                .expect("component of train to experience force");

            value.get()
        };

        let mut train_value = values
            .get_mut(train_entity)
            .expect("train to experience force");

        train_value.set(value);
    }
}