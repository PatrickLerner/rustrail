mod osm_data;
mod path;

use bevy::prelude::*;
pub use osm_data::OSMData;
pub use path::{Path, PathId};

#[coverage(off)]
pub fn load_data(mut commands: Commands) {
    // TODO: make configurable
    let file_name = "assets/hessen-latest.osm.pbf";
    let parsed_file_name = format!("{}.bin", file_name);

    let data = if let Ok(data) = OSMData::load_from_file(&parsed_file_name) {
        // TODO: fail fallback if load fails
        data
    } else {
        let data = OSMData::parse_file(file_name);
        data.save_to_file(&parsed_file_name);
        data
    };
    commands.insert_resource(data);
}
