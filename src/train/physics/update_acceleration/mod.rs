#[cfg(test)]
mod tests;

use crate::train::{
    Acceleration, ForceAirResistance, ForceBraking, ForceDriving, ForceFriction, Mass, Speed,
};
use bevy::prelude::*;

pub fn system(
    mut entries: Query<(
        &mut Acceleration,
        &Speed,
        &ForceDriving,
        &ForceFriction,
        &ForceAirResistance,
        &ForceBraking,
        &Mass,
    )>,
) {
    for (
        mut acceleration,
        speed,
        force_driving,
        force_friction,
        force_air_resistance,
        force_braking,
        mass,
    ) in entries.iter_mut()
    {
        let negative_force = force_friction.0 + force_air_resistance.0 + force_braking.0;
        let positive_force = force_driving.0.abs();

        // currently the driving force can be + or - and all other forces
        // are always positive (and against driving force). We derive the
        // direction here from the sign of driving force or speed to figure
        // out in which direction the other forces go
        let direction = if positive_force != 0.0 {
            force_driving.0.signum()
        } else {
            speed.0.signum()
        };

        let force = (positive_force - negative_force) * direction;
        acceleration.0 = force / mass.0;

        let sign = direction.signum();
        if sign * -acceleration.0 > sign * speed.0 {
            acceleration.0 = -speed.0;
        }
    }
}
