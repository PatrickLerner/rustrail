use super::*;
use coverage_helper::test;

#[test]
fn spawns_landscapes() {
    let mut app = App::new();
    app.add_systems(Update, system);

    app.world_mut().spawn(GameCameraState::default());
    app.world_mut()
        .insert_resource(OriginOffset(CoordinatePoint(0.0, 0.0)));

    assert_eq!(
        app.world_mut()
            .query::<&Landscape>()
            .iter(&app.world())
            .len(),
        0
    );

    app.update();

    let count = (4 * SPAWN_RADIUS * SPAWN_RADIUS) as usize;

    assert_eq!(
        app.world_mut()
            .query::<&Landscape>()
            .iter(&app.world())
            .len(),
        count
    );

    for mut landscape in app
        .world_mut()
        .query::<&mut Landscape>()
        .iter_mut(&mut app.world_mut())
    {
        landscape.ttl = DEFAULT_TTL - 10.0;
    }

    app.update();

    assert_eq!(
        app.world_mut()
            .query::<&Landscape>()
            .iter(&app.world())
            .len(),
        count
    );

    for landscape in app.world_mut().query::<&Landscape>().iter(&mut app.world()) {
        assert_eq!(landscape.ttl, DEFAULT_TTL);
    }
}
