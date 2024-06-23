#![feature(coverage_attribute)]

mod camera;
mod landscape;
mod train;
mod train_controls;
mod ui;

use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        render_resource::WgpuFeatures,
        settings::{RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};
use bevy_egui::EguiPlugin;
use train::{PaintScheme, PaintSchemeColor};

#[coverage(off)]
fn setup(mut commands: Commands) {
    commands
        .spawn(train::TrainBundle::br_218("BR 218 001", 400_000.0))
        .insert(PaintScheme {
            color: PaintSchemeColor::Orientrot,
        });

    commands
        .spawn(train::TrainBundle::br_218("BR 218 002", 0.0))
        .insert(PaintScheme {
            color: PaintSchemeColor::Pasteltuerkis,
        });
}

const HEIGHT_OFFSET: f32 = -101.0;
const TRAIN_HEIGHT: f32 = 5.0;
const TRAIN_HEIGHT_OFFSET: f32 = 1.5 + TRAIN_HEIGHT / 2.0;

#[coverage(off)]
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    // WARN this is a native only feature. It will not work with webgl or webgpu
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }),
                ..default()
            }),
            // You need to add this plugin to enable wireframe rendering
            WireframePlugin,
        ))
        .insert_resource(WireframeConfig {
            global: false,
            default_color: Color::hex("8FBCBB").unwrap(),
        })
        .add_plugins(EguiPlugin)
        .add_plugins(train::TrainPlugins)
        .add_plugins(train_controls::TrainControlsPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(ui::UIPlugin)
        .add_plugins(landscape::LandscapePlugin)
        .add_systems(Startup, setup)
        .run();
}
