use super::{AssetData, HeightMap, Landscape, OSMData};
use crate::{landscape::open_street_map::BuildingType, mesh::generate_mesh, HEIGHT_OFFSET};
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

#[coverage(off)]
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

                let coordinates = building
                    .coordinates
                    .view_for_landscape_position(&landscape.position);

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

                let mesh = meshes.add(generate_mesh(coordinates.list, extrude_amount));

                let position_height = height_map.height_at_position(
                    coordinates.center.0 as f64 + landscape.position.0,
                    -coordinates.center.1 as f64 + landscape.position.1,
                ) + HEIGHT_OFFSET;

                let transform = Transform::from_xyz(
                    coordinates.center.0 as f32,
                    offset + position_height,
                    coordinates.center.1 as f32,
                );

                commands.entity(entity).with_children(
                    #[coverage(off)]
                    |parent| {
                        parent.spawn(PbrBundle {
                            mesh,
                            material: material.clone(),
                            transform,
                            ..default()
                        });
                    },
                );

                count += 1;
            }
            log::debug!("{} buildings spawned", count);
        } else {
            log::debug!("no building data for section found");
        }

        commands.entity(entity).insert(SpawnedBuildings);
    }
}
