#[cfg(test)]
mod tests;

mod helpers;

use super::{Path, PathId};
use crate::{
    landscape::{coordinate_point::Coordinates, CoordinatePoint},
    train::Direction,
};
use bevy::prelude::*;
use helpers::*;
use proj::Proj;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

#[derive(Default, Resource, Debug, Deserialize, Serialize)]
pub struct OSMData {
    pub rails: HashMap<PathId, Path>,
    pub sections: HashMap<(i64, i64), SectionData>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct SectionData {
    pub buildings: Vec<BuildingData>,
    pub areas: Vec<AreaData>,
    pub rails: Vec<PathId>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct BuildingData {
    pub building_type: BuildingType,
    pub coordinates: Coordinates,
    pub levels: Option<u8>,
    pub layer: Option<u8>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct AreaData {
    pub area_type: AreaType,
    pub coordinates: Coordinates,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum BuildingType {
    Building,
    Industrial,
    Office,
    Commercial,
    Platform,
    Roof,
}

impl Default for BuildingType {
    fn default() -> Self {
        Self::Building
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum AreaType {
    Wood,
    Water,
    // River,
    // Stream,
    // Lake,
    // Pond,
}

impl Default for AreaType {
    fn default() -> Self {
        Self::Wood
    }
}

impl OSMData {
    pub fn load_from_file(file_name: &str) -> Result<Self, Box<dyn Error>> {
        let data = std::fs::read(file_name)?;

        #[cfg(not(coverage))]
        log::info!("Read parsed data file");

        // TODO: fail fallback if load fails
        let data: OSMData = bincode::deserialize(&data)?;
        Ok(data)
    }

    pub fn save_to_file(&self, file_name: &str) {
        let serialized_data = bincode::serialize(&self).unwrap();
        std::fs::write(file_name, serialized_data).expect("Unable to write file");

        #[cfg(not(coverage))]
        log::info!("saved parsed data file");
    }

    pub fn parse_file(file_name: &str) -> Self {
        // WGS84 to Mercator
        let converter = Proj::new_known_crs("EPSG:4326", "ESRI:53004", None).unwrap();

        let node_to_coordinates = |node: &osmpbfreader::Node| {
            static DM: f64 = 10_000_000.0;
            let lng = node.decimicro_lon as f64 / DM;
            let lat = node.decimicro_lat as f64 / DM;

            let result = converter.convert((lng, lat));

            let (x, y) = result.unwrap();

            CoordinatePoint(x, y)
        };

        let mut data = Self::default();

        let r = std::fs::File::open(std::path::Path::new(file_name)).unwrap();
        let mut pbf = osmpbfreader::OsmPbfReader::new(r);
        #[allow(unused_variables)] // for coverage
        let mut count = 0;

        #[cfg(not(coverage))]
        log::info!("generating data from OpenStreetMap");

        let objs = pbf.get_objs_and_deps(is_relevant_object);

        #[cfg(not(coverage))]
        log::info!("extracted data points, parsing");

        for obj_tree in objs.into_iter() {
            for (_id, obj) in obj_tree.iter() {
                if let osmpbfreader::OsmObj::Way(way) = obj {
                    let nodes: Vec<&osmpbfreader::Node> = way
                        .nodes
                        .iter()
                        .map(|node_id| obj_tree.get(&osmpbfreader::OsmId::Node(*node_id)).unwrap())
                        .filter_map(|obj| obj.node())
                        .collect();

                    if is_railway_platform(way) || is_building(way) || is_wood(way) {
                        // if nodes.len() == 2 {
                        // TODO: auto fix as a thin line
                        // }

                        #[cfg(not(coverage))]
                        if nodes.len() < 4 {
                            log::warn!("less than 4 nodes in way {:?} - ignored", way.id);
                            continue;
                        }
                    }

                    let coordinates: Vec<(i64, CoordinatePoint)> = nodes
                        .into_iter()
                        .map(|node| (node.id.0, node_to_coordinates(node)))
                        .collect();

                    if is_building(way) || is_railway_platform(way) || is_wood(way) || is_water(way)
                    {
                        let coordinates = coordinates.clone();
                        let sector = coordinates[0].1.sector_coordinates();

                        let sector = data
                            .sections
                            .entry(sector)
                            .or_insert_with(SectionData::default);

                        let coordinates: Vec<CoordinatePoint> = coordinates
                            .into_iter()
                            .map(|(_node, coordinate)| coordinate)
                            .collect();

                        if is_wood(way) {
                            let area = AreaData {
                                coordinates: Coordinates(coordinates),
                                ..default()
                            };

                            sector.areas.push(area);
                        } else if is_water(way) {
                            let area = AreaData {
                                area_type: AreaType::Water,
                                coordinates: Coordinates(coordinates),
                            };

                            sector.areas.push(area);
                        } else {
                            let building_type = if is_building(way) {
                                if is_industrial_building(way) {
                                    BuildingType::Industrial
                                } else if is_office_building(way) {
                                    BuildingType::Office
                                } else if is_commercial_building(way) {
                                    BuildingType::Commercial
                                } else if is_roof_building(way) {
                                    BuildingType::Roof
                                } else {
                                    BuildingType::Building
                                }
                            } else {
                                BuildingType::Platform
                            };

                            let mut building = BuildingData {
                                building_type,
                                coordinates: Coordinates(coordinates),
                                ..default()
                            };

                            for key in ["building:layer", "layer"] {
                                if let Some(layer) = way.tags.get(key) {
                                    if let Ok(layer) = layer.parse::<u8>() {
                                        building.layer = Some(layer);
                                    }
                                }
                            }

                            if let Some(levels) = way.tags.get("building:levels") {
                                if let Ok(levels) = levels.parse::<u8>() {
                                    building.levels = Some(levels);
                                }
                            }

                            if building.building_type == BuildingType::Roof
                                && building.layer.is_none()
                            {
                                building.layer = Some(1)
                            }

                            sector.buildings.push(building);
                        }
                    }

                    if is_rail(way) {
                        let mut node_iter = coordinates.iter();
                        let (mut last_node_id, mut last_node) = node_iter.next().unwrap();

                        for (next_node_id, next_node) in node_iter {
                            let end_coords = *next_node;
                            let start_coords = last_node;

                            let sector = CoordinatePoint(
                                (start_coords.0 + end_coords.0) / 2.0,
                                (start_coords.1 + end_coords.1) / 2.0,
                            )
                            .sector_coordinates();

                            let sector = data
                                .sections
                                .entry(sector)
                                .or_insert_with(SectionData::default);

                            let rail = Path {
                                start_id: last_node_id,
                                end_id: *next_node_id,
                                start_coords,
                                end_coords,
                                ..default()
                            };

                            sector.rails.push(rail.id());
                            data.rails.insert(rail.id(), rail);

                            (last_node_id, last_node) = (*next_node_id, *next_node);
                        }
                    }

                    count += 1;
                }
            }
        }

        #[cfg(not(coverage))]
        log::info!("{} data points extracted", count);

        data.generate_path_connections();
        data
    }

    fn generate_path_connections(&mut self) {
        #[cfg(not(coverage))]
        log::info!("generating path connections");

        #[allow(unused_variables)] // for coverage
        let mut count = 0;

        let mut path_node_id_lookup = HashMap::new();
        for rail in self.rails.values() {
            path_node_id_lookup
                .entry(rail.start_id)
                .and_modify(|list: &mut Vec<Path>| list.push(rail.clone()))
                .or_insert_with(|| vec![rail.clone()]);

            path_node_id_lookup
                .entry(rail.end_id)
                .and_modify(|list: &mut Vec<Path>| list.push(rail.clone()))
                .or_insert_with(|| vec![rail.clone()]);
        }

        for (id, rail) in self.rails.iter_mut() {
            for travel_direction in [Direction::Forward, Direction::Backward] {
                let (end_id, point_coords, point_other) = match travel_direction {
                    Direction::Forward => (rail.end_id, rail.end_coords, rail.start_coords),
                    Direction::Backward => (rail.start_id, rail.start_coords, rail.end_coords),
                };

                let possible_next_paths: Vec<(PathId, Direction)> = path_node_id_lookup
                    .get(&end_id)
                    .unwrap()
                    .iter()
                    .filter_map(|p| {
                        if p.id() == *id {
                            return None;
                        }

                        let pi = std::f64::consts::PI;
                        let two_pi = 2.0 * pi;

                        let path_other = if p.start_coords == point_coords {
                            p.end_coords
                        } else {
                            p.start_coords
                        };

                        let path_angle = f64::atan2(
                            path_other.1 - point_coords.1,
                            path_other.0 - point_coords.0,
                        );

                        let point_angle = f64::atan2(
                            point_other.1 - point_coords.1,
                            point_other.0 - point_coords.0,
                        );

                        let diff = point_angle - path_angle;
                        let diff = (diff + two_pi) % two_pi;

                        const MAX_ALLOWED: f64 = 0.8;

                        if (diff <= pi + MAX_ALLOWED) && (diff >= pi - MAX_ALLOWED) {
                            let new_direction =
                                if (p.end_id == rail.start_id) || (p.start_id == rail.end_id) {
                                    travel_direction
                                } else {
                                    travel_direction.opposite()
                                };

                            return Some((p.id(), new_direction));
                        }

                        None
                    })
                    .collect();

                match travel_direction {
                    Direction::Forward => rail.forward_connections = possible_next_paths,
                    Direction::Backward => rail.backward_connections = possible_next_paths,
                }
            }

            count += 1;
        }

        #[cfg(not(coverage))]
        log::info!("{} path connections created", count);
    }
}
