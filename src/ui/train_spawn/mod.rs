#[cfg(test)]
mod tests;

use crate::train::{
    EngineBundle, Name, TrainBundle, TrainComponent, TrainComposition, WagonBundle,
};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

#[coverage(off)]
fn spawn(mut commands: Commands, mut contexts: EguiContexts, mut id: Local<i64>) {
    egui::Window::new("Debug: Spawn train").show(
        contexts.ctx_mut(),
        #[coverage(off)]
        |ui| {
            if ui.small_button("BR 111 (single engine)").clicked() {
                *id += 1;
                let engine = commands
                    .spawn(EngineBundle::from_file("assets/models/BR111.toml"))
                    .insert(Name(format!("BR 111 {:0>3}", id.to_string())))
                    .id();

                commands.spawn(TrainBundle {
                    name: Name("RB 61".to_string()),
                    composition: TrainComposition {
                        components: vec![TrainComponent::Engine(engine)],
                    },
                    ..default()
                });
            }

            if ui.small_button("BR 111 (with wagons)").clicked() {
                *id += 1;
                let engine = commands
                    .spawn(EngineBundle::from_file("assets/models/BR111.toml"))
                    .insert(Name(format!("BR 111 {:0>3}", id.to_string())))
                    .id();

                let mut components = vec![TrainComponent::Engine(engine)];

                for _ in 0..25 {
                    components.push(TrainComponent::Wagon(
                        commands
                            .spawn(WagonBundle::from_file("assets/models/eanos.toml"))
                            .id(),
                    ));
                }

                commands.spawn(TrainBundle {
                    name: Name("RB 61".to_string()),
                    composition: TrainComposition { components },
                    ..default()
                });
            }
        },
    );
}

pub struct TrainSpawnPlugin;

impl Plugin for TrainSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn);
    }
}
