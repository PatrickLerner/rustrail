use super::*;
use coverage_helper::test;

#[test]
fn spawns_landscapes() {
    let mut app = App::new();
    app.add_systems(Update, system);

    app.world.spawn(GameCameraState::default());
    app.world.insert_resource(OriginOffset { x: 0.0, y: 0.0 });

    assert_eq!(app.world.query::<&Landscape>().iter(&app.world).len(), 0);

    app.update();

    let count = (4 * SPAWN_RADIUS * SPAWN_RADIUS) as usize;

    assert_eq!(
        app.world.query::<&Landscape>().iter(&app.world).len(),
        count
    );

    for mut landscape in app.world.query::<&mut Landscape>().iter_mut(&mut app.world) {
        landscape.ttl = DEFAULT_TTL - 10.0;
    }

    app.update();

    assert_eq!(
        app.world.query::<&Landscape>().iter(&app.world).len(),
        count
    );

    for landscape in app.world.query::<&Landscape>().iter(&mut app.world) {
        assert_eq!(landscape.ttl, DEFAULT_TTL);
    }
}
