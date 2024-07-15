#[cfg(test)]
mod tests;

use crate::train::{AirPressure, AirPressureDelta, BrakeLever, MAX_AIR_PRESSURE};
use bevy::prelude::*;

const COMPRESSOR_SPEED: f32 = 0.6;

pub fn system(
    mut entries: Query<(&mut AirPressureDelta, &AirPressure, &BrakeLever)>,
    time: Res<Time>,
) {
    for (mut air_pressure_delta, air_pressure, brake_lever) in entries.iter_mut() {
        air_pressure_delta.0 = if brake_lever.release_valve > 0.0 {
            let target = MAX_AIR_PRESSURE * (1.0 - brake_lever.release_valve);
            (target - air_pressure.0).min(COMPRESSOR_SPEED * time.delta_seconds())
        } else {
            COMPRESSOR_SPEED * time.delta_seconds()
        };
    }
}
