use crate::{
    landscape::{MAX_RAIL_SEGMENT_LENGTH, RAIL_DISTANCE},
    HEIGHT_OFFSET,
};

use super::{open_street_map::OSMData, HeightMap, Landscape, RAIL_HEIGHT, RAIL_WIDTH};
use bevy::prelude::*;

#[derive(Default)]
pub struct RailData {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

#[derive(Component)]
pub struct SpawnedRails;

#[coverage(off)]
pub fn system(
    mut rail_data: Local<Option<RailData>>,
    mut commands: Commands,
    data: Res<OSMData>,
    landscapes: Query<(Entity, &Landscape), Without<SpawnedRails>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    height_map: Res<HeightMap>,
) {
    for (entity, landscape) in landscapes.iter() {
        let rail_data = rail_data.get_or_insert_with(
            #[coverage(off)]
            || {
                // let material = materials.add(Color::rgb(0.180, 0.204, 0.251));
                RailData {
                    mesh: meshes.add(Cuboid::new(1.0, RAIL_HEIGHT, RAIL_WIDTH)),
                    material: materials.add(Color::rgb(1.0, 0.204, 0.251)),
                }
            },
        );

        let addr = landscape.position.sector_coordinates();

        if let Some(segment) = data.sections.get(&addr) {
            log::debug!("segment found {:?}", addr);

            for rail in segment.rails.iter() {
                let rail = data.rails.get(rail).unwrap();

                let rail_end_coords = rail.end_coords - landscape.position;
                let rail_start_coords = rail.start_coords - landscape.position;
                let diff = rail_end_coords - rail_start_coords;

                let length = rail.length();
                let angle = rail.angle();
                let direction = diff / length;

                let mut remaining_distance = length;
                let mut start_coords = rail.start_coords;

                while remaining_distance > 0.0 {
                    let distance =
                        remaining_distance.clamp(-MAX_RAIL_SEGMENT_LENGTH, MAX_RAIL_SEGMENT_LENGTH);
                    let end_coords = start_coords + direction * distance;

                    let rail_end_coords = end_coords - landscape.position;
                    let rail_start_coords = start_coords - landscape.position;
                    let pos = (rail_end_coords + rail_start_coords) / 2.0;

                    let start_height =
                        height_map.height_at_position(start_coords.0, start_coords.1);
                    let end_height = height_map.height_at_position(end_coords.0, end_coords.1);
                    let position_height = (start_height + end_height) / 2.0 + HEIGHT_OFFSET;

                    let lift_angle =
                        f64::atan2(end_height as f64 - start_height as f64, distance) as f32;

                    let rotation =
                        Quat::from_rotation_z(-lift_angle) * Quat::from_rotation_y(angle as f32);

                    let transform =
                        Transform::from_xyz(pos.0 as f32, position_height, -pos.1 as f32)
                            .with_scale(Vec3::new(distance as f32, 1.0, 1.0))
                            .with_rotation(rotation);

                    commands.entity(entity).insert(SpawnedRails).with_children(
                        #[coverage(off)]
                        |parent| {
                            parent
                                .spawn((
                                    rail.clone(),
                                    PbrBundle {
                                        transform,
                                        ..default()
                                    },
                                ))
                                .with_children(
                                    #[coverage(off)]
                                    |rail| {
                                        rail.spawn(PbrBundle {
                                            mesh: rail_data.mesh.clone(),
                                            material: rail_data.material.clone(),
                                            transform: Transform::from_xyz(
                                                0.0,
                                                0.0,
                                                RAIL_DISTANCE / -2.0,
                                            ),
                                            ..default()
                                        });

                                        rail.spawn(PbrBundle {
                                            mesh: rail_data.mesh.clone(),
                                            material: rail_data.material.clone(),
                                            transform: Transform::from_xyz(
                                                0.0,
                                                0.0,
                                                RAIL_DISTANCE / 2.0,
                                            ),
                                            ..default()
                                        });
                                    },
                                );
                        },
                    );

                    start_coords = end_coords;
                    remaining_distance -= distance;
                }
            }
        }
    }
}
