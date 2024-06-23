#[cfg(test)]
mod tests;

mod move_train;
mod spawn_train;

use bevy::prelude::*;

#[derive(Component)]
struct Train3DModel;

pub struct TrainRenderPlugin;

impl Plugin for TrainRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_train::system, move_train::system));
    }
}
