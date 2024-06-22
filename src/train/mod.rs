#[cfg(test)]
mod tests;

mod update_acceleration;
mod update_air_resistance;
mod update_braking_force;
mod update_distance;
mod update_drive_force;
mod update_friction;
mod update_speed;

use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Name(pub String);

// ref: https://de.wikipedia.org/wiki/RAL-Eisenbahnfarben
#[derive(Copy, Clone)]
pub enum PaintSchemeColor {
    Verkehrsrot,
    Orientrot,
    Lichtgrau,
    Fernblau,
    Ozeanblau,
    Minttuerkis,
    Pasteltuerkis,
    Lachsorange,
}

impl From<PaintSchemeColor> for Color {
    fn from(value: PaintSchemeColor) -> Self {
        match value {
            PaintSchemeColor::Verkehrsrot => Color::hex("C1121C").unwrap(),
            PaintSchemeColor::Orientrot => Color::hex("A7323E").unwrap(),
            PaintSchemeColor::Lichtgrau => Color::hex("D7D7D7").unwrap(),
            PaintSchemeColor::Fernblau => Color::hex("486590").unwrap(),
            PaintSchemeColor::Ozeanblau => Color::hex("2A5059").unwrap(),
            PaintSchemeColor::Minttuerkis => Color::hex("3F8884").unwrap(),
            PaintSchemeColor::Pasteltuerkis => Color::hex("74AEB1").unwrap(),
            PaintSchemeColor::Lachsorange => Color::hex("DB6A50").unwrap(),
        }
    }
}

impl Default for PaintSchemeColor {
    fn default() -> Self {
        Self::Verkehrsrot
    }
}

#[derive(Component, Default)]
pub struct PaintScheme {
    pub color: PaintSchemeColor,
}

#[derive(Component, Default)]
// m/s^2
pub struct Acceleration(pub f32);

#[derive(Component, Default)]
// m/s
pub struct Speed(pub f32);

#[derive(Component, Default)]
// m/s
pub struct MaxSpeed(pub f32);

#[derive(Component, Default)]
// N
pub struct ForceDriving(pub f32);

#[derive(Component, Default)]
// N
pub struct ForceBraking(pub f32);

#[derive(Debug, PartialEq)]
pub enum Direction {
    Forward,
    Backward,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Forward
    }
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Forward => Self::Backward,
            Self::Backward => Self::Forward,
        }
    }
}

#[derive(Component, Default)]
pub struct ThrottleLever {
    // -1..1
    pub percentage: f32,
    pub direction: Direction,
}

#[derive(Component, Default)]
pub struct BrakeLever {
    // -1..1
    pub percentage: f32,
}

#[derive(Component, Default)]
// kW
pub struct MaxPower(pub f32);

#[derive(Component, Default)]
// N
pub struct ForceFriction(pub f32);

#[derive(Component, Default)]
// N
pub struct ForceAirResistance(pub f32);

#[derive(Component, Default)]
// kg
pub struct Mass {
    pub engine: f32,
    pub wagons: f32,
}

impl Mass {
    pub fn total(&self) -> f32 {
        self.engine + self.wagons
    }
}

#[derive(Component, Default)]
// m
pub struct Distance(pub f32);

#[derive(Bundle, Default)]
pub struct TrainBundle {
    name: Name,
    speed: Speed,
    acceleration: Acceleration,
    mass: Mass,
    max_power: MaxPower,
    max_speed: MaxSpeed,
    throttle_lever: ThrottleLever,
    brake_lever: BrakeLever,
    force_driving: ForceDriving,
    force_braking: ForceBraking,
    force_friction: ForceFriction,
    force_air_resistance: ForceAirResistance,
    distance: Distance,
    paint_scheme: PaintScheme,
}

impl TrainBundle {
    pub fn br_218(name: &str, wagon_mass: f32) -> Self {
        Self {
            name: Name(name.to_owned()),
            mass: Mass {
                engine: 78_000.0,
                wagons: wagon_mass,
            },
            max_power: MaxPower(1839.0),
            max_speed: MaxSpeed(140.0 / 3.6),
            ..Default::default()
        }
    }
}

pub struct TrainPlugin;

impl Plugin for TrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_drive_force::system,
                update_braking_force::system,
                update_friction::system,
                update_air_resistance::system,
                update_acceleration::system,
                update_speed::system,
                update_distance::system,
            ),
        );
    }
}
