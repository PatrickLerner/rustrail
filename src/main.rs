#![feature(coverage_attribute)]

mod camera;
mod height_map;
mod train;
mod train_controls;
mod ui;

use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
        render_resource::WgpuFeatures,
        settings::{RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};
use bevy_egui::EguiPlugin;
use height_map::HeightMap;
use proj::Proj;
use train::{PaintScheme, PaintSchemeColor};

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

#[derive(Resource)]
struct OriginOffset {
    x: f64,
    y: f64,
}

const BENSHEIM_STATION: (f64, f64) = (49.68134809269307, 8.61687829630227);
const HEIGHT_OFFSET: f32 = -101.0;
const TRAIN_HEIGHT: f32 = 5.0;
const TRAIN_HEIGHT_OFFSET: f32 = 1.5 + TRAIN_HEIGHT / 2.0;
const TRIANGLE_SIZE: i64 = 10;
const GRID_SIZE: i64 = 2500 / TRIANGLE_SIZE;

fn spawn_height_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let height_map = HeightMap::load_from_file("assets/dgm200_utm32s.tif");
    let converter = Proj::new_known_crs("EPSG:4326", "ESRI:53004", None).unwrap();
    let (lat, lng) = BENSHEIM_STATION;
    let result = converter.convert((lng, lat));
    let (origin_x, origin_y) = result.unwrap();

    commands.insert_resource(OriginOffset {
        x: origin_x,
        y: origin_y,
    });

    let mut verticies: Vec<Vec3> = Vec::new();
    let mut uv: Vec<Vec2> = Vec::new();
    let mut indicies: Vec<u32> = Vec::new();
    let mut normals = Vec::new();

    let mut vertices_y = 0;
    let mut vertices_x = 0;
    for dy in -GRID_SIZE..=GRID_SIZE {
        vertices_y += 1;
        vertices_x = 0;
        for dx in -GRID_SIZE..=GRID_SIZE {
            vertices_x += 1;
            let sx = dx as f64 * TRIANGLE_SIZE as f64;
            let sy = dy as f64 * TRIANGLE_SIZE as f64;

            let h = height_map.height_at_position(sx + origin_x, sy + origin_y);

            verticies.push(Vec3::new(sx as f32, h + HEIGHT_OFFSET, sy as f32));
            normals.push(Vec3::new(0.0, 1.0, 0.0));
            uv.push(Vec2::new(0.0, 0.0));
        }
    }

    let w = 1 + 2 * GRID_SIZE as u32;
    let h = 1 + 2 * GRID_SIZE as u32;
    let mut indices_y = 0;
    let mut indices_x = 0;
    for y in 0..h - 1 {
        indices_y += 1;
        indices_x = 0;
        for x in 0..w - 1 {
            indices_x += 1;

            indicies.push(y * w + x);
            indicies.push(y * w + x + 1 + w);
            indicies.push(y * w + x + 1);

            indicies.push(y * w + x);
            indicies.push(y * w + x + w);
            indicies.push(y * w + x + 1 + w);
        }
    }
    assert!(indices_y == vertices_y - 1);
    assert!(indices_x == vertices_x - 1);

    let indices = Indices::U32(indicies);
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
    mesh.insert_indices(indices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verticies);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv);
    mesh.generate_tangents().unwrap();

    let mesh = meshes.add(mesh);

    commands.spawn(PbrBundle {
        mesh,
        material: materials.add(Color::hex("A3BE8C").unwrap()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    commands.insert_resource(height_map);
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
            default_color: Color::hex("8FBCBB").unwrap(),
        })
        .add_plugins(EguiPlugin)
        .add_plugins(train::TrainPlugins)
        .add_plugins(train_controls::TrainControlsPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(ui::UIPlugin)
        .add_systems(Startup, (setup, spawn_height_map))
        .run();
}
