// modied from https://bevy-cheatbook.github.io/cookbook/pan-orbit-camera.html

use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use std::f32::consts::{FRAC_PI_2, PI, TAU};

// Bundle to spawn our custom camera easily
#[derive(Bundle, Default)]
pub struct PanOrbitCameraBundle {
    pub camera: Camera3dBundle,
    pub state: PanOrbitState,
    pub settings: PanOrbitSettings,
}

// The internal state of the pan-orbit controller
#[derive(Component)]
pub struct PanOrbitState {
    pub center: Vec3,
    pub radius: f32,
    pub upside_down: bool,
    pub pitch: f32,
    pub yaw: f32,
    pub follow: Option<Entity>,
}

/// The configuration of the pan-orbit controller
#[derive(Component)]
pub struct PanOrbitSettings {
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

impl Default for PanOrbitState {
    fn default() -> Self {
        PanOrbitState {
            center: Vec3::ZERO,
            radius: 1.0,
            upside_down: false,
            pitch: 0.0,
            yaw: 0.0,
            follow: None,
        }
    }
}

impl Default for PanOrbitSettings {
    fn default() -> Self {
        PanOrbitSettings {
            pan_sensitivity: 0.001,                 // 1000 pixels per world unit
            orbit_sensitivity: 0.1f32.to_radians(), // 0.1 degree per pixel
            zoom_sensitivity: 0.01,
            scroll_line_sensitivity: 16.0, // 1 "line" == 16 "pixels of motion"
            scroll_pixel_sensitivity: 1.0,
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = PanOrbitCameraBundle::default();
    // Position our camera using our component,
    // not Transform (it would get overwritten)
    camera.state.center = Vec3::new(1.0, 10.0, 3.0);
    camera.state.radius = 50.0;
    camera.state.pitch = -30.0f32.to_radians();
    camera.state.yaw = 30.0f32.to_radians();
    commands.spawn(camera);
}

fn update_follow(mut q_camera: Query<&mut PanOrbitState>, transform: Query<&Transform>) {
    for mut state in &mut q_camera {
        if let Some(follow) = state.follow {
            let transform = transform.get(follow).unwrap();
            state.center = transform.translation;
        }
    }
}

fn pan_orbit_camera(
    mouse: Res<ButtonInput<MouseButton>>,
    mut evr_motion: EventReader<MouseMotion>,
    mut evr_scroll: EventReader<MouseWheel>,
    mut q_camera: Query<(&PanOrbitSettings, &mut PanOrbitState, &mut Transform)>,
    mut egui: EguiContexts,
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

        let egui = egui.ctx_mut();
        if !egui.is_using_pointer() {
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
        if total_pan != Vec2::ZERO {
            any = true;
            let radius = state.radius;
            // state.center += transform.right() * total_pan.x * radius;
            // state.center += transform.up() * total_pan.y * radius;
            let vec = Vec3::new(total_pan.x, 0.0, -total_pan.y) * radius;
            let vec = Quat::from_rotation_y(state.yaw) * vec;

            state.center += vec;
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

fn spawn_light(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 255.0,
    });
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (pan_orbit_camera, update_follow).run_if(any_with_component::<PanOrbitState>),
        )
        .add_systems(Startup, (spawn_camera, spawn_light));
    }
}
