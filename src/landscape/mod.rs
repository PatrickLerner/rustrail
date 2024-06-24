#[cfg(test)]
mod tests;

mod despawn_landscapes;
mod init_height_map;
mod spawn_landscape_mesh;
mod spawn_landscapes;

mod height_map;

use bevy::prelude::*;
pub use height_map::HeightMap;

const TRIANGLE_SIZE: i32 = 10;
const LANDSCAPE_SIZE: i32 = 1000;
const BENSHEIM_STATION: (f64, f64) = (49.68134809269307, 8.61687829630227);
// lifetime of a landscape. if it is not renewed, it will despawn
const DEFAULT_TTL: f32 = 30.0;
const SPAWN_RADIUS: i32 = 5;
const MAX_SPAWN_PER_FRAME: usize = 3;

#[derive(Resource)]
pub struct OriginOffset {
    pub x: f64,
    pub y: f64,
}

#[derive(Component)]
pub struct Landscape {
    pub ttl: f32,
    pub x: f64,
    pub y: f64,
}

pub struct LandscapePlugin;

impl Plugin for LandscapePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_height_map::system)
            .add_systems(
                Update,
                (
                    spawn_landscapes::system,
                    spawn_landscape_mesh::system,
                    despawn_landscapes::system,
                ),
            );
    }
}
