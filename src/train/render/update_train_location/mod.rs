#[cfg(test)]
mod tests;

use super::TrainRailLocation;
use crate::{
    landscape::OSMData,
    train::{Speed, TrainComposition},
};
use bevy::prelude::*;

pub fn system(
    data: Res<OSMData>,
    trains: Query<(Entity, &Speed, &TrainComposition)>,
    mut locations: Query<&mut TrainRailLocation>,
    time: Res<Time>,
) {
    for (entity, speed, composition) in trains.iter() {
        if speed.0.abs() < f32::EPSILON {
            continue;
        }
        let delta_distance = speed.0 as f64 * time.delta_seconds_f64();

        {
            let mut location = locations.get_mut(entity).expect("train to have a location");
            location.add_distance(&data, delta_distance);
        }

        for component_entity in composition.entities() {
            let mut component_location = locations
                .get_mut(component_entity)
                .expect("component to have a location");

            component_location.add_distance(&data, delta_distance);
        }
    }
}
