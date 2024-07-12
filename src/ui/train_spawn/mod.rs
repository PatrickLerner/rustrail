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

            if ui
                .small_button("BR 111 (double engine, with wagons)")
                .clicked()
            {
                let mut components = vec![];

                for _ in 0..2 {
                    *id += 1;
                    let engine = commands
                        .spawn(EngineBundle::from_file("assets/models/BR111.toml"))
                        .insert(Name(format!("BR 111 {:0>3}", id.to_string())))
                        .id();
                    components.push(TrainComponent::Engine(engine));
                }

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

            if ui.small_button("BR 111 (with passenger wagons)").clicked() {
                *id += 1;
                let engine = commands
                    .spawn(EngineBundle::from_file("assets/models/BR111.toml"))
                    .insert(Name(format!("BR 111 {:0>3}", id.to_string())))
                    .id();

                let mut components = vec![TrainComponent::Engine(engine)];

                for _ in 0..3 {
                    components.push(TrainComponent::Wagon(
                        commands
                            .spawn(WagonBundle::from_file("assets/models/nwagen.toml"))
                            .id(),
                    ));
                }

                commands.spawn(TrainBundle {
                    name: Name("RB 61".to_string()),
                    composition: TrainComposition { components },
                    ..default()
                });
            }

            if ui.small_button("BR 147 (with passenger wagons)").clicked() {
                *id += 1;
                let engine = commands
                    .spawn(EngineBundle::from_file("assets/models/BR147.toml"))
                    .insert(Name(format!("BR 147 {:0>3}", id.to_string())))
                    .id();

                let mut components = vec![TrainComponent::Engine(engine)];

                for _ in 0..3 {
                    components.push(TrainComponent::Wagon(
                        commands
                            .spawn(WagonBundle::from_file("assets/models/nwagen.toml"))
                            .id(),
                    ));
                }

                commands.spawn(TrainBundle {
                    name: Name("RB 61".to_string()),
                    composition: TrainComposition { components },
                    ..default()
                });
            }

            if ui.small_button("BR 186 (with passenger wagons)").clicked() {
                *id += 1;
                let engine = commands
                    .spawn(EngineBundle::from_file("assets/models/BR186.toml"))
                    .insert(Name(format!("BR 186 {:0>3}", id.to_string())))
                    .id();

                let mut components = vec![TrainComponent::Engine(engine)];

                for _ in 0..3 {
                    components.push(TrainComponent::Wagon(
                        commands
                            .spawn(WagonBundle::from_file("assets/models/nwagen.toml"))
                            .id(),
                    ));
                }

                commands.spawn(TrainBundle {
                    name: Name("RB 61".to_string()),
                    composition: TrainComposition { components },
                    ..default()
                });
            }

            if ui.small_button("BR 52 (with passenger wagons)").clicked() {
                *id += 1;
                let engine = commands
                    .spawn(EngineBundle::from_file("assets/models/BR52.toml"))
                    .insert(Name(format!("BR 52 {:0>3}", id.to_string())))
                    .id();

                let tender = commands
                    .spawn(WagonBundle::from_file("assets/models/BR52_tender.toml"))
                    .id();

                let mut components = vec![
                    TrainComponent::Engine(engine),
                    TrainComponent::Wagon(tender),
                ];

                for _ in 0..3 {
                    components.push(TrainComponent::Wagon(
                        commands
                            .spawn(WagonBundle::from_file("assets/models/nwagen.toml"))
                            .id(),
                    ));
                }

                commands.spawn(TrainBundle {
                    name: Name("RB 61".to_string()),
                    composition: TrainComposition { components },
                    ..default()
                });
            }

            if ui
                .small_button("Baden VI c (with passenger wagons)")
                .clicked()
            {
                *id += 1;
                let engine = commands
                    .spawn(EngineBundle::from_file("assets/models/BadenVIc.toml"))
                    .insert(Name(format!("BR 75 {:0>3}", id.to_string())))
                    .id();

                let mut components = vec![TrainComponent::Engine(engine)];

                for _ in 0..3 {
                    components.push(TrainComponent::Wagon(
                        commands
                            .spawn(WagonBundle::from_file("assets/models/nwagen.toml"))
                            .id(),
                    ));
                }

                commands.spawn(TrainBundle {
                    name: Name("RB 61".to_string()),
                    composition: TrainComposition { components },
                    ..default()
                });
            }

            if ui
                .small_button("SBB EE922 (with passenger wagons)")
                .clicked()
            {
                *id += 1;
                let engine = commands
                    .spawn(EngineBundle::from_file("assets/models/sbb_ee922.toml"))
                    .insert(Name(format!("SBB EE922 {:0>3}", id.to_string())))
                    .id();

                let mut components = vec![TrainComponent::Engine(engine)];

                for _ in 0..3 {
                    components.push(TrainComponent::Wagon(
                        commands
                            .spawn(WagonBundle::from_file("assets/models/nwagen.toml"))
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
