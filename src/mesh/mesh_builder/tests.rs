use super::*;
use coverage_helper::test;

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
