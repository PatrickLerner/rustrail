#[cfg(test)]
mod tests;

use crate::scenario::ScenarioData;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui, EguiContexts};
use glob::glob;

#[coverage(off)]
fn load_scenario(
    mut contexts: EguiContexts,
    mut commands: Commands,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let files = glob("assets/scenarios/*.toml").unwrap();

    egui::Window::new("Load Scenario").show(contexts.ctx_mut(), |ui| {
        for file in files {
            let file_name = file.unwrap().display().to_string();

            if ui.small_button(&file_name).clicked() {
                let scenario_data = ScenarioData::load_from_file(&file_name);

                let mut window = window.single_mut();
                window.title = format!("rustrail - {}", scenario_data.info.name);

                commands.insert_resource(scenario_data);
            }
        }
    });
}

pub struct LoadScenarioPlugin;

impl Plugin for LoadScenarioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            load_scenario.run_if(not(resource_exists::<ScenarioData>)),
        );
    }
}
