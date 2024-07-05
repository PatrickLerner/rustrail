use super::*;
use crate::{
    landscape::{CoordinatePoint, Path},
    train::Direction,
};
use coverage_helper::test;
use std::collections::HashMap;

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
fn updates_distance() {
    let data = gen_data();

    let mut location = TrackLocation {
        id: (0, 1),
        travel_direction: Direction::Forward,
        distance: 140.0,
    };

    location.add_distance(&data, 10.0);

    {
        assert_eq!(location.id, (1, 2));
        assert_eq!(location.distance.floor(), 8.0);
        assert_eq!(location.travel_direction, Direction::Forward);
    }

    location.add_distance(&data, 10.0);

    {
        assert_eq!(location.id, (1, 2));
        assert_eq!(location.distance.floor(), 18.0);
        assert_eq!(location.travel_direction, Direction::Forward);
    }

    location.add_distance(&data, -20.0);

    {
        assert_eq!(location.id, (0, 1));
        assert_eq!(location.distance.floor(), 140.0);
        assert_eq!(location.travel_direction, Direction::Forward);
    }
}
