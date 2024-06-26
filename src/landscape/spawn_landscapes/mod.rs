#[cfg(test)]
mod tests;

use super::{
    CoordinatePoint, Landscape, OriginOffset, HALF_LANDSCAPE_SIZE, LANDSCAPE_SIZE, SPAWN_RADIUS,
};
use crate::{camera::GameCameraState, landscape::DEFAULT_TTL};
use bevy::prelude::*;

pub fn system(
    mut commands: Commands,
    mut landscapes: Query<&mut Landscape>,
    cameras: Query<&GameCameraState>,
    origin_offset: Res<OriginOffset>,
) {
    let grid_half_length = HALF_LANDSCAPE_SIZE as f64;
    let grid_length = LANDSCAPE_SIZE as f64;

    for camera in cameras.iter() {
        // NOTE: - on z due to bevy's inane projection
        let camera = CoordinatePoint(camera.center.x as f64, -camera.center.z as f64);

        for dx in -SPAWN_RADIUS..SPAWN_RADIUS {
            for dy in -SPAWN_RADIUS..SPAWN_RADIUS {
                let diff = CoordinatePoint(dx as f64, dy as f64);
                let desired = (((camera + grid_half_length) / grid_length).floor() + diff)
                    * grid_length
                    + origin_offset.0;

                if let Some(mut landscape) = landscapes.iter_mut().find(|l| l.position == desired) {
                    landscape.ttl = DEFAULT_TTL;
                } else {
                    #[cfg(not(coverage))]
                    log::debug!(
                        "Requesting landscape at {:?}",
                        (desired.0 as i32, desired.1 as i32)
                    );

                    let position = desired - origin_offset.0;

                    commands.spawn((
                        Landscape {
                            position: desired,
                            ..default()
                        },
                        PbrBundle {
                            transform: Transform::from_xyz(
                                position.0 as f32,
                                0.0,
                                // NOTE: -z due to bevy's inane projection
                                -position.1 as f32,
                            ),
                            ..default()
                        },
                    ));
                }
            }
        }
    }
}
