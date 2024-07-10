use super::*;
use coverage_helper::test;

#[test]
fn plugin() {
    let mut app = App::default();
    app.add_plugins(UIPlugins);
    assert!(app.is_plugin_added::<UIPlugin>());
    assert!(app.is_plugin_added::<train_controls::TrainControlsPlugin>());
    assert!(app.is_plugin_added::<train_spawn::TrainSpawnPlugin>());
}

#[test]
fn performance_monitoring() {
    let mut app = App::default();
    assert_eq!(app.world().entities().len(), 0);

    app.add_systems(Update, setup_performance_monitoring);
    app.update();

    assert_eq!(app.world().entities().len(), 1);
}

#[test]
fn test_wireframe_mode() {
    let mut app = App::new();

    let mut inputs: ButtonInput<KeyCode> = ButtonInput::default();
    app.insert_resource(inputs.clone());

    app.add_systems(Update, wireframe_mode);

    app.init_resource::<WireframeConfig>();

    {
        app.update();
        let config = app.world().get_resource::<WireframeConfig>().unwrap();
        assert!(!config.global);
    }

    {
        inputs.press(KeyCode::KeyG);
        app.insert_resource(inputs.clone());
        app.update();

        let config = app.world().get_resource::<WireframeConfig>().unwrap();
        assert!(!config.global);
    }

    {
        inputs.release(KeyCode::KeyG);
        app.insert_resource(inputs.clone());

        app.update();

        let config = app.world().get_resource::<WireframeConfig>().unwrap();
        assert!(config.global);
    }

    {
        inputs.press(KeyCode::KeyG);
        app.insert_resource(inputs.clone());
        app.update();

        let config = app.world().get_resource::<WireframeConfig>().unwrap();
        assert!(!config.global);
    }
}
