#[cfg(test)]
mod tests;

mod bundles;
mod forces;
mod physics;
mod render;
mod track_location;

use bevy::{app::PluginGroupBuilder, prelude::*};
use serde::{Deserialize, Serialize};
use wrapped_value_derive_macro::WrappedValue;

pub use bundles::{EngineBundle, TrainBundle, WagonBundle};
pub use forces::{ForceAirResistance, ForceBraking, ForceDriving, ForceFriction};
pub use track_location::TrackLocation;

#[derive(Component, Default)]
pub struct Train;

#[derive(Component, Default)]
pub struct Engine;

#[derive(Component, Default)]
pub struct Wagon;

type EngineOrWagons = Or<(With<Engine>, With<Wagon>)>;

#[derive(Component, Default)]
pub struct Name(pub String);

#[derive(Component, Default, Debug)]
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

impl Speed {
    pub fn as_kmh(&self) -> f32 {
        self.0 * 3.6
    }
}

#[derive(Component, Default, Debug, WrappedValue)]
// m/s
pub struct MaxSpeed(pub f32);

impl MaxSpeed {
    pub fn from_kmh(value: f32) -> Self {
        Self(value / 3.6)
    }
}

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
    // 0..1
    pub percentage: f32,
    pub direction: Direction,
}

#[derive(Component, Default)]
pub struct BrakeLever {
    // 0..1
    pub release_valve: f32,
    // 0..1
    pub engine_brake: f32,
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

const MAX_AIR_PRESSURE: f32 = 5.0;

#[derive(Component, Default)]
// bar
pub struct AirPressure(pub f32);

impl WrappedValue for AirPressure {
    fn get(&self) -> f32 {
        self.0
    }

    fn set(&mut self, value: f32) {
        self.0 = value.clamp(0.0, MAX_AIR_PRESSURE);
    }
}

#[derive(Component, Default, WrappedValue)]
pub struct AirPressureDelta(pub f32);

pub enum TrainComponent {
    Engine(Entity),
    Wagon(Entity),
}

#[derive(Component, Default, Clone, Deserialize)]
pub struct Dimension {
    // m
    pub length: f32,
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

pub struct TrainPlugins;

impl PluginGroup for TrainPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(physics::TrainPhysicsPlugin)
            .add(render::TrainRenderPlugin)
    }
}
