use super::*;
use coverage_helper::test;

#[test]
fn id() {
    let path = Path {
        start_id: 69,
        end_id: 420,
        ..default()
    };

    assert_eq!(path.id(), (69, 420));
}

#[test]
fn possible_connections_by_direction() {
    let forward_connections = vec![((0, 0), Direction::Forward)];
    let backward_connections = vec![((1, 1), Direction::Backward)];

    let path = Path {
        forward_connections: forward_connections.clone(),
        backward_connections: backward_connections.clone(),
        ..default()
    };

    assert_eq!(
        path.possible_connections_by_direction(Direction::Forward),
        &forward_connections
    );
    assert_eq!(
        path.possible_connections_by_direction(Direction::Backward),
        &backward_connections
    );
}

#[test]
fn length() {
    let path = Path {
        start_coords: CoordinatePoint(0.0, 0.0),
        end_coords: CoordinatePoint(10.0, 0.0),
        ..default()
    };

    assert_eq!(path.length(), 10.0);

    let path = Path {
        start_coords: CoordinatePoint(0.0, 0.0),
        end_coords: CoordinatePoint(-33.0, -33.0),
        ..default()
    };

    assert_eq!(path.length().floor(), 46.0);

    let path = Path {
        start_coords: CoordinatePoint(0.0, 0.0),
        end_coords: CoordinatePoint(-33.0, 33.0),
        ..default()
    };

    assert_eq!(path.length().floor(), 46.0);
}

#[test]
fn angle() {
    let path = Path {
        start_coords: CoordinatePoint(0.0, 0.0),
        end_coords: CoordinatePoint(10.0, 0.0),
        ..default()
    };

    assert_eq!(path.angle(), 0.0);

    let path = Path {
        start_coords: CoordinatePoint(0.0, 0.0),
        end_coords: CoordinatePoint(-33.0, -33.0),
        ..default()
    };

    assert_eq!(path.angle(), std::f64::consts::PI * -0.75);

    let path = Path {
        start_coords: CoordinatePoint(0.0, 0.0),
        end_coords: CoordinatePoint(-100.0, 0.0),
        ..default()
    };

    assert_eq!(path.angle(), std::f64::consts::PI);

    let path = Path {
        start_coords: CoordinatePoint(0.0, 0.0),
        end_coords: CoordinatePoint(-50.0, 50.0),
        ..default()
    };

    assert_eq!(path.angle(), std::f64::consts::PI * 0.75);
}
