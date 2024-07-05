#[cfg(test)]
mod tests;

mod train_controls;
mod train_spawn;

use bevy::{app::PluginGroupBuilder, pbr::wireframe::WireframeConfig, prelude::*};
use bevy_egui::{egui, EguiContexts};

#[coverage(off)]
fn color_mode(mut contexts: EguiContexts, mut commands: Commands) {
    // commands.spawn(Camera2dBundle::default());
    commands.insert_resource(ClearColor(Color::rgb(
        230.0 / 255.0,
        230.0 / 255.0,
        230.0 / 255.0,
    )));
    contexts.ctx_mut().set_visuals(egui::Visuals::light());
}

fn wireframe_mode(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut wireframe: ResMut<WireframeConfig>,
) {
    if keyboard_input.just_released(KeyCode::KeyG) {
        wireframe.global = !wireframe.global;
    }
}

struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, color_mode)
            .add_systems(Update, wireframe_mode);
    }
}

pub struct UIPlugins;

impl PluginGroup for UIPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(UIPlugin)
            .add(train_controls::TrainControlsPlugin)
            .add(train_spawn::TrainSpawnPlugin)
    }
}
