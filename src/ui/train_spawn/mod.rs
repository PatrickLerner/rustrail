#[cfg(test)]
mod tests;

use crate::train::{
    Dimension, EngineBundle, Mass, MaxSpeed, Name, PaintScheme, PaintSchemeColor, TrainBundle,
    TrainComponent, TrainComposition, WagonBundle,
};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

#[coverage(off)]
fn train_controls(mut commands: Commands, mut contexts: EguiContexts) {
    egui::Window::new("Debug: Spawn train").show(
        contexts.ctx_mut(),
        #[coverage(off)]
        |ui| {
            if ui.small_button("BR 218 (single engine)").clicked() {
                let components = vec![TrainComponent::Engine(
                    commands.spawn(EngineBundle::br_218("BR 218 001")).id(),
                )];

                commands.spawn(TrainBundle {
                    name: Name("RB 61".to_string()),
                    composition: TrainComposition { components },
                    ..default()
                });
            }

            if ui.small_button("BR 218 (with wagons)").clicked() {
                let mut components = vec![TrainComponent::Engine(
                    commands.spawn(EngineBundle::br_218("BR 218 001")).id(),
                )];

                for _ in 0..25 {
                    components.push(TrainComponent::Wagon(
                        commands
                            .spawn(WagonBundle {
                                paint_scheme: PaintScheme {
                                    color: PaintSchemeColor::Tiefschwarz,
                                },
                                mass: Mass(23_000.0),
                                dimension: Dimension { length: 16.0 },
                                // TODO: realistic
                                max_speed: MaxSpeed(140.0),
                                ..default()
                            })
                            .id(),
                    ));
                }

                commands.spawn(TrainBundle {
                    name: Name("RB 61".to_string()),
                    composition: TrainComposition { components },
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

            if ui.small_button("ICE 1").clicked() {
                let mut components = vec![TrainComponent::Engine(
                    commands.spawn(EngineBundle::ice("ICE 1 (Front)")).id(),
                )];

                for _ in 0..25 {
                    components.push(TrainComponent::Wagon(
                        commands
                            .spawn(WagonBundle {
                                paint_scheme: PaintScheme {
                                    color: PaintSchemeColor::Lichtgrau,
                                },
                                mass: Mass(52_000.0),
                                dimension: Dimension { length: 26.43 },
                                // TODO: realistic
                                max_speed: MaxSpeed(280.0 / 3.6),
                                ..default()
                            })
                            .id(),
                    ));
                }

                components.push(TrainComponent::Engine(
                    commands.spawn(EngineBundle::ice("ICE 1 (Rear)")).id(),
                ));

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
        app.add_systems(Update, train_controls);
    }
}
