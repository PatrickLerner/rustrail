#[cfg(test)]
mod tests;

use crate::train::{TrainComposition, WrappedValue};
use bevy::prelude::*;

pub fn system<T: WrappedValue + Component>(
    trains: Query<(Entity, &TrainComposition)>,
    mut values: Query<&mut T>,
) {
    for (train_entity, composition) in trains.iter() {
        let mut sum = 0.0;

        for component_entity in composition.entities() {
            if let Ok(value) = values.get(component_entity) {
                sum += value.get();
            }
        }

        let mut train_value = values
            .get_mut(train_entity)
            .expect("train to experience force");

        train_value.set(sum);
    }
}
