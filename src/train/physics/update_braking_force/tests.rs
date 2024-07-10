use super::*;
use coverage_helper::test;

#[coverage(off)]
fn spawn_train(app: &mut App, brake: BrakeLever, mass: f32) -> Entity {
    app.world_mut()
        .spawn((ForceBraking::default(), Mass(mass), brake))
        .id()
}

#[test]
fn no_brake_no_force() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let train_id = spawn_train(&mut app, BrakeLever::default(), 7000.0);

    app.update();

    assert!(app.world().get::<ForceBraking>(train_id).is_some());
    assert_eq!(app.world().get::<ForceBraking>(train_id).unwrap().0, 0.0);
}

#[test]
fn brake_applies_force() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let train_id = spawn_train(&mut app, BrakeLever { percentage: 0.2 }, 7000.0);

    app.update();

    assert!(app.world().get::<ForceBraking>(train_id).is_some());
    assert!(app.world().get::<ForceBraking>(train_id).unwrap().0 > 0.0);
}

#[test]
fn more_brake_more_force() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let low_brake = spawn_train(&mut app, BrakeLever { percentage: 0.2 }, 7000.);

    let high_brake = spawn_train(&mut app, BrakeLever { percentage: 1.0 }, 7000.);

    app.update();

    assert!(app.world().get::<ForceBraking>(low_brake).is_some());
    assert!(app.world().get::<ForceBraking>(high_brake).is_some());
    assert!(
        app.world().get::<ForceBraking>(low_brake).unwrap().0
            < app.world().get::<ForceBraking>(high_brake).unwrap().0
    );
}

#[test]
fn more_weight_more_brake() {
    let mut app = App::new();
    app.add_systems(Update, system);

    let low_weight = spawn_train(&mut app, BrakeLever { percentage: 0.2 }, 7000.);

    let high_weight = spawn_train(&mut app, BrakeLever { percentage: 0.2 }, 70000.);

    app.update();

    assert!(app.world().get::<ForceBraking>(low_weight).is_some());
    assert!(app.world().get::<ForceBraking>(high_weight).is_some());
    assert!(
        app.world().get::<ForceBraking>(low_weight).unwrap().0
            < app.world().get::<ForceBraking>(high_weight).unwrap().0
    );
}
