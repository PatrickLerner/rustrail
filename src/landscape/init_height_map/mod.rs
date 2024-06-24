#[cfg(test)]
mod tests;

use bevy::prelude::*;
use proj::Proj;

use super::{HeightMap, OriginOffset, BENSHEIM_STATION};

pub fn system(mut commands: Commands) {
    let converter = Proj::new_known_crs("EPSG:4326", "ESRI:53004", None).unwrap();
    let (lat, lng) = BENSHEIM_STATION;
    let result = converter.convert((lng, lat));
    let (origin_x, origin_y) = result.unwrap();

    commands.insert_resource(OriginOffset {
        x: origin_x,
        y: origin_y,
    });

    let height_map = HeightMap::load_from_file("assets/dgm200_utm32s.tif");
    commands.insert_resource(height_map);
}
