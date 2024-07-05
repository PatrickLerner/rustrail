#[cfg(test)]
mod tests;

mod update_acceleration;
mod update_air_resistance;
mod update_braking_force;
mod update_distance;
mod update_drive_force;
mod update_friction;
mod update_speed;
mod update_train_location;

use super::*;
use bevy::prelude::*;

fn apply_min_component_value_to_train<T: WrappedValue + Component>(
    trains: Query<(Entity, &TrainComposition)>,
    mut values: Query<&mut T>,
) {
    for (train_entity, composition) in trains.iter() {
        let mut min_value = f32::MAX;

        for component_entity in composition.entities() {
            let value = values
                .get(component_entity)
                .expect("component of train to experience force");
            min_value = f32::min(min_value, value.get());
        }

        let mut train_value = values
            .get_mut(train_entity)
            .expect("train to experience all forces");

        train_value.set(min_value);
    }
}

fn apply_first_component_value_to_train<T: WrappedValue + Component>(
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
            .expect("train to experience all forces");

        train_value.set(value);
    }
}

fn apply_sum_component_values_to_train<T: WrappedValue + Component>(
    trains: Query<(Entity, &TrainComposition)>,
    mut values: Query<&mut T>,
) {
    for (train_entity, composition) in trains.iter() {
        let mut sum = 0.0;

        for component_entity in composition.entities() {
            let value = values
                .get(component_entity)
                .expect("component of train to experience force");
            sum += value.get();
        }

        let mut train_value = values
            .get_mut(train_entity)
            .expect("train to experience all forces");

        train_value.set(sum);
    }
}

fn apply_train_value_to_components<T: WrappedValue + Component>(
    trains: Query<(Entity, &TrainComposition)>,
    mut values: Query<&mut T>,
) {
    for (train_entity, composition) in trains.iter() {
        let train_value = {
            let train_value = values
                .get(train_entity)
                .expect("train to experience all forces");
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

fn train_component_sync(trains: Query<(Entity, &TrainComposition)>, mut speeds: Query<&mut Speed>) {
    for (train_entity, composition) in trains.iter() {
        let speed = speeds.get(train_entity).expect("train to have speed").0;

        for component_entity in composition.entities() {
            speeds
                .get_mut(component_entity)
                .expect("component to have speed")
                .0 = speed;
        }
    }
}

pub struct TrainPhysicsPlugin;

impl Plugin for TrainPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                apply_train_value_to_components::<Speed>,
                apply_min_component_value_to_train::<MaxSpeed>,
                apply_sum_component_values_to_train::<Mass>,
                apply_sum_component_values_to_train::<ForceDriving>,
                apply_sum_component_values_to_train::<ForceBraking>,
                apply_sum_component_values_to_train::<ForceFriction>,
                // TODO: should not be first in list if driving backwards should be last
                apply_first_component_value_to_train::<ForceAirResistance>,
                train_component_sync,
                update_drive_force::system,
                update_braking_force::system,
                update_friction::system,
                update_air_resistance::system,
                update_acceleration::system,
                update_speed::system,
                update_distance::system,
                update_train_location::system,
            ),
        );
    }
}
