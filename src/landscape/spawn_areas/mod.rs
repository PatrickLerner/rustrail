use super::{HeightMap, Landscape, OSMData};
use crate::{earcutr::generate_mesh_earcutr, landscape::open_street_map::AreaType, HEIGHT_OFFSET};
use bevy::prelude::*;
use fast_poisson::Poisson2D;
use geo::{point, Contains, LineString, Polygon};

const TREE_DENSITY: f64 = 10.0;
const TREE_HEIGHT: f32 = 25.0;
const TREE_CROWN_HEIGHT: f32 = 17.0;
const TREE_CROWN_WIDTH: f32 = 9.0;
const TREE_TRUNK_WIDTH: f32 = 2.0;

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
                let coordinates: Vec<Vec2> = area
                    .coordinates
                    .iter()
                    .map(|coordinate| {
                        let coordinates: Vec2 = (*coordinate - landscape.position).into();
                        coordinates * Vec2::new(1.0, -1.0)
                    })
                    .collect();

                // normalize so that first element is zero
                let x: Vec<f32> = coordinates.iter().map(|e| e.x).collect();
                let y: Vec<f32> = coordinates.iter().map(|e| e.y).collect();

                let max_x = *x.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
                let max_y = *y.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
                let min_x = *x.iter().min_by(|a, b| a.total_cmp(b)).unwrap();
                let min_y = *y.iter().min_by(|a, b| a.total_cmp(b)).unwrap();

                let center = Vec2::new(max_x + min_x, max_y + min_y) / 2.0;
                let path_2d: Vec<Vec2> = coordinates.iter().map(|item| *item - center).collect();

                match area.area_type {
                    AreaType::Wood => {
                        let path_2d: Vec<(f32, f32)> =
                            path_2d.iter().map(|item| (item.x, item.y)).collect();
                        let polygon = Polygon::new(LineString::from(path_2d), vec![]);

                        let points = Poisson2D::new().with_dimensions(
                            [(max_x - min_x) as f64, (max_y - min_y) as f64],
                            TREE_DENSITY * 2.0,
                        );

                        let mut count = 0;

                        for [x, y] in points.iter() {
                            if polygon.contains(&point!(x: x as f32, y: y as f32)) {
                                count += 1;

                                let position_height = height_map.height_at_position(
                                    center.x as f64 + landscape.position.0,
                                    -center.y as f64 + landscape.position.1,
                                ) + HEIGHT_OFFSET;

                                let transform = Transform::from_xyz(
                                    center.x + x as f32,
                                    position_height,
                                    center.y + y as f32,
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

                        // TODO: sometiems spawns 0
                        log::debug!("{} trees spawned", count);
                    }
                    AreaType::Water => {
                        let extrude_amount = 0.1;
                        let mesh = generate_mesh_earcutr(path_2d.clone(), extrude_amount);

                        let mesh = meshes.add(mesh);

                        let position_height = height_map.height_at_position(
                            center.x as f64 + landscape.position.0,
                            -center.y as f64 + landscape.position.1,
                        ) + HEIGHT_OFFSET;

                        let transform = Transform::from_xyz(center.x, position_height, center.y);

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
