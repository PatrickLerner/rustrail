use super::{AssetData, BALLAST_HEIGHT, BALLAST_WIDTH, RAIL_HEIGHT, RAIL_WIDTH};
use bevy::{
    prelude::*,
    render::texture::{
        ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor,
    },
};

#[coverage(off)]
fn load_repeating_asset(asset_server: &AssetServer, file_name: &str) -> Handle<Image> {
    asset_server.load_with_settings(
        file_name.to_owned(),
        #[coverage(off)]
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
    )
}

#[coverage(off)]
pub fn system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(AssetData {
        rail_mesh: meshes.add(Cuboid::new(1.0, RAIL_HEIGHT, RAIL_WIDTH)),
        rail_material: materials.add(asset_server.load("textures/steel.png")),
        ballast_mesh: meshes.add(Cuboid::new(1.0, BALLAST_HEIGHT, BALLAST_WIDTH)),
        ballast_texture: materials.add(asset_server.load("textures/ballast.png")),
        ground_texture: materials.add(asset_server.load("textures/soil.png")),
        // TODO: material
        platform_material: materials.add(Color::srgb(0.847, 0.871, 0.914)),
        building_material: materials
            .add(load_repeating_asset(&asset_server, "textures/building.png")),
        office_material: materials.add(load_repeating_asset(&asset_server, "textures/office.png")),
        industrial_material: materials.add(load_repeating_asset(
            &asset_server,
            "textures/industrial.png",
        )),
        commercial_material: materials.add(load_repeating_asset(
            &asset_server,
            "textures/commercial.png",
        )),
    });
}
