#[cfg(test)]
mod tests;

use super::LANDSCAPE_SIZE;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize, Copy, Clone, PartialEq)]
pub struct CoordinatePoint(pub f64, pub f64);

impl From<CoordinatePoint> for Vec2 {
    fn from(val: CoordinatePoint) -> Self {
        Vec2::new(val.0 as f32, val.1 as f32)
    }
}

impl CoordinatePoint {
    pub fn sector_coordinates(&self) -> (i64, i64) {
        (
            (self.0 / LANDSCAPE_SIZE as f64 + 0.5).floor() as i64,
            (self.1 / LANDSCAPE_SIZE as f64 + 0.5).floor() as i64,
        )
    }

    pub fn floor(&self) -> Self {
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
