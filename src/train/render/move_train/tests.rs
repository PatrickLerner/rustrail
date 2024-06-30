use std::time::Duration;

use crate::landscape::CoordinatePoint;

use super::*;
use coverage_helper::test;

#[test]
fn apply_transform() {
    let mut app = App::new();

    app.add_systems(Update, system);

    app.init_resource::<Time>();
    let mut time = app.world.resource_mut::<Time>();
    time.advance_by(Duration::from_millis(1500));

    app.insert_resource(HeightMap::test_dummy());
    app.insert_resource(OriginOffset(CoordinatePoint(0.0, 0.0)));

    let train_id = app
        .world
        .spawn((Train3DModel::default(), Transform::default(), Speed(1.0)))
        .id();

    app.update();

    let mut transform = app.world.query::<&Transform>();
    let transform = transform.get(&app.world, train_id).unwrap();
    assert_eq!(transform.translation.x, 1.5);
    assert_eq!(transform.translation.z, 0.0);
    assert_eq!(transform.translation.y, HEIGHT_OFFSET + TRAIN_HEIGHT_OFFSET);

    let train_id = app
        .world
        .spawn((Train3DModel::default(), Transform::default(), Speed(30.0)))
        .id();

    let mut time = app.world.resource_mut::<Time>();
    time.advance_by(Duration::from_millis(2000));

    app.update();

    let mut transform = app.world.query::<&Transform>();
    let transform = transform.get(&app.world, train_id).unwrap();
    assert_eq!(transform.translation.x, 60.0);
    assert_eq!(transform.translation.z, 0.0);
    assert_eq!(transform.translation.y, HEIGHT_OFFSET + TRAIN_HEIGHT_OFFSET);
}
