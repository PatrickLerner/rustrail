#[cfg(test)]
mod tests;

mod train_controls;
mod train_spawn;

use bevy::{app::PluginGroupBuilder, pbr::wireframe::WireframeConfig, prelude::*};
use bevy_egui::{egui, EguiContexts};
use iyes_perf_ui::prelude::*;

#[coverage(off)]
fn color_mode(mut contexts: EguiContexts, mut commands: Commands) {
    // commands.spawn(Camera2dBundle::default());
    commands.insert_resource(ClearColor(Color::srgb(
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

fn setup_performance_monitoring(mut commands: Commands) {
    commands.spawn((
        PerfUiRoot::default(),
        PerfUiEntryFPS::default(),
        PerfUiEntryEntityCount::default(),
    ));
}

struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (color_mode, setup_performance_monitoring))
            .add_systems(Update, wireframe_mode);
    }
}

pub struct UIPlugins;

impl PluginGroup for UIPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add(bevy::diagnostic::EntityCountDiagnosticsPlugin)
            .add(PerfUiPlugin)
            .add(UIPlugin)
            .add(train_controls::TrainControlsPlugin)
            .add(train_spawn::TrainSpawnPlugin)
    }
}
