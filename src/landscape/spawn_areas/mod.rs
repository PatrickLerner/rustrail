use super::{HeightMap, Landscape, OSMData};
use crate::{earcutr::generate_mesh_earcutr, landscape::open_street_map::AreaType, HEIGHT_OFFSET};
use bevy::prelude::*;
use fast_poisson::Poisson2D;
use geo::{point, Contains, LineString, Polygon};

const TREE_DENSITY: f64 = 10.0;
const TREE_HEIGHT: f32 = 16.0;
const TREE_CROWN_HEIGHT: f32 = 11.0;
const TREE_CROWN_WIDTH: f32 = 7.0;
const TREE_TRUNK_WIDTH: f32 = 1.5;

#[derive(Component)]
pub struct SpawnedAreas;

pub fn system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    landscapes: Query<(Entity, &Landscape), Without<SpawnedAreas>>,
    data: Res<OSMData>,
    height_map: Res<HeightMap>,
) {
    for (entity, landscape) in landscapes.iter() {
        let trunk_mesh = meshes.add(Cuboid::new(
            TREE_TRUNK_WIDTH,
            TREE_HEIGHT - TREE_CROWN_HEIGHT,
            TREE_TRUNK_WIDTH,
        ));
        let trunk_material = materials.add(Color::rgb(0.711, 0.444, 0.332));
        let crown_mesh = meshes.add(Cuboid::new(
            TREE_CROWN_WIDTH,
            TREE_CROWN_HEIGHT,
            TREE_CROWN_WIDTH,
        ));
        let crown_material = materials.add(Color::rgb(0.381, 0.558, 0.230));

        let mut count = 0;

        let sector = landscape.position.sector_coordinates();
        log::debug!("spawning areas for tile {:?}", sector);

        if let Some(section_data) = data.sections.get(&sector) {
            for area in section_data.areas.iter() {
                let coordinates = area
                    .coordinates
                    .view_for_landscape_position(&landscape.position);

                match area.area_type {
                    AreaType::Wood => {
                        let path_2d: Vec<(f32, f32)> = coordinates
                            .list
                            .iter()
                            .map(|item| (item.x, item.y))
                            .collect();
                        let polygon = Polygon::new(LineString::from(path_2d), vec![]);

                        let points = Poisson2D::new().with_dimensions(
                            [
                                (coordinates.max_x - coordinates.min_x) as f64,
                                (coordinates.max_y - coordinates.min_y) as f64,
                            ],
                            TREE_DENSITY * 2.0,
                        );

                        let mut points: Vec<(f64, f64)> =
                            points.iter().map(|point| (point[0], point[1])).collect();

                        if points.is_empty() {
                            // if nothing is spawned, we spawn at least a single lone tree in the
                            // center
                            points.push((0.0, 0.0));
                        }

                        log::debug!("{} trees spawned", points.len());

                        for (x, y) in points {
                            if polygon.contains(&point!(x: x as f32, y: y as f32)) {
                                let position_height = height_map.height_at_position(
                                    coordinates.center.x as f64 + x + landscape.position.0,
                                    -coordinates.center.y as f64 - y + landscape.position.1,
                                ) + HEIGHT_OFFSET;

                                let transform = Transform::from_xyz(
                                    coordinates.center.x + x as f32,
                                    position_height,
                                    coordinates.center.y + y as f32,
                                );

                                commands.entity(entity).with_children(|parent| {
                                    parent
                                        .spawn(PbrBundle {
                                            transform,
                                            ..default()
                                        })
                                        .with_children(|tree| {
                                            tree.spawn(PbrBundle {
                                                mesh: trunk_mesh.clone(),
                                                material: trunk_material.clone(),
                                                transform: Transform::from_xyz(
                                                    0.0,
                                                    (TREE_HEIGHT - TREE_CROWN_HEIGHT) / 2.0,
                                                    0.0,
                                                ),
                                                ..default()
                                            });

                                            tree.spawn(PbrBundle {
                                                mesh: crown_mesh.clone(),
                                                material: crown_material.clone(),
                                                transform: Transform::from_xyz(
                                                    0.0,
                                                    TREE_HEIGHT - 0.5 * TREE_CROWN_HEIGHT,
                                                    0.0,
                                                ),
                                                ..default()
                                            });
                                        });
                                });
                            }
                        }
                    }
                    AreaType::Water => {
                        let extrude_amount = 0.1;
                        let mesh = generate_mesh_earcutr(coordinates.list.clone(), extrude_amount);

                        let mesh = meshes.add(mesh);

                        let position_height = height_map.height_at_position(
                            coordinates.center.x as f64 + landscape.position.0,
                            -coordinates.center.y as f64 + landscape.position.1,
                        ) + HEIGHT_OFFSET;

                        let transform = Transform::from_xyz(
                            coordinates.center.x,
                            position_height,
                            coordinates.center.y,
                        );

                        let material = materials.add(Color::rgb(0.369, 0.506, 0.675));

                        commands.entity(entity).with_children(|parent| {
                            parent.spawn(PbrBundle {
                                mesh,
                                material: material.clone(),
                                transform,
                                ..default()
                            });
                        });
                    }
                }

                count += 1;
            }
            log::debug!("{} areas spawned", count);
        } else {
            log::debug!("no area data for section found");
        }

        commands.entity(entity).insert(SpawnedAreas);
    }
}