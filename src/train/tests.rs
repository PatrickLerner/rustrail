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
fn mass_sum() {
    let mass = Mass {
        engine: 1.0,
        wagons: 2.0,
    };
    assert_eq!(mass.total(), 3.0);

    let mass = Mass {
        engine: -1.0,
        wagons: 2.0,
    };
    assert_eq!(mass.total(), 1.0);
}

#[test]
fn bundle_initializer() {
    let mut app = App::default();

    let bundle = TrainBundle::br_218("BR 218 001", 123000.0);

    assert!(bundle.max_power.0 > 0.0);
    assert!(bundle.max_speed.0 > 0.0);
    assert!(bundle.mass.engine > 0.0);
    assert_eq!(bundle.mass.wagons, 123000.0);
    assert_eq!(bundle.name.0, "BR 218 001");

    app.world.spawn(bundle);

    let bundle = TrainBundle::default();

    assert_eq!(bundle.max_power.0, 0.0);
    assert_eq!(bundle.max_speed.0, 0.0);
    assert_eq!(bundle.mass.total(), 0.0);
    assert_eq!(bundle.name.0, "");

    app.world.spawn(bundle);
}

#[test]
fn plugin() {
    let mut app = App::default();
    app.add_plugins(TrainPlugins);
    assert!(app.is_plugin_added::<TrainRenderPlugin>());
    assert!(app.is_plugin_added::<TrainPhysicsPlugin>());
}

#[test]
fn paint_scheme_color() {
    let all = vec![
        PaintSchemeColor::Verkehrsrot,
        PaintSchemeColor::Orientrot,
        PaintSchemeColor::Lichtgrau,
        PaintSchemeColor::Fernblau,
        PaintSchemeColor::Ozeanblau,
        PaintSchemeColor::Minttuerkis,
        PaintSchemeColor::Pasteltuerkis,
        PaintSchemeColor::Lachsorange,
    ];

    for paint_scheme in all {
        // we just force the type here because if a hex value is invalid
        // the into while panic
        let _color: Color = paint_scheme.into();
    }
}

#[test]
fn paint_scheme() {
    let paint_scheme = PaintScheme::default();
    assert_eq!(paint_scheme.color, PaintSchemeColor::Verkehrsrot);
}
