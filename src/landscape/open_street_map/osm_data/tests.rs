use super::*;
use coverage_helper::test;
use std::{fs::remove_file, path::Path};

fn temp_file() -> String {
    let temp_file = std::env::temp_dir().join("liechtenstein.osm.pbf.bin");
    temp_file.to_string_lossy().to_string()
}

#[test]
fn parse_file() {
    let file_name = "assets/liechtenstein-latest.osm.pbf";
    let data = OSMData::parse_file(&file_name);
    assert_eq!(data.rails.len(), 299); // too small!
    assert_eq!(data.sections.len(), 290);

    let section = data.sections.get(&(1058, 5949)).unwrap();
    assert_eq!(section.buildings.len(), 5);
    assert_eq!(section.areas.len(), 0);
    assert_eq!(section.rails.len(), 0);

    let section = data.sections.get(&(1059, 5959)).unwrap();
    assert_eq!(section.buildings.len(), 232);
    assert_eq!(section.areas.len(), 0);
    assert_eq!(section.rails.len(), 0);

    let section = data.sections.get(&(1060, 5950)).unwrap();
    assert_eq!(section.areas.len(), 2);
    assert_eq!(section.rails.len(), 0);
    assert_eq!(section.buildings.len(), 30);

    for path in data.rails.values() {
        // every rail has at least one connection
        assert!(path.forward_connections.len() + path.backward_connections.len() > 0);
    }

    // save and load
    let parsed_file = temp_file();
    if Path::new(&parsed_file).exists() {
        remove_file(&parsed_file).unwrap();
    }

    data.save_to_file(&parsed_file);
    assert!(Path::new(&parsed_file).exists());

    let data = OSMData::load_from_file(&parsed_file).unwrap();
    assert_eq!(data.rails.len(), 299); // too small!
    assert_eq!(data.sections.len(), 290);

    remove_file(parsed_file).unwrap();
}

#[test]
// tests the travel_direction.opposite() case
// we need to manipulate the data a bit because liechtenstein has only
// one track direction
fn test_opposite_travel_direction() {
    let file_name = "assets/liechtenstein-latest.osm.pbf";
    let mut data = OSMData::parse_file(&file_name);

    let key = data.rails.keys().next().unwrap().clone();
    let rail = data.rails.get_mut(&key).unwrap();
    // create a new rail that joins opposite onto the track
    let mut rail_copy = rail.clone();
    rail_copy.end_id = 1;
    rail_copy.end_coords = rail.start_coords + (rail.start_coords - rail.end_coords);
    data.rails.insert(rail_copy.id(), rail_copy);

    data.generate_path_connections();

    for path in data.rails.values() {
        // println!("{:?}", path);
        // every rail has at least one connection
        assert!(path.forward_connections.len() + path.backward_connections.len() > 0);
    }
}
