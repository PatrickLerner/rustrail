use super::*;
use coverage_helper::test;

#[test]
fn test_generate_mesh_clockwise() {
    let path_2d = vec![
        CoordinatePoint(0.0, 0.0),
        CoordinatePoint(1.0, 0.0),
        CoordinatePoint(1.0, 1.0),
        CoordinatePoint(0.0, 1.0),
    ];
    let extrude_amount = 1.0;

    let mesh = generate_3d_mesh(path_2d, extrude_amount);

    // Perform assertions
    assert_eq!(mesh.count_vertices(), 26);
    assert_eq!(mesh.indices().unwrap().len(), 36);
}

#[test]
fn test_generate_mesh_counter_clockwise() {
    let path_2d = vec![
        CoordinatePoint(0.0, 0.0),
        CoordinatePoint(0.0, 1.0),
        CoordinatePoint(1.0, 1.0),
        CoordinatePoint(1.0, 0.0),
    ];
    let extrude_amount = 1.0;

    let mesh = generate_3d_mesh(path_2d, extrude_amount);

    // Perform assertions
    assert_eq!(mesh.count_vertices(), 26);
    assert_eq!(mesh.indices().unwrap().len(), 36);
}

#[test]
fn test_generate_mesh_complex_polygon() {
    let path_2d = vec![
        CoordinatePoint(0.0, 0.0),
        CoordinatePoint(2.0, 0.0),
        CoordinatePoint(2.0, 2.0),
        CoordinatePoint(1.0, 3.0),
        CoordinatePoint(0.0, 2.0),
    ];
    let extrude_amount = 1.0;

    let mesh = generate_3d_mesh(path_2d, extrude_amount);

    // Perform assertions
    assert_eq!(mesh.count_vertices(), 32);
    assert_eq!(mesh.indices().unwrap().len(), 48);
}
