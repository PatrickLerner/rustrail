#[cfg(test)]
mod tests;

use super::GameCameraState;
use bevy::prelude::*;

pub fn system(mut q_camera: Query<&mut GameCameraState>, transform: Query<&Transform>) {
    for mut state in &mut q_camera {
        if let Some(follow) = state.follow {
            let transform = transform.get(follow).unwrap();
            state.center = transform.translation;
        }
    }
}
