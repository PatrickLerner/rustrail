#[cfg(test)]
mod tests;

use crate::train::{EngineOrWagons, ForceFriction, Mass};
use bevy::prelude::*;

pub fn system(mut entries: Query<(&mut ForceFriction, &Mass), EngineOrWagons>) {
    let my_rolling = 0.002;
    let g = 9.81;

    for (mut friction, mass) in entries.iter_mut() {
        let n = mass.0 * g;
        friction.0 = my_rolling * n;
    }
}
