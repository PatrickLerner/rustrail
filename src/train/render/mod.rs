#[cfg(test)]
mod tests;

mod move_train_component;
mod spawn_train_component;

use bevy::prelude::*;

pub struct TrainRenderPlugin;

impl Plugin for TrainRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_train_component::system, move_train_component::system),
        );
    }
}
