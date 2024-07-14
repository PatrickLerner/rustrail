use super::*;
use coverage_helper::test;

#[test]
fn bundle_initializer() {
    let mut app = App::default();

    let bundle = EngineBundle::from_file("assets/models/BR111.toml");

    assert!(bundle.max_power.0 > 0.0);
    assert!(bundle.max_speed.0 > 0.0);
    assert!(bundle.mass.0 > 0.0);
    assert_eq!(bundle.name.0, "");

    app.world_mut().spawn(bundle);

    let bundle = EngineBundle::default();

    assert_eq!(bundle.max_power.0, 0.0);
    assert_eq!(bundle.max_speed.0, 0.0);
    assert_eq!(bundle.name.0, "");

    let bundle = WagonBundle::from_file("assets/models/eanos.toml");

    assert!(bundle.mass.0 > 0.0);
    assert!(bundle.max_speed.0 > 0.0);

    app.world_mut().spawn(bundle);

    let bundle = WagonBundle::default();

    assert_eq!(bundle.mass.0, 0.0);
    assert_eq!(bundle.speed.0, 0.0);
    assert_eq!(bundle.max_speed.0, 0.0);

    app.world_mut().spawn(bundle);
}

#[test]
fn train_bundle_initializer() {
    let mut app = App::default();

    let bundle = TrainBundle::default();
    assert_eq!(bundle.speed.0, 0.0);

    app.world_mut().spawn(bundle);

    let bundle = TrainBundle::new("Test", vec![TrainComponent::Engine(Entity::from_raw(123))]);
    assert_eq!(bundle.name.0, "Test");
    assert_eq!(bundle.composition.entities(), vec![Entity::from_raw(123)]);

    app.world_mut().spawn(bundle);
}
