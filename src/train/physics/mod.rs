#[cfg(test)]
mod tests;

mod update_acceleration;
mod update_air_resistance;
mod update_braking_force;
mod update_distance;
mod update_drive_force;
mod update_friction;
mod update_speed;

use bevy::prelude::*;

pub struct TrainPhysicsPlugin;

impl Plugin for TrainPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_drive_force::system,
                update_braking_force::system,
                update_friction::system,
                update_air_resistance::system,
                update_acceleration::system,
                update_speed::system,
                update_distance::system,
            ),
        );
    }
}
