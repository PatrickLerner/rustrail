use super::*;
use coverage_helper::test;

#[test]
fn plugin() {
    let mut app = App::default();
    app.add_plugins(UIPlugin);
    assert!(app.is_plugin_added::<UIPlugin>());
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
        let config = app.world.get_resource::<WireframeConfig>().unwrap();
        assert!(!config.global);
    }

    {
        inputs.press(KeyCode::KeyG);
        app.insert_resource(inputs.clone());
        app.update();

        let config = app.world.get_resource::<WireframeConfig>().unwrap();
        assert!(!config.global);
    }

    {
        inputs.release(KeyCode::KeyG);
        app.insert_resource(inputs.clone());

        app.update();

        let config = app.world.get_resource::<WireframeConfig>().unwrap();
        assert!(config.global);
    }

    {
        inputs.press(KeyCode::KeyG);
        app.insert_resource(inputs.clone());
        app.update();

        let config = app.world.get_resource::<WireframeConfig>().unwrap();
        assert!(!config.global);
    }
}
