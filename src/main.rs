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
use train::{Mass, Name, PaintScheme, PaintSchemeColor, TrainComponent, TrainComposition};

#[coverage(off)]
fn setup(mut commands: Commands) {
    let engine_1 = commands
        .spawn(train::EngineBundle::br_218("BR 218 001"))
        .insert(PaintScheme {
            color: PaintSchemeColor::Orientrot,
        })
        .id();

    let engine_2 = commands
        .spawn(train::EngineBundle::br_218("BR 218 002"))
        .insert(PaintScheme {
            color: PaintSchemeColor::Pasteltuerkis,
        })
        .id();

    let wagon_1 = commands
        .spawn(train::EngineBundle::br_218("Wagon"))
        .insert(PaintScheme {
            color: PaintSchemeColor::Fernblau,
        })
        .id();

    commands.spawn(train::TrainBundle {
        name: Name("RB 61".to_string()),
        // TODO: speed poison
        mass: Mass(1.0),
        composition: TrainComposition {
            components: vec![
                TrainComponent::Engine(engine_1),
                TrainComponent::Engine(engine_2),
                TrainComponent::Engine(wagon_1),
            ],
        },
        ..default()
    });

    let engine_3 = commands
        .spawn(train::EngineBundle::br_218("BR 218 003"))
        .insert(PaintScheme {
            color: PaintSchemeColor::Lachsorange,
        })
        .id();

    let wagon_2 = commands
        .spawn(train::EngineBundle::br_218("Wagon"))
        .insert(PaintScheme {
            color: PaintSchemeColor::Fernblau,
        })
        .id();

    commands.spawn(train::TrainBundle {
        name: Name("RE 61".to_string()),
        // TODO: speed poison
        mass: Mass(1.0),
        composition: TrainComposition {
            components: vec![
                TrainComponent::Engine(engine_3),
                TrainComponent::Engine(wagon_2),
            ],
        },
        ..default()
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
