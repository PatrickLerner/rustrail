use super::{AssetData, HeightMap, Landscape, OSMData};
use crate::{
    earcutr::generate_mesh_earcutr, landscape::open_street_map::BuildingType, HEIGHT_OFFSET,
};
use bevy::prelude::*;

const PLATFORM_HEIGHT: f32 = 0.55;
const BUILDING_HEIGHT: f32 = 4.0;
const INDUSTRIAL_HEIGHT: f32 = 15.0;
const OFFICE_HEIGHT: f32 = 20.0;
const COMMERCIAL_HEIGHT: f32 = 12.0;
const ROOF_HEIGHT: f32 = 0.3;

const FIRST_LEVEL_HEIGHT: f32 = 7.0;
const LEVEL_HEIGHT: f32 = 5.0;

#[derive(Component)]
pub struct SpawnedBuildings;

pub fn system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    landscapes: Query<(Entity, &Landscape), Without<SpawnedBuildings>>,
    height_map: Res<HeightMap>,
    data: Res<OSMData>,
    assets: Res<AssetData>,
) {
    for (entity, landscape) in landscapes.iter() {
        let mut count = 0;

        let sector = landscape.position.sector_coordinates();
        log::debug!("spawning buildings for tile {:?}", sector);

        if let Some(section_data) = data.sections.get(&sector) {
            for building in section_data.buildings.iter() {
                // TODO: add more textures
                let material = match building.building_type {
                    BuildingType::Building => assets.building_material.clone(),
                    BuildingType::Industrial => assets.building_material.clone(),
                    BuildingType::Office => assets.building_material.clone(),
                    BuildingType::Commercial => assets.building_material.clone(),
                    BuildingType::Roof => assets.building_material.clone(),
                    BuildingType::Platform => assets.platform_material.clone(),
                };

                let coordinates: Vec<Vec2> = building
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

                let extrude_amount = if let Some(level) = building.levels {
                    if level == 0 {
                        FIRST_LEVEL_HEIGHT
                    } else {
                        level as f32 * LEVEL_HEIGHT
                    }
                } else {
                    match building.building_type {
                        BuildingType::Building => BUILDING_HEIGHT,
                        BuildingType::Industrial => INDUSTRIAL_HEIGHT,
                        BuildingType::Office => OFFICE_HEIGHT,
                        BuildingType::Commercial => COMMERCIAL_HEIGHT,
                        BuildingType::Platform => PLATFORM_HEIGHT,
                        BuildingType::Roof => ROOF_HEIGHT,
                    }
                };

                let offset = if let Some(layer) = building.layer {
                    if layer == 0 {
                        FIRST_LEVEL_HEIGHT
                    } else {
                        layer as f32 * LEVEL_HEIGHT
                    }
                } else {
                    0.0
                };

                let mesh = meshes.add(generate_mesh_earcutr(path_2d.clone(), extrude_amount));

                let position_height = height_map.height_at_position(
                    center.x as f64 + landscape.position.0,
                    -center.y as f64 + landscape.position.1,
                ) + HEIGHT_OFFSET;

                let transform = Transform::from_xyz(center.x, offset + position_height, center.y);

                commands.entity(entity).with_children(|parent| {
                    parent.spawn(PbrBundle {
                        mesh,
                        material: material.clone(),
                        transform,
                        ..default()
                    });
                });

                count += 1;
            }
            log::debug!("{} buildings spawned", count);
        } else {
            log::debug!("no building data for section found");
        }

        commands.entity(entity).insert(SpawnedBuildings);
    }
}
