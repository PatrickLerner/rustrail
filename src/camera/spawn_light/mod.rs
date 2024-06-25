#[cfg(test)]
mod tests;

use bevy::prelude::*;

pub fn system(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::hex("EEE3D6").unwrap(),
        brightness: 2.0 * 683.0,
    });
}
