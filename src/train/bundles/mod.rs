#[cfg(test)]
mod tests;

use super::{Name, *};
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Bundle, Default)]
pub struct TrainBundle {
    name: Name,
    composition: TrainComposition,
    speed: Speed,
    max_speed: MaxSpeed,
    mass: Mass,
    acceleration: Acceleration,
    distance: Distance,
    force_driving: ForceDriving,
    force_braking: ForceBraking,
    force_friction: ForceFriction,
    force_air_resistance: ForceAirResistance,
    air_pressure: AirPressure,
}

impl TrainBundle {
    pub fn new(name: &str, components: Vec<TrainComponent>) -> Self {
        Self {
            name: Name(name.to_string()),
            composition: TrainComposition { components },
            ..default()
        }
    }
}

#[derive(Bundle, Default)]
pub struct EngineBundle {
    name: Name,
    mass: Mass,
    max_power: MaxPower,
    max_speed: MaxSpeed,
    speed: Speed,
    dimension: Dimension,
    throttle_lever: ThrottleLever,
    brake_lever: BrakeLever,
    force_driving: ForceDriving,
    force_braking: ForceBraking,
    force_friction: ForceFriction,
    force_air_resistance: ForceAirResistance,
    load_model_file: LoadModelFile,
    air_pressure: AirPressure,
    air_pressure_delta: AirPressureDelta,
}

#[derive(Deserialize)]
struct EngineData {
    file_name: String,
    max_speed: f32,
    mass: Mass,
    max_power: MaxPower,
    dimension: Dimension,
}

impl EngineBundle {
    pub fn from_file(file_name: &str) -> Self {
        let data = std::fs::read_to_string(file_name).expect("file to be readable");
        let data: EngineData = toml::from_str(&data).expect("engine to be loadable");

        Self {
            load_model_file: LoadModelFile(format!("models/{}", data.file_name)),
            max_speed: MaxSpeed::from_kmh(data.max_speed),
            mass: data.mass,
            max_power: data.max_power,
            dimension: data.dimension,
            ..default()
        }
    }
}

#[derive(Bundle, Default)]
pub struct WagonBundle {
    mass: Mass,
    max_speed: MaxSpeed,
    speed: Speed,
    dimension: Dimension,
    force_friction: ForceFriction,
    force_air_resistance: ForceAirResistance,
    load_model_file: LoadModelFile,
    air_pressure: AirPressure,
}

#[derive(Deserialize)]
struct WagonData {
    file_name: String,
    max_speed: f32,
    mass: Mass,
    dimension: Dimension,
}

impl WagonBundle {
    pub fn from_file(file_name: &str) -> Self {
        let data = std::fs::read_to_string(file_name).expect("file to be readable");
        let data: WagonData = toml::from_str(&data).expect("engine to be loadable");

        Self {
            load_model_file: LoadModelFile(format!("models/{}", data.file_name)),
            max_speed: MaxSpeed::from_kmh(data.max_speed),
            mass: data.mass,
            dimension: data.dimension,
            ..default()
        }
    }
}
