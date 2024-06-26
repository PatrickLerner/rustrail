#[cfg(test)]
mod tests;

mod despawn_landscapes;
mod height_map;
mod init_height_map;
mod open_street_map;
mod spawn_landscape_mesh;
mod spawn_landscapes;
mod spawn_rails;

use bevy::prelude::*;
pub use height_map::HeightMap;
use serde::{Deserialize, Serialize};

const TRIANGLE_SIZE: i32 = 10;
const LANDSCAPE_SIZE: i32 = 1000;
const HALF_LANDSCAPE_SIZE: i32 = LANDSCAPE_SIZE / 2;
const ORIGIN: (f64, f64) = (49.68134809269307, 8.61687829630227);
// lifetime of a landscape. if it is not renewed, it will despawn
const DEFAULT_TTL: f32 = 30.0;
const SPAWN_RADIUS: i32 = 5;
const MAX_SPAWN_PER_FRAME: usize = 3;

const MAX_RAIL_SEGMENT_LENGTH: f64 = 3.0;

#[derive(Component)]
pub struct Landscape {
    pub ttl: f32,
    pub position: CoordinatePoint,
}

impl Default for Landscape {
    fn default() -> Self {
        Self {
            ttl: DEFAULT_TTL,
            position: CoordinatePoint::default(),
        }
    }
}

type PathId = (i64, i64);

// TODO: merge with CoordinatePoint

#[derive(Resource)]
pub struct OriginOffset(pub CoordinatePoint);

// TODO: add some basic math operations

// TODO: move to file to test!
#[derive(Default, Debug, Deserialize, Serialize, Copy, Clone, PartialEq)]
pub struct CoordinatePoint(pub f64, pub f64);

impl From<CoordinatePoint> for Vec2 {
    fn from(val: CoordinatePoint) -> Self {
        Vec2::new(val.0 as f32, val.1 as f32)
    }
}

impl CoordinatePoint {
    fn sector_coordinates(&self) -> (i64, i64) {
        (
            (self.0 / LANDSCAPE_SIZE as f64 + 0.5).floor() as i64,
            (self.1 / LANDSCAPE_SIZE as f64 + 0.5).floor() as i64,
        )
    }

    fn floor(&self) -> Self {
        Self(self.0.floor(), self.1.floor())
    }
}

impl std::ops::Sub<CoordinatePoint> for CoordinatePoint {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::Add<CoordinatePoint> for CoordinatePoint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Add<f64> for CoordinatePoint {
    type Output = Self;

    fn add(self, rhs: f64) -> Self {
        Self(self.0 + rhs, self.1 + rhs)
    }
}

impl std::ops::Div<f64> for CoordinatePoint {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl std::ops::Mul<f64> for CoordinatePoint {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

pub const RAIL_HEIGHT: f32 = 0.2;
const RAIL_DISTANCE: f32 = 1.435;
const RAIL_WIDTH: f32 = 0.1;

pub struct LandscapePlugin;

impl Plugin for LandscapePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (init_height_map::system, open_street_map::load_data),
        )
        .add_systems(
            Update,
            (
                spawn_landscapes::system,
                spawn_landscape_mesh::system,
                despawn_landscapes::system,
                spawn_rails::system,
            ),
        );
    }
}
