#[cfg(test)]
mod tests;

use super::{EguiUnlocked, GameCameraSettings, GameCameraState};
use crate::landscape::{HeightMap, OriginOffset};
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use std::f32::consts::{FRAC_PI_2, PI, TAU};

// some code modified from https://bevy-cheatbook.github.io/cookbook/pan-orbit-camera.html

pub fn system(
    In(egui_unlocked): In<EguiUnlocked>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut evr_motion: EventReader<MouseMotion>,
    mut evr_scroll: EventReader<MouseWheel>,
    mut q_camera: Query<(&GameCameraSettings, &mut GameCameraState, &mut Transform)>,
    height_map: Res<HeightMap>,
    origin_offset: Res<OriginOffset>,
) {
    // First, accumulate the total amount of
    // mouse motion and scroll, from all pending events:
    let mut total_motion: Vec2 = evr_motion.read().map(|ev| ev.delta).sum();

    // Reverse Y (Bevy's Worldspace coordinate system is Y-Up,
    // but events are in window/ui coordinates, which are Y-Down)
    total_motion.y = -total_motion.y;

    let mut total_scroll_lines = Vec2::ZERO;
    let mut total_scroll_pixels = Vec2::ZERO;
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                total_scroll_lines.x += ev.x;
                total_scroll_lines.y -= ev.y;
            }
            MouseScrollUnit::Pixel => {
                total_scroll_pixels.x += ev.x;
                total_scroll_pixels.y -= ev.y;
            }
        }
    }

    for (settings, mut state, mut transform) in &mut q_camera {
        // Check how much of each thing we need to apply.
        // Accumulate values from motion and scroll,
        // based on our configuration settings.

        let mut total_pan = Vec2::ZERO;
        let mut total_zoom = Vec2::ZERO;
        let mut total_orbit = Vec2::ZERO;

        if egui_unlocked.0 {
            if mouse.pressed(MouseButton::Left) {
                total_pan -= total_motion * settings.pan_sensitivity;
                state.follow = None;
            }

            if mouse.pressed(MouseButton::Right) {
                total_orbit -= total_motion * settings.orbit_sensitivity;
            }

            // Upon starting a new orbit maneuver (key is just pressed),
            // check if we are starting it upside-down
            if mouse.just_pressed(MouseButton::Right) {
                state.upside_down = state.pitch < -FRAC_PI_2 || state.pitch > FRAC_PI_2;
            }

            // If we are upside down, reverse the X orbiting
            if state.upside_down {
                total_orbit.x = -total_orbit.x;
            }
            total_orbit.y = -total_orbit.y;

            total_zoom -=
                total_scroll_lines * settings.scroll_line_sensitivity * settings.zoom_sensitivity;
            total_zoom -=
                total_scroll_pixels * settings.scroll_pixel_sensitivity * settings.zoom_sensitivity;
        }

        // Now we can actually do the things!

        let mut any = false;

        // To ZOOM, we need to multiply our radius.
        if total_zoom != Vec2::ZERO {
            any = true;
            // in order for zoom to feel intuitive,
            // everything needs to be exponential
            // (done via multiplication)
            // not linear
            // (done via addition)

            // so we compute the exponential of our
            // accumulated value and multiply by that
            state.radius *= (-total_zoom.y).exp();
        }

        // To ORBIT, we change our pitch and yaw values
        if total_orbit != Vec2::ZERO {
            any = true;
            state.yaw += total_orbit.x;
            state.pitch += total_orbit.y;
            // wrap around, to stay between +- 180 degrees
            if state.yaw > PI {
                state.yaw -= TAU; // 2 * PI
            }
            if state.yaw < -PI {
                state.yaw += TAU; // 2 * PI
            }
            if state.pitch > PI {
                state.pitch -= TAU; // 2 * PI
            }
            if state.pitch < -PI {
                state.pitch += TAU; // 2 * PI
            }
        }

        // To PAN, we can get the UP and RIGHT direction
        // vectors from the camera's transform, and use
        // them to move the center point. Multiply by the
        // radius to make the pan adapt to the current zoom.
        if total_pan != Vec2::ZERO || state.is_added() {
            any = true;
            let radius = state.radius;
            // state.center += transform.right() * total_pan.x * radius;
            // state.center += transform.up() * total_pan.y * radius;
            let vec = Vec3::new(total_pan.x, 0.0, -total_pan.y) * radius;
            let vec = Quat::from_rotation_y(state.yaw) * vec;

            state.center += vec;
            state.center.y = height_map.height_at_position(
                state.center.x as f64 + origin_offset.0 .0,
                state.center.y as f64 + origin_offset.0 .1,
            );
        }

        // Finally, compute the new camera transform.
        // (if we changed anything, or if the pan-orbit
        // controller was just added and thus we are running
        // for the first time and need to initialize)
        if any || state.is_added() || state.is_changed() {
            // YXZ Euler Rotation performs yaw/pitch/roll.
            transform.rotation = Quat::from_euler(EulerRot::YXZ, state.yaw, state.pitch, 0.0);
            // To position the camera, get the backward direction vector
            // and place the camera at the desired radius from the center.
            transform.translation = state.center + transform.back() * state.radius;
        }
    }
}
