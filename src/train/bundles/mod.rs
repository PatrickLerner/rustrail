#[cfg(test)]
mod tests;

use super::{Name, *};
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Bundle, Default)]
pub struct TrainBundle {
    pub name: Name,
    pub composition: TrainComposition,
    pub speed: Speed,
    pub max_speed: MaxSpeed,
    pub mass: Mass,
    pub acceleration: Acceleration,
    pub distance: Distance,
    pub force_driving: ForceDriving,
    pub force_braking: ForceBraking,
    pub force_friction: ForceFriction,
    pub force_air_resistance: ForceAirResistance,
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
    pub mass: Mass,
    pub max_speed: MaxSpeed,
    pub speed: Speed,
    pub dimension: Dimension,
    pub force_friction: ForceFriction,
    pub force_air_resistance: ForceAirResistance,
    pub load_model_file: LoadModelFile,
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
