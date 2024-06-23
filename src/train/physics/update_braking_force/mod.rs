#[cfg(test)]
mod tests;

use crate::train::{BrakeLever, ForceBraking, Mass};
use bevy::prelude::*;

pub fn system(mut entries: Query<(&mut ForceBraking, &Mass, &BrakeLever)>) {
    let friction_coefficient = 0.3;
    let g = 9.81;

    for (mut friction, mass, brake_lever) in entries.iter_mut() {
        let n = mass.total() * g;
        friction.0 = friction_coefficient * n * brake_lever.percentage;
    }
}
