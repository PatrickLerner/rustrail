#[cfg(test)]
mod tests;

use std::process::exit;

use super::{Train3DModel, TrainPathInformation};
use crate::{
    landscape::{HeightMap, OriginOffset, Path},
    train::{Direction, Speed},
    HEIGHT_OFFSET, TRAIN_HEIGHT_OFFSET,
};
use bevy::prelude::*;

const SPEED: f32 = 3.0; // m/s

pub fn system(
    paths: Query<(Entity, &Path)>,
    mut transforms: Query<&mut Transform>,
    mut trains: Query<(Entity, &Speed, &mut Train3DModel)>,
    time: Res<Time>,
    height_map: Res<HeightMap>,
    origin_offset: Res<OriginOffset>,
) {
    for (entity, speed, mut model) in trains.iter_mut() {
        if let Some(path) = &mut model.path {
            if speed.0.abs() > f32::EPSILON {
                let delta_distance = speed.0 * time.delta_seconds();
                path.distance += delta_distance;

                let (_rail_entity, rail) = paths.get(path.entity).unwrap();
                let mut train_transform = transforms.get_mut(entity).unwrap();

                let diff = rail.end_coords - rail.start_coords;
                let length = diff.length();
                let diff = diff / length;

                let dest = rail.start_coords + diff * path.distance as f64;

                let dest = dest - origin_offset.0;

                // need to move to next rail segment
                if (length < path.distance as f64) || path.distance < 0.0 {
                    let direction = if path.distance < 0.0 {
                        Direction::Backward
                    } else {
                        Direction::Forward
                    };

                    let next = rail.possible_connections_by_direction(direction);
                    let (id, direction) = next.first().unwrap();

                    // TODO: path can also not be spawned...
                    let (next_entity, next_rail) =
                        paths.iter().find(|(_e, p)| p.id() == *id).unwrap();

                    path.entity = next_entity;
                    path.direction = *direction;

                    // TODO: we lose a bit of the overshooting when switching segments
                    // probably you just want to subtract things here
                    path.distance = match direction {
                        Direction::Forward => 0.0,
                        Direction::Backward => {
                            // connected to end, we start at max distance
                            (next_rail.end_coords - next_rail.start_coords)
                                .length()
                                .abs() as f32
                        }
                    };
                }

                // calculate height

                let height = height_map
                    .height_at_position(dest.0 + origin_offset.0 .0, dest.1 + origin_offset.0 .1);
                let height = height + HEIGHT_OFFSET + TRAIN_HEIGHT_OFFSET;

                let target = Vec3::new(dest.0 as f32, height, -dest.1 as f32);

                let diff = target - train_transform.translation;

                train_transform.translation +=
                    (diff * SPEED * time.delta_seconds()).clamp(-diff.abs(), diff);
            }
        }
    }
}
