#[cfg(test)]
mod tests;

use super::{Acceleration, MaxSpeed, Speed};
use bevy::prelude::*;

pub fn system(mut entries: Query<(&mut Speed, &MaxSpeed, &Acceleration)>, time: Res<Time>) {
    for (mut speed, max_speed, acceleration) in entries.iter_mut() {
        speed.0 += acceleration.0 * time.delta_seconds();
        speed.0 = speed.0.clamp(-max_speed.0, max_speed.0);
    }
}
