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

// ref: https://de.wikipedia.org/wiki/RAL-Eisenbahnfarben
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PaintSchemeColor {
    Verkehrsrot,
    Orientrot,
    Lichtgrau,
    Achatgrau,
    Fernblau,
    Ozeanblau,
    Minttuerkis,
    Pasteltuerkis,
    Lachsorange,
    Tiefschwarz,
}

impl From<PaintSchemeColor> for Color {
    fn from(value: PaintSchemeColor) -> Self {
        match value {
            PaintSchemeColor::Verkehrsrot => Color::hex("C1121C").unwrap(),
            PaintSchemeColor::Orientrot => Color::hex("A7323E").unwrap(),
            PaintSchemeColor::Lichtgrau => Color::hex("D7D7D7").unwrap(),
            PaintSchemeColor::Achatgrau => Color::hex("B1B3AF").unwrap(),
            PaintSchemeColor::Fernblau => Color::hex("486590").unwrap(),
            PaintSchemeColor::Ozeanblau => Color::hex("2A5059").unwrap(),
            PaintSchemeColor::Minttuerkis => Color::hex("3F8884").unwrap(),
            PaintSchemeColor::Pasteltuerkis => Color::hex("74AEB1").unwrap(),
            PaintSchemeColor::Lachsorange => Color::hex("DB6A50").unwrap(),
            PaintSchemeColor::Tiefschwarz => Color::hex("151619").unwrap(),
        }
    }
}

impl Default for PaintSchemeColor {
    fn default() -> Self {
        Self::Verkehrsrot
    }
}

#[derive(Component, Default, Debug)]
pub struct PaintScheme {
    pub color: PaintSchemeColor,
}

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

#[derive(Component, Default, Debug, WrappedValue)]
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

#[derive(Component, Default)]
// kW
pub struct MaxPower(pub f32);

#[derive(Component, Default, WrappedValue)]
// kg
pub struct Mass(pub f32);

#[derive(Component, Default)]
// m
pub struct Distance(pub f32);

#[derive(Component, Default, Clone)]
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
    pub paint_scheme: PaintScheme,
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
    paint_scheme: PaintScheme,
}

impl EngineBundle {
    pub fn br_218(name: &str) -> Self {
        Self {
            name: Name(name.to_owned()),
            mass: Mass(78_000.0),
            max_power: MaxPower(1839.0),
            max_speed: MaxSpeed(140.0 / 3.6),
            dimension: Dimension { length: 16.4 },
            paint_scheme: PaintScheme {
                color: PaintSchemeColor::Pasteltuerkis,
            },
            ..Default::default()
        }
    }

    pub fn br_111(name: &str) -> Self {
        Self {
            name: Name(name.to_owned()),
            mass: Mass(83_000.0),
            max_power: MaxPower(3700.0),
            max_speed: MaxSpeed(160.0 / 3.6),
            dimension: Dimension { length: 16.75 },
            paint_scheme: PaintScheme {
                color: PaintSchemeColor::Verkehrsrot,
            },
            ..Default::default()
        }
    }

    pub fn br_110(name: &str) -> Self {
        Self {
            name: Name(name.to_owned()),
            mass: Mass(85_000.0),
            max_power: MaxPower(3620.0),
            max_speed: MaxSpeed(150.0 / 3.6),
            dimension: Dimension { length: 16.49 },
            paint_scheme: PaintScheme {
                color: PaintSchemeColor::Verkehrsrot,
            },
            ..Default::default()
        }
    }

    pub fn br_89(name: &str) -> Self {
        Self {
            name: Name(name.to_owned()),
            mass: Mass(46_600.0),
            max_power: MaxPower(235.0),
            max_speed: MaxSpeed(45.0 / 3.6),
            dimension: Dimension { length: 9.6 },
            paint_scheme: PaintScheme {
                color: PaintSchemeColor::Tiefschwarz,
            },
            ..Default::default()
        }
    }

    pub fn ice(name: &str) -> Self {
        Self {
            name: Name(name.to_owned()),
            mass: Mass(80_000.0),
            max_power: MaxPower(4790.0),
            max_speed: MaxSpeed(280.0 / 3.6),
            dimension: Dimension { length: 20.56 },
            paint_scheme: PaintScheme {
                color: PaintSchemeColor::Lichtgrau,
            },
            ..Default::default()
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
