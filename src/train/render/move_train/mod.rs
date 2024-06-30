#[cfg(test)]
mod tests;

use super::TrainRailLocation;
use crate::{
    landscape::{HeightMap, OSMData, OriginOffset},
    train::Direction,
    HEIGHT_OFFSET, TRAIN_HEIGHT_OFFSET,
};
use bevy::prelude::*;

const SPEED: f32 = 3.0; // m/s

pub fn system(
    data: Res<OSMData>,
    mut trains: Query<(&TrainRailLocation, &mut Transform)>,
    time: Res<Time>,
    height_map: Res<HeightMap>,
    origin_offset: Res<OriginOffset>,
) {
    for (location, mut transform) in trains.iter_mut() {
        let rail = data
            .rails
            .get(&location.id)
            .expect("train location to be valid");

        let (s, e) = match location.travel_direction {
            Direction::Forward => (rail.start_coords, rail.end_coords),
            Direction::Backward => (rail.end_coords, rail.start_coords),
        };

        let diff = e - s;
        let length = rail.length();
        let diff = diff / length;

        let dest = s + diff * location.distance;
        let dest = dest - origin_offset.0;

        // calculate height

        let height =
            height_map.height_at_position(dest.0 + origin_offset.0 .0, dest.1 + origin_offset.0 .1);
        let height = height + HEIGHT_OFFSET + TRAIN_HEIGHT_OFFSET;

        let target = Vec3::new(dest.0 as f32, height, -dest.1 as f32);
        let diff = target - transform.translation;
        transform.translation += (diff * SPEED * time.delta_seconds()).clamp(-diff.abs(), diff);
    }
}
