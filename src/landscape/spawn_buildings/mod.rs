use super::{HeightMap, Landscape, OSMData, OriginOffset};
use crate::{
    earcutr::generate_mesh_earcutr,
    landscape::{open_street_map::BuildingType, LANDSCAPE_SIZE},
    HEIGHT_OFFSET,
};
use bevy::{
    prelude::*,
    render::texture::{
        ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor,
    },
};

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
    mut materials: ResMut<Assets<StandardMaterial>>,
    landscapes: Query<(Entity, &Landscape), Without<SpawnedBuildings>>,
    height_map: Res<HeightMap>,
    asset_server: Res<AssetServer>,
    data: Res<OSMData>,
    origin_offset: Res<OriginOffset>,
) {
    for (entity, landscape) in landscapes.iter() {
        let mut count = 0;

        let sector = landscape.position.sector_coordinates();
        log::debug!("spawning buildings for tile {:?}", sector);

        // let building_material = materials.add(Color::rgb(0.693, 0.740, 0.827));

        let building = asset_server.load_with_settings(
            "facade.png",
            |s: &mut ImageLoaderSettings| match &mut s.sampler {
                ImageSampler::Default => {
                    s.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        ..default()
                    });
                }
                ImageSampler::Descriptor(sampler) => {
                    sampler.address_mode_u = ImageAddressMode::Repeat;
                    sampler.address_mode_v = ImageAddressMode::Repeat;
                }
            },
        );
        let building_material = materials.add(building);

        let platform_material = materials.add(Color::rgb(0.847, 0.871, 0.914));

        if let Some(section_data) = data.sections.get(&sector) {
            for building in section_data.buildings.iter() {
                let material = match building.building_type {
                    BuildingType::Building => building_material.clone(),
                    BuildingType::Industrial => building_material.clone(),
                    BuildingType::Office => building_material.clone(),
                    BuildingType::Commercial => building_material.clone(),
                    BuildingType::Roof => building_material.clone(),
                    BuildingType::Platform => platform_material.clone(),
                };

                let coordinates: Vec<Vec2> = building
                    .coordinates
                    .iter()
                    .map(|coordinate| {
                        // TODO: use coords
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

                let mesh = generate_mesh_earcutr(path_2d.clone(), extrude_amount);

                let mesh = meshes.add(mesh);

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
