#[cfg(test)]
mod tests;

use bevy::prelude::*;

pub fn system(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Srgba::hex("EEE3D6").unwrap().into(),
        brightness: 2.0 * 683.0,
    });
}
