use super::Train3DModel;
use crate::{
    landscape::OSMData,
    train::{Dimension, Direction, PaintScheme, TrackLocation, TrainComposition},
    TRAIN_HEIGHT, TRAIN_HEIGHT_OFFSET,
};
use bevy::prelude::*;

const WAGON_DISTANCE: f64 = 1.0;

#[coverage(off)]
pub fn system(
    trains: Query<(Entity, &TrainComposition), Without<TrackLocation>>,
    engines: Query<(Entity, &PaintScheme, &Dimension), Without<Train3DModel>>,
    dimensions: Query<&Dimension>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    data: Res<OSMData>,
) {
    if trains.is_empty() {
        return;
    }

    const START_ID: i64 = 1687958953;
    let id = data
        .rails
        .keys()
        .find(
            #[coverage(off)]
            |(s, _e)| *s == START_ID,
        )
        .expect("to find rail with start id");

    for (entity, paint_scheme, dimension) in engines.iter() {
        let color: Color = paint_scheme.color.into();

        commands
            .entity(entity)
            .insert((
                Train3DModel,
                TrackLocation {
                    id: *id,
                    distance: 0.0,
                    travel_direction: Direction::Forward,
                },
                PbrBundle::default(),
            ))
            .with_children(
                #[coverage(off)]
                |parent| {
                    parent.spawn(PbrBundle {
                        mesh: meshes.add(Cuboid::new(dimension.length, TRAIN_HEIGHT, 4.0)),
                        material: materials.add(color),
                        transform: Transform::from_xyz(0.0, TRAIN_HEIGHT_OFFSET, 0.0),
                        ..default()
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
