mod update_acceleration;
mod update_drive_force;

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

#[derive(Debug)]
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

fn update_braking_force(mut entries: Query<(&mut ForceBraking, &Mass, &BrakeLever)>) {
    let friction_coefficient = 0.3;
    let g = 9.81;

    for (mut friction, mass, brake_lever) in entries.iter_mut() {
        let n = mass.total() * g;
        friction.0 = friction_coefficient * n * brake_lever.percentage;
    }
}

fn update_friction(mut entries: Query<(&mut ForceFriction, &Mass)>) {
    let my_rolling = 0.002;
    let g = 9.81;

    for (mut friction, mass) in entries.iter_mut() {
        let n = mass.total() * g;
        friction.0 = my_rolling * n;
    }
}

fn update_air_resistance(mut entries: Query<(&mut ForceAirResistance, &Speed)>) {
    let air_density = 1.225; // kg/m^3
    let drag_coefficient = 0.8;
    let frontal_area = 10.0; // m^2

    for (mut air_resistance, speed) in entries.iter_mut() {
        air_resistance.0 = 0.5 * air_density * speed.0.powi(2) * drag_coefficient * frontal_area;
    }
}

fn update_speed(mut entries: Query<(&mut Speed, &MaxSpeed, &Acceleration)>, time: Res<Time>) {
    for (mut speed, max_speed, acceleration) in entries.iter_mut() {
        speed.0 += acceleration.0 * time.delta_seconds();
        speed.0 = speed.0.clamp(-max_speed.0, max_speed.0);
    }
}

fn update_distance(mut entries: Query<(&mut Distance, &Speed)>, time: Res<Time>) {
    for (mut distance, speed) in entries.iter_mut() {
        distance.0 += speed.0 * time.delta_seconds();
    }
}

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
    pub fn br_218(name: String, wagon_mass: f32) -> Self {
        Self {
            name: Name(name),
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
                update_braking_force,
                update_friction,
                update_air_resistance,
                update_acceleration::system,
                update_speed,
                update_distance,
            ),
        );
    }
}
