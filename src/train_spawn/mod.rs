#[cfg(test)]
mod tests;

use crate::train::{EngineBundle, Name, TrainBundle, TrainComponent, TrainComposition};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

#[coverage(off)]
fn train_controls(mut commands: Commands, mut contexts: EguiContexts) {
    egui::Window::new("Debug: Spawn train").show(
        contexts.ctx_mut(),
        #[coverage(off)]
        |ui| {
            if ui.small_button("BR 218 (single engine)").clicked() {
                let engine = commands.spawn(EngineBundle::br_218("BR 218 001")).id();

                commands.spawn(TrainBundle {
                    name: Name("RB 61".to_string()),
                    composition: TrainComposition {
                        components: vec![TrainComponent::Engine(engine)],
                    },
                    ..default()
                });
            }

            if ui.small_button("E10 / BR 110 (single engine)").clicked() {
                let engine = commands.spawn(EngineBundle::br_110("BR 110 001")).id();

                commands.spawn(TrainBundle {
                    name: Name("RB 61".to_string()),
                    composition: TrainComposition {
                        components: vec![TrainComponent::Engine(engine)],
                    },
                    ..default()
                });
            }

            if ui.small_button("BR 89 (single engine)").clicked() {
                let engine = commands.spawn(EngineBundle::br_89("BR 89 001")).id();

                commands.spawn(TrainBundle {
                    name: Name("RB 61".to_string()),
                    composition: TrainComposition {
                        components: vec![TrainComponent::Engine(engine)],
                    },
                    ..default()
                });
            }

            if ui.small_button("ICE 1 (no wagons)").clicked() {
                let engine_front = commands.spawn(EngineBundle::ice("ICE 1 (Front)")).id();
                let engine_rear = commands.spawn(EngineBundle::ice("ICE 1 (Rear)")).id();

                commands.spawn(TrainBundle {
                    name: Name("RB 61".to_string()),
                    composition: TrainComposition {
                        components: vec![
                            TrainComponent::Engine(engine_front),
                            TrainComponent::Engine(engine_rear),
                        ],
                    },
                    ..default()
                });
            }
        },
    );
}

pub struct TrainSpawnPlugin;

impl Plugin for TrainSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, train_controls);
    }
}
