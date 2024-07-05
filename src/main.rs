#![feature(coverage_attribute)]

mod camera;
mod earcutr;
mod landscape;
mod mesh;
mod train;
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
        .add_plugins(camera::CameraPlugin)
        .add_plugins(ui::UIPlugins)
        .add_plugins(landscape::LandscapePlugin)
        .run();
}
