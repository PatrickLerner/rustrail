#[cfg(test)]
mod tests;

mod move_train;
mod spawn_train;
mod update_train_location;

use crate::{landscape::PathId, train::Direction};
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct TrainRailLocation {
    id: PathId,
    distance: f64,
    travel_direction: Direction,
}

#[derive(Component, Default)]
pub struct Train3DModel;

pub struct TrainRenderPlugin;

impl Plugin for TrainRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_train::system,
                move_train::system,
                update_train_location::system,
            ),
        );
    }
}
