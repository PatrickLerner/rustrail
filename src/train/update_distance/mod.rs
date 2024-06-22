#[cfg(test)]
mod tests;

use super::{Distance, Speed};
use bevy::prelude::*;

pub fn system(mut entries: Query<(&mut Distance, &Speed)>, time: Res<Time>) {
    for (mut distance, speed) in entries.iter_mut() {
        distance.0 += speed.0 * time.delta_seconds();
    }
}
