#[cfg(test)]
mod tests;

use crate::train::{Direction, ForceDriving, MaxPower, Speed, ThrottleLever};
use bevy::prelude::*;

pub fn system(mut entries: Query<(&mut ForceDriving, &MaxPower, &Speed, &ThrottleLever)>) {
    for (mut force_driving, max_power, speed, throttle_lever) in entries.iter_mut() {
        let direction = match throttle_lever.direction {
            Direction::Forward => 1.0,
            Direction::Backward => -1.0,
        };

        force_driving.0 =
            direction * (max_power.0 * 1000.0 * throttle_lever.percentage) / speed.0.abs().max(1.0);
    }
}
