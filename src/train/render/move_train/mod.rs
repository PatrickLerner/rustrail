#[cfg(test)]
mod tests;

use std::process::exit;

use super::Train3DModel;
use crate::{
    landscape::{HeightMap, OSMData, OriginOffset},
    train::{render::Perspective, Direction, Speed},
    HEIGHT_OFFSET, TRAIN_HEIGHT_OFFSET,
};
use bevy::prelude::*;

const SPEED: f32 = 3.0; // m/s

pub fn system(
    data: Res<OSMData>,
    mut trains: Query<(&Speed, &mut Train3DModel, &mut Transform)>,
    time: Res<Time>,
    height_map: Res<HeightMap>,
    origin_offset: Res<OriginOffset>,
) {
    for (speed, mut model, mut transform) in trains.iter_mut() {
        if let Some(path) = &mut model.path {
            if speed.0.abs() < f32::EPSILON {
                continue;
            }

            // TODO: remove unwraps
            let rail = data.rails.get(&path.id).unwrap();

            let (s, e) = match path.travel_direction {
                Direction::Forward => (rail.start_coords, rail.end_coords),
                Direction::Backward => (rail.end_coords, rail.start_coords),
            };

            let diff = e - s;
            let length = rail.length();
            let diff = diff / length;

            let dest = s + diff * path.distance;
            let dest = dest - origin_offset.0;

            // calculate height

            let height = height_map
                .height_at_position(dest.0 + origin_offset.0 .0, dest.1 + origin_offset.0 .1);
            let height = height + HEIGHT_OFFSET + TRAIN_HEIGHT_OFFSET;

            let target = Vec3::new(dest.0 as f32, height, -dest.1 as f32);
            let diff = target - transform.translation;
            transform.translation += (diff * SPEED * time.delta_seconds()).clamp(-diff.abs(), diff);

            // TODO: two systems ^ v

            let delta_distance = speed.0 as f64 * time.delta_seconds_f64();
            path.distance += delta_distance;

            let crossing_direction = if length < path.distance {
                Some(Direction::Forward)
            } else if path.distance < 0.0 {
                Some(Direction::Backward)
            } else {
                None
            };

            // need to move to next rail segment
            if let Some(crossing_direction) = crossing_direction {
                let old_travel_direction = path.travel_direction;
                let possible = rail.possible_connections_by_direction(match crossing_direction {
                    Direction::Forward => path.travel_direction,
                    Direction::Backward => path.travel_direction.opposite(),
                });
                let (next_id, next_direction) = possible.first().unwrap();
                let next_rail = data.rails.get(next_id).unwrap();

                path.id = *next_id;
                // path.travel_direction = *next_direction;
                path.travel_direction = match crossing_direction {
                    Direction::Forward => *next_direction,
                    Direction::Backward => next_direction.opposite(),
                };
                // path.distance -= length;

                path.distance += match crossing_direction {
                    Direction::Forward => -length,
                    Direction::Backward => next_rail.length(),
                };

                log::debug!("=== new rail segment ===");
                log::debug!("path.distance: {:?}", path.distance);
                log::debug!("current id: {:?}", rail.id());
                log::debug!("next id: {:?}", next_id);
                log::debug!("current travel direction: {:?}", old_travel_direction);
                log::debug!("next travel direction: {:?}", path.travel_direction);
                log::debug!("=== new rail segment end ===");
            }
        }
    }
}
