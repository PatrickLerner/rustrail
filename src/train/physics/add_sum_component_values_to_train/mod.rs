#[cfg(test)]
mod tests;

use crate::train::{TrainComposition, WrappedValue};
use bevy::prelude::*;

pub fn system<T: WrappedValue + Component, S: WrappedValue + Component>(
    trains: Query<(Entity, &TrainComposition)>,
    values: Query<&T>,
    mut train_values: Query<&mut S>,
) {
    for (train_entity, composition) in trains.iter() {
        let mut sum = 0.0;

        for component_entity in composition.entities() {
            if let Ok(value) = values.get(component_entity) {
                sum += value.get();
            }
        }

        let mut train_value = train_values
            .get_mut(train_entity)
            .expect("train to experience force");

        let new_value = train_value.get() + sum;
        train_value.set(new_value);
    }
}
