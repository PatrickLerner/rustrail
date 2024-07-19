use crate::{
    landscape::OSMData,
    scenario::ScenarioData,
    train::{Dimension, Direction, LoadModelFile, TrackLocation, TrainComposition},
    TRAIN_HEIGHT_OFFSET,
};
use bevy::prelude::*;

const WAGON_DISTANCE: f64 = 0.0;

#[coverage(off)]
pub fn system(
    trains: Query<(Entity, &TrainComposition), Without<TrackLocation>>,
    engines: Query<(Entity, &LoadModelFile)>,
    dimensions: Query<&Dimension>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    data: Res<OSMData>,
    scenario_data: Res<ScenarioData>,
) {
    if trains.is_empty() {
        return;
    }

    let start_rail = scenario_data.stops.first().unwrap().node_id;

    let id = data
        .rails
        .keys()
        .find(
            #[coverage(off)]
            |(s, _e)| *s == start_rail,
        )
        .expect("to find rail with start id");

    for (entity, load_model_file) in engines.iter() {
        let model = asset_server.load(format!("{}#Scene0", load_model_file.0));

        commands
            .entity(entity)
            .insert((
                TrackLocation {
                    id: *id,
                    distance: 0.0,
                    travel_direction: scenario_data.info.starting_direction,
                },
                PbrBundle::default(),
            ))
            .remove::<LoadModelFile>()
            .with_children(
                #[coverage(off)]
                |parent| {
                    parent.spawn(SceneBundle {
                        scene: model,
                        transform: Transform::from_xyz(0.0, TRAIN_HEIGHT_OFFSET, 0.0),
                        ..Default::default()
                    });
                },
            );
    }

    for (entity, composition) in trains.iter() {
        let mut location = TrackLocation {
            id: *id,
            distance: 0.0,
            travel_direction: Direction::Forward,
        };

        commands.entity(entity).insert(location.clone());

        for (index, component_entity) in composition.entities().into_iter().enumerate() {
            let dimension = dimensions
                .get(component_entity)
                .expect("component to have a dimension");

            let mut new_location = location.clone();
            if index > 0 {
                new_location.add_distance(&data, -dimension.length as f64 / 2.0 - WAGON_DISTANCE);
            }

            commands
                .entity(component_entity)
                .insert(new_location.clone());

            new_location.add_distance(&data, -dimension.length as f64 / 2.0);
            location = new_location;
        }
    }
}
