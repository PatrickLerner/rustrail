#[cfg(test)]
mod tests;

mod move_train;
mod spawn_train;

use crate::{
    landscape::{OSMData, PathId},
    train::Direction,
};
use bevy::prelude::*;

#[derive(Debug, PartialEq)]
pub enum Perspective {
    Normal,
    Inverse,
}

impl Perspective {
    fn opposite(&self) -> Self {
        match self {
            Self::Normal => Self::Inverse,
            Self::Inverse => Self::Normal,
        }
    }
}

#[derive(Component)]
pub struct TrainPathInformation {
    id: PathId,
    distance: f64,
    pub travel_direction: Direction,
}

#[derive(Component, Default)]
pub struct Train3DModel {
    pub path: Option<TrainPathInformation>,
}

// TODO
fn add_path(mut trains: Query<&mut Train3DModel>, data: Res<OSMData>) {
    for mut train in trains.iter_mut() {
        if train.path.is_some() {
            continue;
        }

        // TODO:
        log::info!("placed train");
        train.path = Some(TrainPathInformation {
            id: (1687958953, 10246601301),
            distance: 0.0,
            travel_direction: Direction::Forward,
        });
    }
}

pub struct TrainRenderPlugin;

impl Plugin for TrainRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_train::system, move_train::system, add_path));
    }
}
