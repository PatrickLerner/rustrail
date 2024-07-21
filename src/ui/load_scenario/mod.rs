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
    mut scenarios: Local<Option<Vec<(String, String)>>>,
) {
    if scenarios.is_none() {
        let files = glob("assets/scenarios/*.toml").unwrap();
        let items = files
            .map(
                #[coverage(off)]
                |file| {
                    let file_name = file.unwrap().display().to_string();
                    let scenario = ScenarioData::load_from_file(&file_name);

                    (file_name, scenario.info.name)
                },
            )
            .collect();

        *scenarios = Some(items);
    }

    let scenarios = scenarios.as_ref().unwrap();

    egui::Window::new("Load Scenario")
        .anchor(egui::Align2::CENTER_CENTER, (0.0, 0.0))
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .show(
            contexts.ctx_mut(),
            #[coverage(off)]
            |ui| {
                for (file_name, scenario_name) in scenarios {
                    if ui.small_button(scenario_name).clicked() {
                        let scenario_data = ScenarioData::load_from_file(file_name);

                        let mut window = window.single_mut();
                        window.title = format!("rustrail - {}", scenario_data.info.name);

                        commands.insert_resource(scenario_data);
                    }
                }
            },
        );
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
