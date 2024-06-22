use super::*;

#[test]
fn higher_mass_higher_friction() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let light_train = app
        .world
        .spawn((
            ForceFriction::default(),
            Mass {
                engine: 7000.0,
                wagons: 0.0,
            },
        ))
        .id();

    let heavy_train = app
        .world
        .spawn((
            ForceFriction::default(),
            Mass {
                engine: 70000.0,
                wagons: 0.0,
            },
        ))
        .id();

    app.update();

    assert!(app.world.get::<ForceFriction>(light_train).is_some());
    assert!(app.world.get::<ForceFriction>(heavy_train).is_some());
    assert!(
        app.world.get::<ForceFriction>(light_train).unwrap().0
            < app.world.get::<ForceFriction>(heavy_train).unwrap().0
    );
}
