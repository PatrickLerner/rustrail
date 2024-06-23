#[cfg(test)]
mod tests;

use bevy::prelude::*;

pub fn system(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 255.0,
    });
}
