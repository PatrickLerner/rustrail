#[cfg(test)]
mod tests;

use super::{TrainComposition, WrappedValue};
use bevy::prelude::*;

pub fn system<T: WrappedValue + Component>(
    trains: Query<(Entity, &TrainComposition)>,
    mut values: Query<&mut T>,
) {
    for (train_entity, composition) in trains.iter() {
        let train_value = {
            let train_value = values.get(train_entity).expect("train to experience force");
            train_value.get()
        };

        for component_entity in composition.entities() {
            let mut value = values
                .get_mut(component_entity)
                .expect("component of train to experience force");

            value.set(train_value);
        }
    }
}
