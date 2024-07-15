#[cfg(test)]
mod tests;

use super::{AirPressure, EngineOrWagons, ForceBraking, Mass, MAX_AIR_PRESSURE};
use bevy::prelude::*;

pub fn system(mut entries: Query<(&mut ForceBraking, &Mass, &AirPressure), EngineOrWagons>) {
    let friction_coefficient = 0.3;
    let g = 9.81;

    for (mut braking, mass, air_pressure) in entries.iter_mut() {
        let n = mass.0 * g;
        let pressure_percentage = (MAX_AIR_PRESSURE - air_pressure.0) / MAX_AIR_PRESSURE;
        braking.0 = friction_coefficient * n * pressure_percentage;
        log::info!("pressure {:?}; brake {:?}", air_pressure.0, braking.0);
    }
}
