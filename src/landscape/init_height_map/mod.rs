#[cfg(test)]
mod tests;

use super::{CoordinatePoint, HeightMap, OriginOffset};
use crate::scenario::ScenarioData;
use bevy::prelude::*;
use proj::Proj;

pub fn system(mut commands: Commands, scenario: Res<ScenarioData>) {
    let converter = Proj::new_known_crs("EPSG:4326", "ESRI:53004", None).unwrap();
    let result = converter.convert((scenario.origin.longitude, scenario.origin.latitude));
    let (origin_x, origin_y) = result.unwrap();

    commands.insert_resource(OriginOffset(CoordinatePoint(origin_x, origin_y)));

    let height_map = HeightMap::load_from_file(&scenario.map.height_map);
    commands.insert_resource(height_map);
}
