#[cfg(test)]
mod tests;

mod physics;
mod render;
mod track_location;

use bevy::{app::PluginGroupBuilder, prelude::*};
use serde::{Deserialize, Serialize};
use wrapped_value_derive_macro::WrappedValue;

pub use track_location::TrackLocation;

#[derive(Component, Default)]
pub struct Name(pub String);

#[derive(Component, Default, Debug, Deserialize)]
pub struct LoadModelFile(pub String);

trait WrappedValue {
    fn get(&self) -> f32;
    fn set(&mut self, value: f32);
}

#[derive(Component, Default, Debug)]
// m/s^2
pub struct Acceleration(pub f32);

#[derive(Component, Default, Debug, WrappedValue)]
// m/s
pub struct Speed(pub f32);

#[derive(Component, Default, Debug, WrappedValue, Deserialize)]
// m/s
pub struct MaxSpeed(pub f32);

#[derive(Component, Default, Debug, WrappedValue)]
// N
pub struct ForceDriving(pub f32);

#[derive(Component, Default, Debug, WrappedValue)]
// N
pub struct ForceBraking(pub f32);

#[derive(Component, Default, Debug, WrappedValue)]
// N
pub struct ForceFriction(pub f32);

#[derive(Component, Default, Debug, WrappedValue)]
// N
pub struct ForceAirResistance(pub f32);

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
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

#[derive(Component, Default, Deserialize)]
// kW
pub struct MaxPower(pub f32);

#[derive(Component, Default, WrappedValue, Deserialize)]
// kg
pub struct Mass(pub f32);

#[derive(Component, Default)]
// m
pub struct Distance(pub f32);

#[derive(Component, Default, Clone, Deserialize)]
pub struct Dimension {
    // m
    pub length: f32,
}

pub enum TrainComponent {
    Engine(Entity),
    Wagon(Entity),
}

#[derive(Component, Default)]
pub struct TrainComposition {
    pub components: Vec<TrainComponent>,
}

impl TrainComposition {
    fn entities(&self) -> Vec<Entity> {
        self.components
            .iter()
            .map(|component| match component {
                TrainComponent::Engine(entity) => *entity,
                TrainComponent::Wagon(entity) => *entity,
            })
            .collect()
    }
}

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
    max_speed: MaxSpeed,
    mass: Mass,
    dimension: Dimension,
}

impl WagonBundle {
    pub fn from_file(file_name: &str) -> Self {
        let data = std::fs::read_to_string(file_name).expect("file to be readable");
        let data: WagonData = toml::from_str(&data).expect("engine to be loadable");

        Self {
            load_model_file: LoadModelFile(format!("models/{}", data.file_name)),
            max_speed: data.max_speed,
            mass: data.mass,
            dimension: data.dimension,
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
}

#[derive(Deserialize)]
struct EngineData {
    file_name: String,
    max_speed: MaxSpeed,
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
            max_speed: data.max_speed,
            mass: data.mass,
            max_power: data.max_power,
            dimension: data.dimension,
            ..default()
        }
    }
}

pub struct TrainPlugins;

impl PluginGroup for TrainPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(physics::TrainPhysicsPlugin)
            .add(render::TrainRenderPlugin)
    }
}
