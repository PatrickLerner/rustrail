use super::*;
use crate::{
    landscape::{CoordinatePoint, Path},
    train::render::Train3DModel,
};
use coverage_helper::test;
use std::{collections::HashMap, time::Duration};

#[coverage(off)]
fn gen_data() -> OSMData {
    let mut rails = HashMap::default();
    rails.insert(
        (0, 1),
        Path {
            start_id: 1,
            end_id: 2,
            start_coords: CoordinatePoint(0.0, 0.0),
            end_coords: CoordinatePoint(100.0, 100.0),
            ..default()
        },
    );
    OSMData { rails, ..default() }
}

#[test]
fn apply_transform_forwards() {
    let mut app = App::new();

    app.add_systems(Update, system);

    app.insert_resource(HeightMap::test_dummy());
    app.insert_resource(OriginOffset(CoordinatePoint(0.0, 0.0)));
    app.insert_resource(gen_data());

    let location = TrackLocation {
        id: (0, 1),
        travel_direction: Direction::Forward,
        distance: 0.0,
    };

    let train_id = app
        .world
        .spawn((Train3DModel, Transform::default(), location))
        .id();

    {
        app.init_resource::<Time>();
        let mut time = app.world.resource_mut::<Time>();
        time.advance_by(Duration::from_millis(1500));
    }

    app.update();

    {
        let mut transform = app.world.query::<&Transform>();
        let transform = transform.get(&app.world, train_id).unwrap();
        assert_eq!(transform.translation.x, 0.0);
        assert_eq!(transform.translation.z, 0.0);
        assert_eq!(transform.translation.y, HEIGHT_OFFSET + TRAIN_HEIGHT_OFFSET);
    }

    let location = TrackLocation {
        id: (0, 1),
        travel_direction: Direction::Forward,
        distance: 141.42135624,
    };

    let train_id = app
        .world
        .spawn((Train3DModel, Transform::default(), location))
        .id();

    {
        let mut time = app.world.resource_mut::<Time>();
        time.advance_by(Duration::from_millis(100));
    }
    app.update();

    {
        let mut transform = app.world.query::<&Transform>();
        let transform = transform.get(&app.world, train_id).unwrap();
        // moves partially to final destination
        assert_eq!(transform.translation.x, 30.0);
        assert_eq!(transform.translation.z, -30.0);
        assert_eq!(transform.translation.y, -29.1);
    }

    {
        let mut time = app.world.resource_mut::<Time>();
        time.advance_by(Duration::from_millis(10000));
    }
    app.update();

    {
        let mut transform = app.world.query::<&Transform>();
        let transform = transform.get(&app.world, train_id).unwrap();
        // moves partially to final destination
        assert_eq!(transform.translation.x, 100.0);
        assert_eq!(transform.translation.z, -100.0);
        assert_eq!(transform.translation.y, HEIGHT_OFFSET + TRAIN_HEIGHT_OFFSET);
    }
}

#[test]
fn apply_transform_backwards() {
    let mut app = App::new();

    app.add_systems(Update, system);

    app.insert_resource(HeightMap::test_dummy());
    app.insert_resource(OriginOffset(CoordinatePoint(0.0, 0.0)));
    app.insert_resource(gen_data());

    let location = TrackLocation {
        id: (0, 1),
        travel_direction: Direction::Backward,
        distance: 0.0,
    };

    let train_id = app
        .world
        .spawn((Train3DModel, Transform::default(), location))
        .id();

    {
        app.init_resource::<Time>();
        let mut time = app.world.resource_mut::<Time>();
        time.advance_by(Duration::from_millis(1500));
    }

    app.update();

    {
        let mut transform = app.world.query::<&Transform>();
        let transform = transform.get(&app.world, train_id).unwrap();
        assert_eq!(transform.translation.x, 100.0);
        assert_eq!(transform.translation.z, -100.0);
        assert_eq!(transform.translation.y, HEIGHT_OFFSET + TRAIN_HEIGHT_OFFSET);
    }

    let location = TrackLocation {
        id: (0, 1),
        travel_direction: Direction::Backward,
        distance: 141.42135624,
    };

    let train_id = app
        .world
        .spawn((Train3DModel, Transform::default(), location))
        .id();

    {
        let mut time = app.world.resource_mut::<Time>();
        time.advance_by(Duration::from_millis(100));
    }
    app.update();

    {
        let mut transform = app.world.query::<&Transform>();
        let transform = transform.get(&app.world, train_id).unwrap();
        // moves partially to final destination
        assert_eq!(transform.translation.x.round(), 0.0);
        assert_eq!(transform.translation.z.round(), 0.0);
        assert_eq!(transform.translation.y, -29.1);
    }
}
