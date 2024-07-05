#[cfg(test)]
mod tests;

mod move_train_component;
mod spawn_engine;

use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Train3DModel;

pub struct TrainRenderPlugin;

impl Plugin for TrainRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_engine::system, move_train_component::system));
    }
}
