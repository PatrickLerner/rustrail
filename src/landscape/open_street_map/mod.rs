mod path;

use super::CoordinatePoint;
use crate::train::Direction;
use bevy::prelude::*;
use osmpbfreader::{OsmObj, Way};
use proj::Proj;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use path::{Path, PathId};

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

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct AreaData {
    pub area_type: AreaType,
    pub coordinates: Vec<CoordinatePoint>,
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

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct BuildingData {
    pub building_type: BuildingType,
    pub coordinates: Vec<CoordinatePoint>,
    pub levels: Option<u8>,
    pub layer: Option<u8>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct SectionData {
    pub buildings: Vec<BuildingData>,
    pub areas: Vec<AreaData>,
    pub rails: Vec<PathId>,
    pub corner_position_heights: [f32; 4],
}

#[derive(Default, Resource, Debug, Deserialize, Serialize)]
pub struct OSMData {
    pub rails: HashMap<PathId, Path>,
    pub sections: HashMap<(i64, i64), SectionData>,
}

fn is_rail(obj: &Way) -> bool {
    obj.tags.contains("railway", "rail")
}

fn is_wood(obj: &Way) -> bool {
    obj.tags.contains("natural", "wood")
}

fn is_water(obj: &Way) -> bool {
    obj.tags.contains_key("water")
}

fn is_building(obj: &Way) -> bool {
    obj.tags.contains_key("building")
}

fn is_roof_building(obj: &Way) -> bool {
    obj.tags.contains("building", "roof")
}

fn is_industrial_building(obj: &Way) -> bool {
    obj.tags.contains("building", "industrial")
}

fn is_office_building(obj: &Way) -> bool {
    obj.tags.contains("building", "office")
}

fn is_commercial_building(obj: &Way) -> bool {
    obj.tags.contains("building", "commercial")
}

fn is_railway_platform(obj: &Way) -> bool {
    obj.tags.contains("railway", "platform")
}

fn is_relevant_object(obj: &OsmObj) -> bool {
    if let OsmObj::Way(obj) = obj {
        return is_rail(obj)
            || is_building(obj)
            || is_railway_platform(obj)
            || is_wood(obj)
            || is_water(obj);
    }

    false
}

pub fn load_data(mut commands: Commands) {
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

    // TODO
    let file_name = "assets/hessen-latest.osm.pbf";
    let parsed_file_name = format!("{}.bin", file_name);

    if let Ok(data) = std::fs::read(&parsed_file_name) {
        log::info!("Read parsed data file");

        // TODO: fail fallback if load fails
        let data: OSMData = bincode::deserialize(&data).unwrap();
        commands.insert_resource(data);
        return;
    }

    let mut data = OSMData::default();

    {
        let r = std::fs::File::open(std::path::Path::new(file_name)).unwrap();
        let mut pbf = osmpbfreader::OsmPbfReader::new(r);
        let mut count = 0;

        log::info!("generating data from OpenStreetMap");

        let objs = pbf.get_objs_and_deps(is_relevant_object);

        log::info!("extracted data points, parsing");

        for obj_tree in objs.into_iter() {
            for (_id, obj) in obj_tree.iter() {
                if let osmpbfreader::OsmObj::Way(way) = obj {
                    let nodes: Vec<&osmpbfreader::Node> = way
                        .nodes
                        .iter()
                        .map(|node_id| obj_tree.get(&osmpbfreader::OsmId::Node(*node_id)).unwrap())
                        .filter_map(|obj| {
                            if let osmpbfreader::OsmObj::Node(node) = obj {
                                Some(node)
                            } else {
                                None
                            }
                        })
                        .collect();

                    if is_railway_platform(way) || is_building(way) || is_wood(way) {
                        if nodes.len() == 2 {
                            // TODO: auto fix as a thin line
                        }

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
                                area_type: AreaType::Wood,
                                coordinates,
                            };

                            sector.areas.push(area);
                        } else if is_water(way) {
                            let area = AreaData {
                                area_type: AreaType::Water,
                                coordinates,
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
                                levels: None,
                                layer: None,
                                coordinates,
                            };

                            if let Some(layer) = way.tags.get("building:layer") {
                                if let Ok(layer) = layer.parse::<u8>() {
                                    building.layer = Some(layer);
                                }
                            }

                            if let Some(layer) = way.tags.get("layer") {
                                if let Ok(layer) = layer.parse::<u8>() {
                                    building.layer = Some(layer);
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
                    } else if is_rail(way) {
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
                                forward_connections: vec![],
                                backwards_connections: vec![],
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

        log::info!("{} data points extracted", count);
    }

    log::info!("generating path connections");

    let mut count = 0;

    let mut path_node_id_lookup = HashMap::new();
    for rail in data.rails.values() {
        path_node_id_lookup
            .entry(rail.start_id)
            .and_modify(|list: &mut Vec<Path>| list.push(rail.clone()))
            .or_insert_with(|| vec![rail.clone()]);

        path_node_id_lookup
            .entry(rail.end_id)
            .and_modify(|list: &mut Vec<Path>| list.push(rail.clone()))
            .or_insert_with(|| vec![rail.clone()]);
    }

    for (id, rail) in data.rails.iter_mut() {
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

                    let path_angle =
                        f64::atan2(path_other.1 - point_coords.1, path_other.0 - point_coords.0);

                    let point_angle = f64::atan2(
                        point_other.1 - point_coords.1,
                        point_other.0 - point_coords.0,
                    );

                    let diff = point_angle - path_angle;
                    let diff = (diff + two_pi) % two_pi;

                    const MAX_ALLOWED: f64 = 0.8;

                    let new_direction = {
                        if (diff <= pi + MAX_ALLOWED) && (diff >= pi - MAX_ALLOWED) {
                            if (p.end_id == rail.start_id) || (p.start_id == rail.end_id) {
                                Some(travel_direction)
                            } else {
                                Some(travel_direction.opposite())
                            }
                        } else {
                            None
                        }
                    };

                    if let Some(new_direction) = new_direction {
                        return Some((p.id(), new_direction));
                    }

                    None
                })
                .collect();

            match travel_direction {
                Direction::Forward => rail.forward_connections = possible_next_paths,
                Direction::Backward => rail.backwards_connections = possible_next_paths,
            }
        }

        count += 1;
    }

    log::info!("{} path connections created", count);

    let serialized_data = bincode::serialize(&data).unwrap();
    std::fs::write(parsed_file_name, serialized_data).expect("Unable to write file");
    log::info!("saved parsed data file");

    commands.insert_resource(data);
}
