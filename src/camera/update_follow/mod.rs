#[cfg(test)]
mod tests;

use super::GameCameraState;
use bevy::prelude::*;

const SPEED: f32 = 3.0; // m/s

pub fn system(
    mut q_camera: Query<&mut GameCameraState>,
    following: Query<&Transform>,
    time: Res<Time>,
) {
    for mut state in &mut q_camera {
        if let Some(follow) = state.follow {
            let transform = following.get(follow).unwrap();

            let diff = transform.translation - state.center;
            state.center += (SPEED * diff * time.delta_seconds()).clamp(-diff.abs(), diff.abs());
        }
    }
}
