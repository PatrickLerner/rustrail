#[cfg(test)]
mod tests;

use super::GameCameraBundle;
use bevy::prelude::*;

pub fn system(mut commands: Commands) {
    let mut camera = GameCameraBundle::default();
    // Position our camera using our component,
    // not Transform (it would get overwritten)
    camera.state.center = Vec3::new(1.0, 10.0, 3.0);
    camera.state.radius = 50.0;
    camera.state.pitch = -30.0f32.to_radians();
    camera.state.yaw = 30.0f32.to_radians();
    commands.spawn(camera);
}
