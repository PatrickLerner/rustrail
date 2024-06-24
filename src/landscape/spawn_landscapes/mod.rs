#[cfg(test)]
mod tests;

use super::{Landscape, OriginOffset, LANDSCAPE_SIZE, SPAWN_RADIUS};
use crate::{camera::GameCameraState, landscape::DEFAULT_TTL};
use bevy::prelude::*;

pub fn system(
    mut commands: Commands,
    mut landscapes: Query<&mut Landscape>,
    cameras: Query<&GameCameraState>,
    origin_offset: Res<OriginOffset>,
) {
    let grid_half_length = (LANDSCAPE_SIZE / 2) as f64;

    for camera in cameras.iter() {
        for dx in -SPAWN_RADIUS..SPAWN_RADIUS {
            for dy in -SPAWN_RADIUS..SPAWN_RADIUS {
                let x = (((camera.center.x as f64 + grid_half_length) / (2.0 * grid_half_length))
                    .floor()
                    + dx as f64)
                    * (2.0 * grid_half_length);
                let y = (((camera.center.z as f64 + grid_half_length) / (2.0 * grid_half_length))
                    .floor()
                    + dy as f64)
                    * (2.0 * grid_half_length);

                let desired_x = x + origin_offset.x;
                let desired_y = y + origin_offset.y;

                if let Some(mut landscape) = landscapes
                    .iter_mut()
                    .find(|l| l.x == desired_x && l.y == desired_y)
                {
                    landscape.ttl = DEFAULT_TTL;
                } else {
                    #[cfg(not(coverage))]
                    log::debug!("Requesting landscape at {:?}", (x as i32, y as i32));

                    commands.spawn(Landscape {
                        ttl: DEFAULT_TTL,
                        x: desired_x,
                        y: desired_y,
                    });
                }
            }
        }
    }
}
