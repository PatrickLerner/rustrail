use super::{AssetData, HeightMap, Landscape, OriginOffset, HALF_LANDSCAPE_SIZE, TRIANGLE_SIZE};
use bevy::{
    ecs::{system::SystemState, world::CommandQueue},
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
    tasks::{block_on, futures_lite::future, AsyncComputeTaskPool, Task},
};

#[derive(Component)]
pub struct SpawnedMesh;

#[derive(Component)]
pub struct ComputeMesh(Task<CommandQueue>);

#[coverage(off)]
pub fn system(
    mut transform_tasks: Query<&mut ComputeMesh>,
    landscapes: Query<(Entity, &Landscape), Without<SpawnedMesh>>,
    origin_offset: Res<OriginOffset>,
    height_map: Res<HeightMap>,
    mut commands: Commands,
) {
    for mut task in &mut transform_tasks {
        if let Some(mut commands_queue) = block_on(future::poll_once(&mut task.0)) {
            // append the returned command queue to have it execute later
            commands.append(&mut commands_queue);
        }
    }

    let thread_pool = AsyncComputeTaskPool::get();

    for (entity, landscape) in landscapes.iter() {
        // TODO
        let landscape = landscape.clone();
        let entity = entity.clone();
        let origin_offset = origin_offset.0.clone();
        let height_map = height_map.clone();

        let task = thread_pool.spawn(async move {
            let position = landscape.position - origin_offset;

            log::debug!(
                "Spawning landscape at {:?}",
                (position.0 as i32, position.1 as i32)
            );

            let mut verticies: Vec<Vec3> = Vec::new();
            let mut uv: Vec<Vec2> = Vec::new();
            let mut indicies: Vec<u32> = Vec::new();
            let mut normals = Vec::new();

            let mut vertices_y = 0;
            let mut vertices_x = 0;

            let grid_size = (HALF_LANDSCAPE_SIZE as f64 / TRIANGLE_SIZE as f64) as i32;

            for dy in -grid_size..=grid_size {
                vertices_y += 1;
                vertices_x = 0;
                for dx in -grid_size..=grid_size {
                    vertices_x += 1;
                    let sx = dx as f64 * TRIANGLE_SIZE as f64;
                    let sy = dy as f64 * TRIANGLE_SIZE as f64;

                    let h = height_map
                        .height_at_position(sx + landscape.position.0, sy + landscape.position.1);

                    verticies.push(Vec3::new(
                        sx as f32, h, // NOTE: - on z due to bevy's inane projection
                        -sy as f32,
                    ));
                    normals.push(Vec3::new(0.0, 0.0, 0.0));
                    uv.push(Vec2::new(
                        if dx % 2 == 0 { 0.0 } else { 1.0 },
                        if dy % 2 == 0 { 0.0 } else { 1.0 },
                    ));
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
                    indicies.push(y * w + x + 1);
                    indicies.push(y * w + x + 1 + w);

                    indicies.push(y * w + x);
                    indicies.push(y * w + x + 1 + w);
                    indicies.push(y * w + x + w);
                }
            }

            // calculate normals
            for i in (0..indicies.len()).step_by(3) {
                //vi v(i+1) v(i+2) are the three faces of a triangle
                let a = verticies[indicies[i] as usize];
                let b = verticies[indicies[i + 1] as usize];
                let c = verticies[indicies[i + 2] as usize];

                let ab = b - a;
                let ac = c - a;

                let ab_x_ac = ab.cross(ac);

                normals[indicies[i] as usize] += ab_x_ac;
                normals[indicies[i + 1] as usize] += ab_x_ac;
                normals[indicies[i + 2] as usize] += ab_x_ac;
            }

            for normal in normals.iter_mut() {
                *normal = normal.normalize();
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

            let mut command_queue = CommandQueue::default();
            command_queue.push(move |world: &mut World| {
                let (mesh, material) = {
                    let mut system_state =
                        SystemState::<(ResMut<Assets<Mesh>>, Res<AssetData>)>::new(world);
                    let (mut meshes, assets) = system_state.get_mut(world);

                    (meshes.add(mesh), assets.ground_texture.clone())
                };

                world
                    .entity_mut(entity)
                    .remove::<ComputeMesh>()
                    .with_children(
                        #[coverage(off)]
                        |parent| {
                            parent.spawn(PbrBundle {
                                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                                mesh,
                                material,
                                ..default()
                            });
                        },
                    );
            });

            command_queue
        });

        commands
            .entity(entity)
            .insert((SpawnedMesh, ComputeMesh(task)));
    }
}
