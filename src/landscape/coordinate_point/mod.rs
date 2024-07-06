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

    pub fn length(&self) -> f64 {
        f64::sqrt(f64::powi(self.0, 2) + f64::powi(self.1, 2))
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

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Coordinates(pub Vec<CoordinatePoint>);

impl Coordinates {
    pub fn view_for_landscape_position(
        &self,
        landscape_position: &CoordinatePoint,
    ) -> CoordinateView {
        CoordinateView::for_landscape_position(self, landscape_position)
    }
}

/// The CoordinateView is constructed to access a list of coordinates
/// relative to a central point (landscape_position). All coordinates
/// will be transformed to be relative to this point and all y will be
/// inverted to align with Bevy's inane coordinate system and make
/// spawning things easy.
#[derive(Default, Debug, Deserialize, Serialize)]
pub struct CoordinateView {
    pub list: Vec<Vec2>,
    pub center: Vec2,
    pub max_x: f32,
    pub max_y: f32,
    pub min_x: f32,
    pub min_y: f32,
}

impl CoordinateView {
    pub fn for_landscape_position(
        coordinates: &Coordinates,
        landscape_position: &CoordinatePoint,
    ) -> Self {
        let list: Vec<Vec2> = coordinates
            .0
            .iter()
            .map(|coordinate| {
                let coordinates: Vec2 = (*coordinate - *landscape_position).into();
                coordinates * Vec2::new(1.0, -1.0)
            })
            .collect();

        let x: Vec<f32> = list.iter().map(|e| e.x).collect();
        let y: Vec<f32> = list.iter().map(|e| e.y).collect();

        let max_x = *x.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
        let max_y = *y.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
        let min_x = *x.iter().min_by(|a, b| a.total_cmp(b)).unwrap();
        let min_y = *y.iter().min_by(|a, b| a.total_cmp(b)).unwrap();

        let center = Vec2::new(max_x + min_x, max_y + min_y) / 2.0;
        let list: Vec<Vec2> = list.iter().map(|item| *item - center).collect();

        Self {
            min_x,
            max_x,
            min_y,
            max_y,
            list,
            center,
        }
    }
}
