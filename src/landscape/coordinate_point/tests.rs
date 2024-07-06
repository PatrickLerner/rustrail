use super::*;
use coverage_helper::test;

#[test]
fn coord_arithmetic() {
    let z = CoordinatePoint(0.0, 0.0);
    let a = CoordinatePoint(1.0, 1.0);
    let b = CoordinatePoint(2.0, 2.0);
    let i_a = CoordinatePoint(-1.0, 1.0);
    let i_b = CoordinatePoint(-2.0, 2.0);

    assert_eq!(a + a, b);
    assert_eq!(a - a, z);

    assert_eq!(i_a + i_a, i_b);
    assert_eq!(i_a - i_a, z);

    assert_eq!(b * i_a, i_b);
}

#[test]
fn f64_arithmetic() {
    let z = CoordinatePoint(0.0, 0.0);
    let a = CoordinatePoint(1.0, 1.0);
    let b = CoordinatePoint(2.0, 2.0);
    let i_a = CoordinatePoint(-1.0, 1.0);

    assert_eq!(a + 1.0, b);

    assert_eq!(a * 2.0, b);
    assert_eq!(a * 0.0, z);
    assert_eq!(i_a * 0.0, z);
    assert_eq!(i_a * -1.0 * -1.0, i_a);

    assert_eq!(b / 2.0, a);
}

#[test]
fn vec2() {
    let z = CoordinatePoint(0.0, 0.0);
    let a = CoordinatePoint(1.0, 1.0);
    let i_a = CoordinatePoint(-1.0, 1.0);

    let z: Vec2 = z.into();
    let a: Vec2 = a.into();
    let i_a: Vec2 = i_a.into();

    assert_eq!(z, Vec2::new(0.0, 0.0));
    assert_eq!(a, Vec2::new(1.0, 1.0));
    assert_eq!(i_a, Vec2::new(-1.0, 1.0));
}

#[test]
fn sector_coordinates() {
    let z = CoordinatePoint(0.0, 0.0);
    let a = CoordinatePoint(1000.0, 1500.0);
    let b = CoordinatePoint(499.0, 499.0);
    let c = CoordinatePoint(-500.1, 499.0);

    assert_eq!(z.sector_coordinates(), (0, 0));
    assert_eq!(a.sector_coordinates(), (1, 2));
    assert_eq!(b.sector_coordinates(), (0, 0));
    assert_eq!(c.sector_coordinates(), (-1, 0));
}

#[test]
fn coordinate_view() {
    let coordinates = Coordinates(vec![
        CoordinatePoint(0.0, 0.0),
        CoordinatePoint(-10.0, 100.0),
        CoordinatePoint(100.0, 100.0),
    ]);

    let view = coordinates.view_for_landscape_position(&CoordinatePoint(10.0, 10.0));

    assert_eq!(view.list.len(), 3);
    assert_eq!(view.list[0], CoordinatePoint(-45.0, 50.0));

    // NOTE: the view also negates all y values
    assert_eq!(view.min_x, -20.0);
    assert_eq!(view.max_x, 90.0);
    assert_eq!(view.min_y, -90.0);
    assert_eq!(view.max_y, 10.0);

    assert_eq!(view.center, CoordinatePoint(35.0, -40.0));
}

#[test]
fn serialization() {
    let coord = CoordinatePoint(123.0, 456.0);
    let data = bincode::serialize(&coord).unwrap();
    let coord: CoordinatePoint = bincode::deserialize(&data).unwrap();
    assert_eq!(coord.0, 123.0);
    assert_eq!(coord.1, 456.0);
}
