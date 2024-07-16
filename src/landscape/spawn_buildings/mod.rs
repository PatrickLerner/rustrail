use super::{AssetData, HeightMap, Landscape, OSMData};
use crate::{landscape::open_street_map::BuildingType, mesh::generate_3d_mesh};
use bevy::{
    ecs::{system::SystemState, world::CommandQueue},
    prelude::*,
    tasks::{block_on, futures_lite::future, AsyncComputeTaskPool, Task},
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

#[derive(Component)]
pub struct ComputeBuildings(Task<CommandQueue>);

#[coverage(off)]
pub fn system(
    mut tasks: Query<&mut ComputeBuildings>,
    mut commands: Commands,
    landscapes: Query<(Entity, &Landscape), Without<SpawnedBuildings>>,
    height_map: Res<HeightMap>,
    data: Res<OSMData>,
) {
    for mut task in &mut tasks {
        if let Some(mut commands_queue) = block_on(future::poll_once(&mut task.0)) {
            // append the returned command queue to have it execute later
            commands.append(&mut commands_queue);
        }
    }

    let thread_pool = AsyncComputeTaskPool::get();

    for (entity, landscape) in landscapes.iter() {
        let landscape = landscape.clone();
        let height_map = height_map.clone();

        let sector = landscape.position.sector_coordinates();
        log::debug!("spawning buildings for tile {:?}", sector);

        if let Some(section_data) = data.sections.get(&sector) {
            let buildings = section_data.buildings.clone();

            let task = thread_pool.spawn(
                #[coverage(off)]
                async move {
                    let mut count = 0;

                    let mut command_queue = CommandQueue::default();
                    for building in buildings.into_iter() {
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

                        let position_height = height_map.height_at_position(
                            coordinates.center.0 + landscape.position.0,
                            -coordinates.center.1 + landscape.position.1,
                        );

                        let transform = Transform::from_xyz(
                            coordinates.center.0 as f32,
                            offset + position_height,
                            coordinates.center.1 as f32,
                        );

                        command_queue.push(
                            #[coverage(off)]
                            move |world: &mut World| {
                                let (mesh, material) = {
                                    let mut system_state =
                                        SystemState::<(ResMut<Assets<Mesh>>, Res<AssetData>)>::new(
                                            world,
                                        );
                                    let (mut meshes, assets) = system_state.get_mut(world);
                                    let mesh = meshes
                                        .add(generate_3d_mesh(coordinates.list, extrude_amount));

                                    let material = match building.building_type {
                                        BuildingType::Building => assets.building_material.clone(),
                                        BuildingType::Industrial => {
                                            assets.industrial_material.clone()
                                        }
                                        BuildingType::Office => assets.office_material.clone(),
                                        BuildingType::Commercial => {
                                            assets.commercial_material.clone()
                                        }
                                        BuildingType::Roof => assets.building_material.clone(),
                                        BuildingType::Platform => assets.platform_material.clone(),
                                    };

                                    (mesh, material)
                                };

                                world.entity_mut(entity).with_children(
                                    #[coverage(off)]
                                    |parent| {
                                        parent.spawn(PbrBundle {
                                            mesh,
                                            material,
                                            transform,
                                            ..default()
                                        });
                                    },
                                );
                            },
                        );

                        count += 1;
                    }
                    log::debug!("{} buildings spawned", count);

                    command_queue.push(
                        #[coverage(off)]
                        move |world: &mut World| {
                            world.entity_mut(entity).remove::<ComputeBuildings>();
                        },
                    );

                    command_queue
                },
            );

            commands
                .entity(entity)
                .insert((SpawnedBuildings, ComputeBuildings(task)));
        } else {
            log::debug!("no building data for section found");
            commands.entity(entity).insert(SpawnedBuildings);
        }
    }
}
