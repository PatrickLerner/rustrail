#[cfg(test)]
mod tests;

mod height_map;

use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
};
pub use height_map::HeightMap;
use proj::Proj;

use crate::{camera::GameCameraState, HEIGHT_OFFSET};

const TRIANGLE_SIZE: i32 = 10;
const LANDSCAPE_SIZE: i32 = 1000;
const BENSHEIM_STATION: (f64, f64) = (49.68134809269307, 8.61687829630227);
// lifetime of a landscape. if it is not renewed, it will despawn
const DEFAULT_TTL: f32 = 30.0;
const SPAWN_RADIUS: i32 = 5;
const MAX_SPAWN_PER_FRAME: usize = 3;

#[derive(Resource)]
pub struct OriginOffset {
    pub x: f64,
    pub y: f64,
}

#[derive(Component)]
pub struct Landscape {
    pub ttl: f32,
    pub x: f64,
    pub y: f64,
}

fn init_height_map(mut commands: Commands) {
    let converter = Proj::new_known_crs("EPSG:4326", "ESRI:53004", None).unwrap();
    let (lat, lng) = BENSHEIM_STATION;
    let result = converter.convert((lng, lat));
    let (origin_x, origin_y) = result.unwrap();

    commands.insert_resource(OriginOffset {
        x: origin_x,
        y: origin_y,
    });

    let height_map = HeightMap::load_from_file("assets/dgm200_utm32s.tif");
    commands.insert_resource(height_map);
}

fn despawn_landscapes(
    mut commands: Commands,
    mut landscapes: Query<(Entity, &mut Landscape, &Transform)>,
    time: Res<Time>,
) {
    for (entity, mut landscape, transform) in landscapes.iter_mut() {
        landscape.ttl -= time.delta_seconds();
        if landscape.ttl <= 0.0 {
            commands.entity(entity).despawn_recursive();

            log::debug!(
                "Despawning landscape at {:?}",
                (
                    transform.translation.x as i32,
                    transform.translation.z as i32,
                )
            );
        }
    }
}

fn spawn_landscapes(
    mut commands: Commands,
    mut landscapes: Query<&mut Landscape>,
    cameras: Query<&GameCameraState>,
    origin_offset: Res<OriginOffset>,
) {
    let grid_half_length = (LANDSCAPE_SIZE / 2) as f64;

    for camera in cameras.iter() {
        for dx in -SPAWN_RADIUS..SPAWN_RADIUS {
            for dy in -SPAWN_RADIUS..SPAWN_RADIUS {
                let x = (((camera.center.x as f64 + grid_half_length) / (2.0 * grid_half_length))
                    .floor()
                    + dx as f64)
                    * (2.0 * grid_half_length);
                let y = (((camera.center.z as f64 + grid_half_length) / (2.0 * grid_half_length))
                    .floor()
                    + dy as f64)
                    * (2.0 * grid_half_length);

                let desired_x = x + origin_offset.x;
                let desired_y = y + origin_offset.y;

                if let Some(mut landscape) = landscapes
                    .iter_mut()
                    .find(|l| l.x == desired_x && l.y == desired_y)
                {
                    landscape.ttl = DEFAULT_TTL;
                } else {
                    log::debug!("Requesting landscape at {:?}", (x as i32, y as i32));

                    commands.spawn(Landscape {
                        ttl: DEFAULT_TTL,
                        x: desired_x,
                        y: desired_y,
                    });
                }
            }
        }
    }
}

fn spawn_height_maps(
    landscapes: Query<(Entity, &Landscape), Without<Transform>>,
    origin_offset: Res<OriginOffset>,
    height_map: Res<HeightMap>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, landscape) in landscapes.iter().take(MAX_SPAWN_PER_FRAME) {
        let position = Vec2::new(
            (landscape.x - origin_offset.x) as f32,
            (landscape.y - origin_offset.y) as f32,
        );

        log::debug!(
            "Spawning landscape at {:?}",
            (position.x as i32, position.y as i32)
        );

        let mut verticies: Vec<Vec3> = Vec::new();
        let mut uv: Vec<Vec2> = Vec::new();
        let mut indicies: Vec<u32> = Vec::new();
        let mut normals = Vec::new();

        let mut vertices_y = 0;
        let mut vertices_x = 0;

        let grid_size = ((LANDSCAPE_SIZE / 2) as f64 / TRIANGLE_SIZE as f64) as i32;

        for dy in -grid_size..=grid_size {
            vertices_y += 1;
            vertices_x = 0;
            for dx in -grid_size..=grid_size {
                vertices_x += 1;
                let sx = dx as f64 * TRIANGLE_SIZE as f64;
                let sy = dy as f64 * TRIANGLE_SIZE as f64;

                let h = height_map.height_at_position(sx + landscape.x, sy + landscape.y);

                verticies.push(Vec3::new(sx as f32, h + HEIGHT_OFFSET, sy as f32));
                normals.push(Vec3::new(0.0, 1.0, 0.0));
                uv.push(Vec2::new(0.0, 0.0));
            }
        }

        let w = 1 + 2 * grid_size as u32;
        let h = 1 + 2 * grid_size as u32;
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

        commands.entity(entity).insert(PbrBundle {
            mesh,
            material: materials.add(Color::hex("A3BE8C").unwrap()),
            transform: Transform::from_xyz(position.x, 0.0, position.y),
            ..default()
        });
    }
}

pub struct LandscapePlugin;

impl Plugin for LandscapePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_height_map).add_systems(
            Update,
            (spawn_landscapes, spawn_height_maps, despawn_landscapes),
        );
    }
}
