#[cfg(test)]
mod tests;

mod spawn_camera;
mod spawn_light;
mod update_camera;
mod update_follow;

use crate::{landscape::HeightMap, moving_things};
use bevy::prelude::*;
use bevy_egui::EguiContexts;

#[derive(Bundle, Default)]
pub struct GameCameraBundle {
    pub camera: Camera3dBundle,
    pub state: GameCameraState,
    pub settings: GameCameraSettings,
}

#[derive(Component)]
pub struct GameCameraState {
    pub center: Vec3,
    pub radius: f32,
    pub upside_down: bool,
    pub pitch: f32,
    pub yaw: f32,
    pub follow: Option<Entity>,
}

#[derive(Component)]
pub struct GameCameraSettings {
    /// World units per pixel of mouse motion
    pub pan_sensitivity: f32,
    /// Radians per pixel of mouse motion
    pub orbit_sensitivity: f32,
    /// Exponent per pixel of mouse motion
    pub zoom_sensitivity: f32,
    /// For devices with a notched scroll wheel, like desktop mice
    pub scroll_line_sensitivity: f32,
    /// For devices with smooth scrolling, like touchpads
    pub scroll_pixel_sensitivity: f32,
}

impl Default for GameCameraState {
    fn default() -> Self {
        Self {
            center: Vec3::ZERO,
            radius: 1.0,
            upside_down: false,
            pitch: 0.0,
            yaw: 0.0,
            follow: None,
        }
    }
}

impl Default for GameCameraSettings {
    fn default() -> Self {
        Self {
            pan_sensitivity: 0.001,                 // 1000 pixels per world unit
            orbit_sensitivity: 0.1f32.to_radians(), // 0.1 degree per pixel
            zoom_sensitivity: 0.01,
            scroll_line_sensitivity: 16.0, // 1 "line" == 16 "pixels of motion"
            scroll_pixel_sensitivity: 1.0,
        }
    }
}

struct EguiUnlocked(bool);

#[coverage(off)]
fn egui_unlocked(mut egui: EguiContexts) -> EguiUnlocked {
    let egui = egui.ctx_mut();
    EguiUnlocked(!egui.is_using_pointer())
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                egui_unlocked
                    .pipe(update_camera::system)
                    .run_if(resource_exists::<HeightMap>),
                update_follow::system.before(moving_things),
            )
                .run_if(any_with_component::<GameCameraState>),
        )
        .add_systems(Startup, (spawn_camera::system, spawn_light::system));
    }
}
