use super::*;
use crate::{
    landscape::{CoordinatePoint, Path},
    train::{Direction, TrainComponent},
};
use coverage_helper::test;
use std::{collections::HashMap, f32::EPSILON, time::Duration};

#[coverage(off)]
fn gen_data() -> OSMData {
    let mut rails = HashMap::default();
    rails.insert(
        (0, 1),
        Path {
            start_id: 0,
            end_id: 1,
            start_coords: CoordinatePoint(0.0, 0.0),
            end_coords: CoordinatePoint(100.0, 100.0),
            forward_connections: vec![((1, 2), Direction::Forward)],
            ..default()
        },
    );
    rails.insert(
        (1, 2),
        Path {
            start_id: 1,
            end_id: 2,
            start_coords: CoordinatePoint(100.0, 100.0),
            end_coords: CoordinatePoint(300.0, 100.0),
            backward_connections: vec![((0, 1), Direction::Backward)],
            ..default()
        },
    );
    OSMData { rails, ..default() }
}

#[test]
fn updates_components() {
    let mut app = App::new();

    app.add_systems(Update, system);
    app.insert_resource(gen_data());

    let location = TrackLocation {
        id: (0, 1),
        travel_direction: Direction::Forward,
        distance: 140.0,
    };

    let engine_id = app.world.spawn(location.clone()).id();

    let train_id = app
        .world
        .spawn((
            Transform::default(),
            location,
            Speed(5.0),
            TrainComposition {
                components: vec![TrainComponent::Engine(engine_id)],
            },
        ))
        .id();

    {
        app.init_resource::<Time>();
        let mut time = app.world.resource_mut::<Time>();
        time.advance_by(Duration::from_millis(1500));
    }

    app.update();

    let mut location = app.world.query::<&TrackLocation>();

    {
        let location = location.get(&app.world, train_id).unwrap();
        assert_eq!(location.id, (1, 2));
        assert_eq!(location.distance.floor(), 6.0);
        assert_eq!(location.travel_direction, Direction::Forward);
    }

    {
        let location = location.get(&app.world, engine_id).unwrap();
        assert_eq!(location.id, (1, 2));
        assert_eq!(location.distance.floor(), 6.0);
        assert_eq!(location.travel_direction, Direction::Forward);
    }
}

#[test]
fn updates_distance() {
    let mut app = App::new();

    app.add_systems(Update, system);
    app.insert_resource(gen_data());

    let location = TrackLocation {
        id: (0, 1),
        travel_direction: Direction::Forward,
        distance: 140.0,
    };

    let train_id = app
        .world
        .spawn((
            Transform::default(),
            location,
            Speed(5.0),
            TrainComposition::default(),
        ))
        .id();

    {
        app.init_resource::<Time>();
        let mut time = app.world.resource_mut::<Time>();
        time.advance_by(Duration::from_millis(1500));
    }

    app.update();

    {
        let mut location = app.world.query::<&TrackLocation>();
        let location = location.get(&app.world, train_id).unwrap();
        assert_eq!(location.id, (1, 2));
        assert_eq!(location.distance.floor(), 6.0);
        assert_eq!(location.travel_direction, Direction::Forward);
    }

    {
        app.init_resource::<Time>();
        let mut time = app.world.resource_mut::<Time>();
        time.advance_by(Duration::from_millis(1500));
    }

    app.update();

    {
        let mut location = app.world.query::<&TrackLocation>();
        let location = location.get(&app.world, train_id).unwrap();
        assert_eq!(location.id, (1, 2));
        assert_eq!(location.distance.floor(), 13.0);
        assert_eq!(location.travel_direction, Direction::Forward);
    }

    {
        let mut speed = app.world.query::<&mut Speed>();
        let mut speed = speed.get_mut(&mut app.world, train_id).unwrap();
        speed.0 = -10.0;
    }

    {
        app.init_resource::<Time>();
        let mut time = app.world.resource_mut::<Time>();
        time.advance_by(Duration::from_millis(1500));
    }

    app.update();

    {
        let mut location = app.world.query::<&TrackLocation>();
        let location = location.get(&app.world, train_id).unwrap();
        assert_eq!(location.id, (0, 1));
        assert_eq!(location.distance.floor(), 140.0);
        assert_eq!(location.travel_direction, Direction::Forward);
    }
}

#[test]
fn super_low_speed() {
    let mut app = App::new();

    app.add_systems(Update, system);
    app.insert_resource(gen_data());

    let location = TrackLocation {
        id: (0, 1),
        travel_direction: Direction::Forward,
        distance: 0.0,
    };

    let train_id = app
        .world
        .spawn((
            Transform::default(),
            location,
            Speed(EPSILON / 2.0),
            TrainComposition::default(),
        ))
        .id();

    {
        app.init_resource::<Time>();
        let mut time = app.world.resource_mut::<Time>();
        time.advance_by(Duration::from_millis(1500));
    }

    app.update();

    {
        let mut location = app.world.query::<&TrackLocation>();
        let location = location.get(&app.world, train_id).unwrap();
        assert_eq!(location.id, (0, 1));
        assert_eq!(location.distance, 0.0);
        assert_eq!(location.travel_direction, Direction::Forward);
    }
}
