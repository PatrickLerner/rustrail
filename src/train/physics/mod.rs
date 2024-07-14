#[cfg(test)]
mod tests;

mod apply_first_component_value_to_train;
mod apply_min_component_value_to_train;
mod apply_sum_component_values_to_train;
mod apply_train_value_to_components;
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

pub struct TrainPhysicsPlugin;

impl Plugin for TrainPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                apply_train_value_to_components::system::<AirPressure>,
                apply_train_value_to_components::system::<Speed>,
                apply_min_component_value_to_train::system::<MaxSpeed>,
                apply_sum_component_values_to_train::system::<Mass>,
                apply_sum_component_values_to_train::system::<ForceDriving>,
                apply_sum_component_values_to_train::system::<ForceBraking>,
                apply_sum_component_values_to_train::system::<ForceFriction>,
                // TODO: should not be first in list if driving backwards should be last
                apply_first_component_value_to_train::system::<ForceAirResistance>,
            ),
        )
        .add_systems(
            Update,
            (
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
