#[cfg(test)]
mod tests;

use crate::train::{Direction, Engine, ForceDriving, MaxPower, Speed, ThrottleLever};
use bevy::prelude::*;

use super::BrakeLever;

pub fn system(
    mut entries: Query<
        (
            &mut ForceDriving,
            &MaxPower,
            &Speed,
            &ThrottleLever,
            &BrakeLever,
        ),
        With<Engine>,
    >,
) {
    for (mut force_driving, max_power, speed, throttle_lever, brake_lever) in entries.iter_mut() {
        force_driving.0 = if brake_lever.release_valve > 0.0 || brake_lever.engine_brake > 0.0 {
            0.0
        } else {
            let direction = match throttle_lever.direction {
                Direction::Forward => 1.0,
                Direction::Backward => -1.0,
            };

            direction * (max_power.0 * 1000.0 * throttle_lever.percentage) / speed.0.abs().max(1.0)
        }
    }
}
