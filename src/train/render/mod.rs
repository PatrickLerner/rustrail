#[cfg(test)]
mod tests;

mod move_train;
mod spawn_train;

use super::Direction;
use crate::landscape::Path;
use bevy::prelude::*;

#[derive(Component)]
struct TrainPathInformation {
    entity: Entity,
    direction: Direction,
    distance: f32,
}

#[derive(Component, Default)]
struct Train3DModel {
    path: Option<TrainPathInformation>,
}

// TODO
fn add_path(mut trains: Query<&mut Train3DModel>, paths: Query<(Entity, &Path)>) {
    for mut train in trains.iter_mut() {
        if train.path.is_some() {
            continue;
        }

        let result = paths.iter().find(|(_e, p)| p.start_id == 2029327559);

        if let Some((entity, _path)) = result {
            log::info!("placed train");
            train.path = Some(TrainPathInformation {
                entity,
                distance: 0.0,
                direction: Direction::Forward,
            });
        }
    }
}

pub struct TrainRenderPlugin;

impl Plugin for TrainRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_train::system, move_train::system, add_path));
    }
}
