#[cfg(test)]
mod tests;

use crate::train::{ForceAirResistance, Speed};
use bevy::prelude::*;

pub fn system(mut entries: Query<(&mut ForceAirResistance, &Speed)>) {
    let air_density = 1.225; // kg/m^3
    let drag_coefficient = 0.8;
    let frontal_area = 10.0; // m^2

    for (mut air_resistance, speed) in entries.iter_mut() {
        air_resistance.0 = 0.5 * air_density * speed.0.powi(2) * drag_coefficient * frontal_area;
    }
}
