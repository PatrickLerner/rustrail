#[cfg(test)]
mod tests;

mod update_acceleration;
mod update_air_resistance;
mod update_braking_force;
mod update_distance;
mod update_drive_force;
mod update_friction;
mod update_speed;

use super::*;
use bevy::prelude::*;

fn sum_component_force_to_train(
    trains: Query<(Entity, &TrainComposition)>,
    mut forces: Query<(
        &mut ForceDriving,
        &mut ForceBraking,
        &mut ForceFriction,
        &mut ForceAirResistance,
    )>,
    mut speeds: Query<&mut Speed>,
    mut masses: Query<&mut Mass>,
    mut max_speeds: Query<&mut MaxSpeed>,
) {
    for (train_entity, composition) in trains.iter() {
        let speed = speeds.get(train_entity).expect("train to have speed").0;
        let mut max_speed = f32::MAX;

        let mut mass = 0.0;
        let mut f_dr = 0.0;
        let mut f_br = 0.0;
        let mut f_fr = 0.0;
        let mut f_ar = 0.0;

        for (index, component_entity) in composition.entities().into_iter().enumerate() {
            let (cf_dr, cf_br, cf_fr, cf_ar) = forces
                .get(component_entity)
                .expect("component of train to experience all forces");
            f_dr += cf_dr.0;
            f_br += cf_br.0;
            f_fr += cf_fr.0;
            // TODO: direction
            if index == 0 {
                f_ar = cf_ar.0;
            }

            mass += masses
                .get(component_entity)
                .expect("component to have mass")
                .0;

            speeds
                .get_mut(component_entity)
                .expect("component to have speed")
                .0 = speed;

            max_speed = f32::min(
                max_speed,
                max_speeds
                    .get_mut(component_entity)
                    .expect("component to have max speed")
                    .0,
            );
        }

        let (mut tf_dr, mut tf_br, mut tf_fr, mut tf_ar) = forces
            .get_mut(train_entity)
            .expect("train to experience all forces");

        tf_dr.0 = f_dr;
        tf_br.0 = f_br;
        tf_fr.0 = f_fr;
        tf_ar.0 = f_ar;

        masses.get_mut(train_entity).expect("train to have mass").0 = mass;
        max_speeds
            .get_mut(train_entity)
            .expect("train to have max speed")
            .0 = max_speed;
    }
}

pub struct TrainPhysicsPlugin;

impl Plugin for TrainPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                sum_component_force_to_train,
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
