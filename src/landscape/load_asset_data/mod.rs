use super::{BALLAST_HEIGHT, BALLAST_WIDTH, RAIL_HEIGHT, RAIL_WIDTH};
use bevy::prelude::*;

use super::AssetData;

#[coverage(off)]
pub fn system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(AssetData {
        rail_mesh: meshes.add(Cuboid::new(1.0, RAIL_HEIGHT, RAIL_WIDTH)),
        rail_material: materials.add(asset_server.load("steel.png")),
        ballast_mesh: meshes.add(Cuboid::new(1.0, BALLAST_HEIGHT, BALLAST_WIDTH)),
        ballast_texture: materials.add(asset_server.load("ballast.png")),
        ground_texture: materials.add(asset_server.load("soil.png")),
    });
}
