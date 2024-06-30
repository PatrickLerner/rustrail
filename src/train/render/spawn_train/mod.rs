use super::Train3DModel;
use crate::{
    landscape::OSMData,
    train::{render::TrainRailLocation, Direction, PaintScheme},
    TRAIN_HEIGHT, TRAIN_HEIGHT_OFFSET,
};
use bevy::prelude::*;

#[coverage(off)]
pub fn system(
    trains: Query<(Entity, &PaintScheme), Without<Train3DModel>>,
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

    for (entity, paint_scheme) in trains.iter() {
        let color: Color = paint_scheme.color.into();

        commands
            .entity(entity)
            .insert((
                Train3DModel,
                TrainRailLocation {
                    id: *id,
                    distance: 0.0,
                    travel_direction: Direction::Forward,
                },
            ))
            .insert(PbrBundle {
                mesh: meshes.add(Cuboid::new(20.0, TRAIN_HEIGHT, 4.0)),
                material: materials.add(color),
                transform: Transform::from_xyz(0.0, TRAIN_HEIGHT_OFFSET, -0.0),
                ..default()
            });
    }
}
