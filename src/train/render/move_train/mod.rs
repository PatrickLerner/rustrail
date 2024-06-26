#[cfg(test)]
mod tests;

use super::Train3DModel;
use crate::{
    landscape::{HeightMap, OriginOffset},
    train::Speed,
    HEIGHT_OFFSET, TRAIN_HEIGHT_OFFSET,
};
use bevy::prelude::*;

const SPEED: f32 = 3.0; // m/s

pub fn system(
    mut trains: Query<(&mut Transform, &Speed), With<Train3DModel>>,
    time: Res<Time>,
    height_map: Res<HeightMap>,
    origin_offset: Res<OriginOffset>,
) {
    for (mut transform, speed) in trains.iter_mut() {
        if speed.0.abs() > f32::EPSILON {
            transform.translation.x += speed.0 * time.delta_seconds();

            let h = height_map.height_at_position(
                transform.translation.x as f64 + origin_offset.0 .0,
                transform.translation.z as f64 + origin_offset.0 .1,
            );

            let target = h + HEIGHT_OFFSET + TRAIN_HEIGHT_OFFSET;
            let diff = target - transform.translation.y;

            transform.translation.y +=
                (SPEED * diff * time.delta_seconds()).clamp(-diff.abs(), diff.abs());
        }
    }
}
