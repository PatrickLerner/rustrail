#[cfg(test)]
mod tests;

use super::TrainRailLocation;
use crate::{
    landscape::OSMData,
    train::{Direction, Speed},
};
use bevy::prelude::*;

pub fn system(
    data: Res<OSMData>,
    mut trains: Query<(&Speed, &mut TrainRailLocation)>,
    time: Res<Time>,
) {
    for (speed, mut location) in trains.iter_mut() {
        if speed.0.abs() < f32::EPSILON {
            continue;
        }

        let rail = data
            .rails
            .get(&location.id)
            .expect("train location to be valid");

        let delta_distance = speed.0 as f64 * time.delta_seconds_f64();
        location.distance += delta_distance;

        let length = rail.length();

        let crossing_direction = if length < location.distance {
            Some(Direction::Forward)
        } else if location.distance < 0.0 {
            Some(Direction::Backward)
        } else {
            None
        };

        // need to move to next rail segment
        if let Some(crossing_direction) = crossing_direction {
            let old_travel_direction = location.travel_direction;
            let possible = rail.possible_connections_by_direction(match crossing_direction {
                Direction::Forward => location.travel_direction,
                Direction::Backward => location.travel_direction.opposite(),
            });
            let (next_id, next_direction) = possible
                .first()
                .expect("at least one possible followup location to exist");
            let next_rail = data.rails.get(next_id).expect("location to be valid");

            location.id = *next_id;
            location.travel_direction = match crossing_direction {
                Direction::Forward => *next_direction,
                Direction::Backward => next_direction.opposite(),
            };
            location.distance += match crossing_direction {
                Direction::Forward => -length,
                Direction::Backward => next_rail.length(),
            };

            log::debug!("=== new rail segment ===");
            log::debug!("path.distance: {:?}", location.distance);
            log::debug!("current id: {:?}", rail.id());
            log::debug!("next id: {:?}", next_id);
            log::debug!("current travel direction: {:?}", old_travel_direction);
            log::debug!("next travel direction: {:?}", location.travel_direction);
            log::debug!("=== new rail segment end ===");
        }
    }
}
