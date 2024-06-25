#[cfg(test)]
mod tests;

mod despawn_landscapes;
mod init_height_map;
mod open_street_map;
mod spawn_landscape_mesh;
mod spawn_landscapes;

mod height_map;

use bevy::prelude::*;
pub use height_map::HeightMap;
use open_street_map::Data;
use serde::{Deserialize, Serialize};

const TRIANGLE_SIZE: i32 = 10;
const LANDSCAPE_SIZE: i32 = 1000;
const HALF_LANDSCAPE_SIZE: i32 = LANDSCAPE_SIZE / 2;
const BENSHEIM_STATION: (f64, f64) = (49.68134809269307, 8.61687829630227);
// lifetime of a landscape. if it is not renewed, it will despawn
const DEFAULT_TTL: f32 = 30.0;
const SPAWN_RADIUS: i32 = 5;
const MAX_SPAWN_PER_FRAME: usize = 3;

#[derive(Component)]
pub struct Landscape {
    pub ttl: f32,
    pub x: f64,
    pub y: f64,
    pub spawned_rails: bool,
}

impl Default for Landscape {
    fn default() -> Self {
        Self {
            ttl: DEFAULT_TTL,
            x: 0.0,
            y: 0.0,
            spawned_rails: false,
        }
    }
}

type PathId = (i64, i64);

// TODO: merge with CoordinatePoint

#[derive(Resource)]
pub struct OriginOffset {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug, Deserialize, Serialize, Copy, Clone, PartialEq)]
pub struct CoordinatePoint(f64, f64);

impl Into<Vec2> for CoordinatePoint {
    fn into(self) -> Vec2 {
        Vec2::new(self.0 as f32, self.1 as f32)
    }
}

impl CoordinatePoint {
    fn sector_coordinates(&self) -> (i64, i64) {
        (
            (self.0 / LANDSCAPE_SIZE as f64 + 0.5).floor() as i64,
            (self.1 / LANDSCAPE_SIZE as f64 + 0.5).floor() as i64,
        )
    }
}

pub const RAIL_HEIGHT: f32 = 0.2;
const RAIL_DISTANCE: f32 = 1.435;
const RAIL_WIDTH: f32 = 0.1;

fn spawn_rails(
    mut commands: Commands,
    data: Res<Data>,
    mut landscapes: Query<(Entity, &mut Landscape)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    height_map: Res<HeightMap>,
) {
    for (entity, mut landscape) in landscapes.iter_mut() {
        if landscape.spawned_rails {
            return;
        }
        landscape.spawned_rails = true;

        // TODO
        let mesh = meshes.add(Cuboid::new(1.0, RAIL_HEIGHT, RAIL_WIDTH));
        // let material = materials.add(Color::rgb(0.180, 0.204, 0.251));
        let material = materials.add(Color::rgb(1.0, 0.204, 0.251));

        //log::info!("{:?}", data.sections.keys().next());

        // TODO
        let addr = CoordinatePoint(landscape.x, landscape.y).sector_coordinates();

        if let Some(segment) = data.sections.get(&addr) {
            log::info!("segment found {:?}", addr);

            for rail in segment.rails.iter() {
                let rail = data.rails.get(&rail).unwrap();

                let rail_end_coords = Vec2::new(
                    (rail.end_coords.0 - landscape.x) as f32,
                    (rail.end_coords.1 - landscape.y) as f32,
                );
                let rail_start_coords = Vec2::new(
                    (rail.start_coords.0 - landscape.x) as f32,
                    (rail.start_coords.1 - landscape.y) as f32,
                );

                let pos = (rail_end_coords + rail_start_coords) / 2.0;
                let diff = rail_end_coords - rail_start_coords;

                let distance = f32::sqrt(f32::powi(diff.x, 2) + f32::powi(diff.y, 2));
                let angle = f32::atan2(diff.y, diff.x);

                let start_height =
                    height_map.height_at_position(rail.start_coords.0, rail.start_coords.1);
                let end_height =
                    height_map.height_at_position(rail.end_coords.0, rail.end_coords.1);
                let landscape_height = height_map.height_at_position(landscape.x, landscape.y);
                let position_height = (start_height + end_height) / 2.0 - landscape_height;

                let lift_angle = f32::atan2(end_height - start_height, distance);

                let axis = Vec3::Y;
                let rotation = Quat::from_rotation_z(-lift_angle);
                let axis = rotation * axis;

                let transform = Transform::from_xyz(pos.x, position_height, pos.y)
                    .with_scale(Vec3::new(distance, 1.0, 1.0))
                    .with_rotation(Quat::from_axis_angle(axis, -angle));

                commands.entity(entity).with_children(|parent| {
                    parent
                        .spawn((
                            rail.clone(),
                            PbrBundle {
                                transform,
                                ..default()
                            },
                        ))
                        .with_children(|rail| {
                            rail.spawn(PbrBundle {
                                mesh: mesh.clone(),
                                material: material.clone(),
                                transform: Transform::from_xyz(0.0, 0.0, RAIL_DISTANCE / -2.0),
                                ..default()
                            });

                            rail.spawn(PbrBundle {
                                mesh: mesh.clone(),
                                material: material.clone(),
                                transform: Transform::from_xyz(0.0, 0.0, RAIL_DISTANCE / 2.0),
                                ..default()
                            });
                        });
                });
            }
        }
    }
}

pub struct LandscapePlugin;

impl Plugin for LandscapePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (init_height_map::system, open_street_map::load_data),
        )
        .add_systems(
            Update,
            (
                spawn_landscapes::system,
                spawn_landscape_mesh::system,
                despawn_landscapes::system,
                spawn_rails,
            ),
        );
    }
}
