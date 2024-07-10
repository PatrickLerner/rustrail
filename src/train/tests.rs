use super::*;
use coverage_helper::test;
use physics::TrainPhysicsPlugin;
use render::TrainRenderPlugin;

#[test]
fn direction_reverse() {
    assert_eq!(Direction::Forward.opposite(), Direction::Backward);
    assert_eq!(Direction::Backward.opposite(), Direction::Forward);
}

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
}

#[test]
fn plugin() {
    let mut app = App::default();
    app.add_plugins(TrainPlugins);
    assert!(app.is_plugin_added::<TrainRenderPlugin>());
    assert!(app.is_plugin_added::<TrainPhysicsPlugin>());
}

#[test]
fn train_composition() {
    let train_composition = TrainComposition {
        components: vec![
            TrainComponent::Engine(Entity::from_raw(1)),
            TrainComponent::Engine(Entity::from_raw(2)),
            TrainComponent::Wagon(Entity::from_raw(3)),
        ],
    };

    assert_eq!(
        train_composition.entities(),
        vec![
            Entity::from_raw(1),
            Entity::from_raw(2),
            Entity::from_raw(3),
        ],
    )
}

#[test]
fn wrapped_value_derived() {
    let mut items: Vec<Box<dyn WrappedValue>> = vec![
        Box::new(Mass(0.0)),
        Box::new(Speed(0.0)),
        Box::new(MaxSpeed(0.0)),
        Box::new(ForceDriving(0.0)),
        Box::new(ForceBraking(0.0)),
        Box::new(ForceFriction(0.0)),
        Box::new(ForceAirResistance(0.0)),
    ];

    for item in items.iter_mut() {
        assert_eq!(item.get(), 0.0);
        item.set(item.get() + 1.0);
        assert_eq!(item.get(), 1.0);
    }
}

#[test]
fn meter_per_second_conversions() {
    let speed = Speed(10.0);
    assert_eq!(speed.as_kmh(), 36.0);

    let max_speed = MaxSpeed::from_kmh(36.0);
    assert_eq!(max_speed.0, 10.0);
}
