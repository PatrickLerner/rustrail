#[cfg(test)]
mod tests;

use bevy::prelude::*;
use proj::Proj;

use super::{CoordinatePoint, HeightMap, OriginOffset, ORIGIN};

pub fn system(mut commands: Commands) {
    let converter = Proj::new_known_crs("EPSG:4326", "ESRI:53004", None).unwrap();
    let (lat, lng) = ORIGIN;
    let result = converter.convert((lng, lat));
    let (origin_x, origin_y) = result.unwrap();

    commands.insert_resource(OriginOffset(CoordinatePoint(origin_x, origin_y)));

    let height_map = HeightMap::load_from_file("assets/dgm200_utm32s.tif");
    commands.insert_resource(height_map);
}
