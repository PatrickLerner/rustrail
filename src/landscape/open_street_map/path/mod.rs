use crate::{landscape::CoordinatePoint, train::Direction};
use bevy::prelude::*;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

pub type PathId = (i64, i64);

#[derive(Component, Default, Debug, Deserialize, Serialize, Clone)]
pub struct Path {
    pub start_id: i64,
    pub end_id: i64,
    pub start_coords: CoordinatePoint,
    pub end_coords: CoordinatePoint,
    pub forward_connections: Vec<(PathId, Direction)>,
    pub backwards_connections: Vec<(PathId, Direction)>,
}

impl Path {
    pub fn id(&self) -> PathId {
        (self.start_id, self.end_id)
    }
}

// TODO
#[allow(dead_code)]
impl Path {
    fn possible_connections_by_direction(&self, direction: Direction) -> &Vec<(PathId, Direction)> {
        match direction {
            Direction::Forward => &self.forward_connections,
            Direction::Backward => &self.backwards_connections,
        }
    }

    fn random_connection_by_direction(&self, direction: Direction) -> Option<&(PathId, Direction)> {
        let possible = self.possible_connections_by_direction(direction);

        if possible.is_empty() {
            return None;
        }

        let mut random = thread_rng();
        let index = random.gen_range(0..possible.len());

        Some(&possible[index])
    }
}
