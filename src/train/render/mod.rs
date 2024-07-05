#[cfg(test)]
mod tests;

mod move_train_component;
mod spawn_engine;
mod update_train_location;

use crate::{
    landscape::{OSMData, PathId},
    train::Direction,
};
use bevy::prelude::*;

#[derive(Component, Default, Clone, Debug)]
pub struct TrainRailLocation {
    id: PathId,
    distance: f64,
    travel_direction: Direction,
}

impl TrainRailLocation {
    fn add_distance(&mut self, data: &OSMData, amount: f64) {
        self.distance += amount;

        // when we add or subtract distance we might change id or travel direction
        // so here we will fix it
        loop {
            let rail = data
                .rails
                .get(&self.id)
                .expect("train location to be valid");

            let length = rail.length();

            let crossing_direction = if length < self.distance {
                Some(Direction::Forward)
            } else if self.distance < 0.0 {
                Some(Direction::Backward)
            } else {
                None
            };

            // need to move to next rail segment
            if let Some(crossing_direction) = crossing_direction {
                let old_travel_direction = self.travel_direction;
                let possible = rail.possible_connections_by_direction(match crossing_direction {
                    Direction::Forward => self.travel_direction,
                    Direction::Backward => self.travel_direction.opposite(),
                });
                let (next_id, next_direction) = possible
                    .first()
                    .expect("at least one possible followup location to exist");
                let next_rail = data.rails.get(next_id).expect("location to be valid");

                self.id = *next_id;
                self.travel_direction = match crossing_direction {
                    Direction::Forward => *next_direction,
                    Direction::Backward => next_direction.opposite(),
                };
                self.distance += match crossing_direction {
                    Direction::Forward => -length,
                    Direction::Backward => next_rail.length(),
                };

                log::debug!("=== new rail segment ===");
                log::debug!("path.distance: {:?}", self.distance);
                log::debug!("current id: {:?}", rail.id());
                log::debug!("next id: {:?}", next_id);
                log::debug!("current travel direction: {:?}", old_travel_direction);
                log::debug!("next travel direction: {:?}", self.travel_direction);
                log::debug!("=== new rail segment end ===");
            } else {
                break;
            }
        }
    }
}

#[derive(Component, Default)]
pub struct Train3DModel;

pub struct TrainRenderPlugin;

impl Plugin for TrainRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_engine::system,
                move_train_component::system,
                update_train_location::system,
            ),
        );
    }
}
