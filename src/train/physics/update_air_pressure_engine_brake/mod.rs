#[cfg(test)]
mod tests;

use crate::train::{AirPressure, BrakeLever};
use bevy::prelude::*;

pub fn system(mut entries: Query<(&mut AirPressure, &BrakeLever)>) {
    for (mut air_pressure, brake_lever) in entries.iter_mut() {
        let value = brake_lever.engine_brake.powi(2);
        air_pressure.0 *= 1.0 - value;
    }
}