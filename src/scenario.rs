use crate::train::Direction;
use bevy::prelude::*;
use serde::Deserialize;

// TBD: disallow additional attributes?

#[derive(Default, Debug, Deserialize)]
pub struct ScenarioInfo {
    pub name: String,
    pub starting_direction: Direction,
}

#[derive(Default, Debug, Deserialize)]
pub struct ScenarioMap {
    pub osm_data: String,
    pub height_map: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct ScenarioStop {
    pub name: String,
    pub node_id: i64,
}

#[derive(Default, Debug, Deserialize)]
pub struct ScenarioOrigin {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Default, Debug, Deserialize, Resource)]
pub struct ScenarioData {
    pub info: ScenarioInfo,
    pub origin: ScenarioOrigin,
    pub map: ScenarioMap,
    pub stops: Vec<ScenarioStop>,
}

impl ScenarioData {
    pub fn load_from_file(file_name: &str) -> Self {
        let data = std::fs::read_to_string(file_name).expect("file to be readable");

        toml::from_str(&data).expect("scenario to be valid")
    }
}
