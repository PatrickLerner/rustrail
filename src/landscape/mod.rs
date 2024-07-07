#[cfg(test)]
mod tests;

mod coordinate_point;
mod despawn_landscapes;
mod height_map;
mod init_height_map;
mod load_asset_data;
mod open_street_map;
mod spawn_areas;
mod spawn_buildings;
mod spawn_landscape_mesh;
mod spawn_landscapes;
mod spawn_rails;

use bevy::prelude::*;
pub use coordinate_point::CoordinatePoint;
pub use height_map::HeightMap;
#[cfg(test)]
pub use open_street_map::Path;
pub use open_street_map::{OSMData, PathId};

const TRIANGLE_SIZE: i32 = 10;
const LANDSCAPE_SIZE: i32 = 1000;
const HALF_LANDSCAPE_SIZE: i32 = LANDSCAPE_SIZE / 2;
const ORIGIN: (f64, f64) = (49.68134809269307, 8.61687829630227);
// lifetime of a landscape. if it is not renewed, it will despawn
const DEFAULT_TTL: f32 = 30.0;
const SPAWN_RADIUS: i32 = 5;
const MAX_SPAWN_PER_FRAME: usize = 3;

pub const BALLAST_WIDTH: f32 = RAIL_DISTANCE + 1.75;
pub const BALLAST_HEIGHT: f32 = 0.4;
const MAX_RAIL_SEGMENT_LENGTH: f64 = 3.0;

pub const RAIL_HEIGHT: f32 = 0.2;
const RAIL_DISTANCE: f32 = 1.435;
pub const RAIL_WIDTH: f32 = 0.1;

#[derive(Resource, Default)]
pub struct AssetData {
    rail_mesh: Handle<Mesh>,
    rail_material: Handle<StandardMaterial>,
    ballast_mesh: Handle<Mesh>,
    ballast_texture: Handle<StandardMaterial>,
    ground_texture: Handle<StandardMaterial>,
    platform_material: Handle<StandardMaterial>,
    building_material: Handle<StandardMaterial>,
    office_material: Handle<StandardMaterial>,
    industrial_material: Handle<StandardMaterial>,
    commercial_material: Handle<StandardMaterial>,
}

#[derive(Component)]
pub struct Landscape {
    pub ttl: f32,
    pub position: CoordinatePoint,
}

impl Default for Landscape {
    fn default() -> Self {
        Self {
            ttl: DEFAULT_TTL,
            position: CoordinatePoint::default(),
        }
    }
}

#[derive(Resource, Default)]
pub struct OriginOffset(pub CoordinatePoint);

pub struct LandscapePlugin;

impl Plugin for LandscapePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                init_height_map::system,
                open_street_map::load_data,
                load_asset_data::system,
            ),
        )
        .add_systems(
            Update,
            (
                spawn_landscapes::system,
                spawn_landscape_mesh::system,
                despawn_landscapes::system,
                spawn_rails::system,
                spawn_buildings::system,
                spawn_areas::system,
            ),
        );
    }
}
