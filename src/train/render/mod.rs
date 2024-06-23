#[cfg(test)]
mod tests;

use crate::{
    landscape::{HeightMap, OriginOffset},
    train::{PaintScheme, Speed},
    HEIGHT_OFFSET, TRAIN_HEIGHT, TRAIN_HEIGHT_OFFSET,
};
use bevy::prelude::*;

#[derive(Component)]
struct Train3DModel;

fn spawn_train(
    trains: Query<(Entity, &PaintScheme), Without<Train3DModel>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, paint_scheme) in trains.iter() {
        let color: Color = paint_scheme.color.into();

        commands
            .entity(entity)
            .insert(Train3DModel)
            .insert(PbrBundle {
                mesh: meshes.add(Cuboid::new(20.0, TRAIN_HEIGHT, 4.0)),
                material: materials.add(color),
                transform: Transform::from_xyz(0.0, TRAIN_HEIGHT_OFFSET, 0.0),
                ..default()
            });
    }
}

fn move_train(
    mut trains: Query<(&mut Transform, &Speed), With<Train3DModel>>,
    time: Res<Time>,
    height_map: Res<HeightMap>,
    origin_offset: Res<OriginOffset>,
) {
    for (mut transform, speed) in trains.iter_mut() {
        if speed.0.abs() > f32::EPSILON {
            transform.translation.x += speed.0 * time.delta_seconds();

            let h = height_map.height_at_position(
                transform.translation.x as f64 + origin_offset.x,
                transform.translation.z as f64 + origin_offset.y,
            );

            transform.translation.y = h + HEIGHT_OFFSET + TRAIN_HEIGHT_OFFSET;
        }
    }
}

pub struct TrainRenderPlugin;

impl Plugin for TrainRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_train, move_train));
    }
}
