use std::time::Duration;

use super::*;
use coverage_helper::test;

#[test]
fn spawns_landscapes() {
    let mut app = App::new();

    app.add_systems(Update, system);

    let landscape_id = app
        .world_mut()
        .spawn((Landscape::default(), Transform::default()))
        .id();

    app.init_resource::<Time>();
    let mut time = app.world_mut().resource_mut::<Time>();
    time.advance_by(Duration::from_millis(500));

    app.update();

    let landscape = app
        .world_mut()
        .query::<&Landscape>()
        .get(&app.world(), landscape_id)
        .unwrap();
    assert_eq!(landscape.ttl, 29.5);

    let mut time = app.world_mut().resource_mut::<Time>();
    time.advance_by(Duration::from_millis(50000));

    app.update();

    let landscape = app
        .world_mut()
        .query::<&Landscape>()
        .get(&app.world(), landscape_id);

    assert!(landscape.is_err());
}
