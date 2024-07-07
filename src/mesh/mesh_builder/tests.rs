use super::*;
use coverage_helper::test;
use geo::Polygon;

#[test]
fn base() {
    let builder = MeshBuilder::new();
    let mesh = builder.build();

    assert_eq!(mesh.count_vertices(), 0);
    assert_eq!(mesh.indices().unwrap().len(), 0);
}

#[test]
fn triangle() {
    let mut builder = MeshBuilder::new();

    let v1 = builder.add_vertex(Vertex {
        pos: Vec3::new(0.0, 0.0, 0.0),
        normal: Vec3::Y,
        uv: Vec2::ZERO,
    });

    let v2 = builder.add_vertex(Vertex {
        pos: Vec3::new(1.0, 0.0, 0.0),
        normal: Vec3::Y,
        uv: Vec2::ZERO,
    });

    let v3 = builder.add_vertex(Vertex {
        pos: Vec3::new(1.0, 0.0, 1.0),
        normal: Vec3::Y,
        uv: Vec2::ZERO,
    });

    builder.add_triangle(v1, v2, v3);

    let mesh = builder.build();

    assert_eq!(mesh.count_vertices(), 3);
    assert_eq!(mesh.indices().unwrap().len(), 3);
}

#[test]
fn quad() {
    let mut builder = MeshBuilder::new();

    builder.add_quad(
        [
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 1.0),
        ],
        Vec3::Y,
    );

    let mesh = builder.build();

    assert_eq!(mesh.count_vertices(), 4);
    assert_eq!(mesh.indices().unwrap().len(), 6);
}

#[test]
fn triangulate_hexagon() {
    let mut builder = MeshBuilder::new();

    let hexagon = Polygon::new(
        LineString::from(vec![
            (1.0, 0.0),
            (0.5, 0.866),
            (-0.5, 0.866),
            (-1.0, 0.0),
            (-0.5, -0.866),
            (0.5, -0.866),
        ]),
        vec![],
    );

    builder.triangulate_polygon(&hexagon, 0.0, Vec3::Y);

    let mesh = builder.build();

    assert_eq!(mesh.count_vertices(), 7);
    assert_eq!(mesh.indices().unwrap().len(), 12);
}

#[test]
fn triangulate_hexagon_with_hole() {
    let mut builder = MeshBuilder::new();

    let hexagon = Polygon::new(
        LineString::from(vec![
            (1.0, 0.0),
            (0.5, 0.866),
            (-0.5, 0.866),
            (-1.0, 0.0),
            (-0.5, -0.866),
            (0.5, -0.866),
            (1.0, 0.0),
        ]),
        vec![LineString::from(vec![
            (0.25, 0.0),
            (0.0, 0.433),
            (-0.25, 0.0),
            (0.0, -0.433),
            (0.25, 0.0),
        ])],
    );

    builder.triangulate_polygon(&hexagon, 0.0, Vec3::Y);

    let mesh = builder.build();

    assert_eq!(mesh.count_vertices(), 12);
    assert_eq!(mesh.indices().unwrap().len(), 30);
}
