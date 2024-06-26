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

/*
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
*/
