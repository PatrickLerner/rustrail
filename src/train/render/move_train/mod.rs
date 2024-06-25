#[cfg(test)]
mod tests;

use super::Train3DModel;
use crate::{
    landscape::{HeightMap, OriginOffset},
    train::Speed,
    HEIGHT_OFFSET, TRAIN_HEIGHT_OFFSET,
};
use bevy::prelude::*;

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
                transform.translation.x as f64 + origin_offset.x,
                transform.translation.z as f64 + origin_offset.y,
            );

            let target = h + HEIGHT_OFFSET + TRAIN_HEIGHT_OFFSET;
            let diff = target - transform.translation.y;

            transform.translation.y += (diff * time.delta_seconds()).clamp(-diff.abs(), diff.abs());
        }
    }
}
