use super::Train3DModel;
use crate::{train::PaintScheme, TRAIN_HEIGHT, TRAIN_HEIGHT_OFFSET};
use bevy::prelude::*;

#[coverage(off)]
pub fn system(
    trains: Query<(Entity, &PaintScheme), Without<Train3DModel>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, paint_scheme) in trains.iter() {
        let color: Color = paint_scheme.color.into();

        commands
            .entity(entity)
            .insert(Train3DModel::default())
            .insert(PbrBundle {
                mesh: meshes.add(Cuboid::new(20.0, TRAIN_HEIGHT, 4.0)),
                material: materials.add(color),
                transform: Transform::from_xyz(0.0, TRAIN_HEIGHT_OFFSET, -0.0),
                ..default()
            });
    }
}
