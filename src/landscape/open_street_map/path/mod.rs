#[cfg(test)]
mod tests;

use crate::{landscape::CoordinatePoint, train::Direction};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub type PathId = (i64, i64);

#[derive(Component, Default, Debug, Deserialize, Serialize, Clone)]
pub struct Path {
    pub start_id: i64,
    pub end_id: i64,
    pub start_coords: CoordinatePoint,
    pub end_coords: CoordinatePoint,
    pub forward_connections: Vec<(PathId, Direction)>,
    pub backward_connections: Vec<(PathId, Direction)>,
}

impl Path {
    pub fn id(&self) -> PathId {
        (self.start_id, self.end_id)
    }
}

/*
impl Path {
    fn possible_connections_by_direction(&self, direction: Direction) -> &Vec<(PathId, Direction)> {
        match direction {
            Direction::Forward => &self.forward_connections,
            Direction::Backward => &self.backward_connections,
        }
    }
}
*/
