#![feature(coverage_attribute)]

mod camera;
mod landscape;
mod mesh;
mod scenario;
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
use landscape::{BALLAST_HEIGHT, RAIL_HEIGHT};
use scenario::ScenarioData;

const TRAIN_HEIGHT_OFFSET: f32 = BALLAST_HEIGHT + RAIL_HEIGHT;

// marker methods for system ordering
#[coverage(off)]
fn moving_things() {}

fn load_scenario(mut commands: Commands) {
    let scenario_data = ScenarioData::load_from_file("assets/scenarios/rb35.toml");
    commands.insert_resource(scenario_data);
}

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
            default_color: Srgba::hex("8FBCBB").unwrap().into(),
        })
        .add_plugins(EguiPlugin)
        .add_plugins(train::TrainPlugins)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(ui::UIPlugins)
        .add_plugins(landscape::LandscapePlugin)
        .add_systems(Update, moving_things)
        .add_systems(
            Update,
            load_scenario.run_if(not(resource_exists::<ScenarioData>)),
        )
        .run();
}
