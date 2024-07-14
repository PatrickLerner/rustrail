#[cfg(test)]
mod tests;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{
    camera,
    train::{AirPressure, BrakeLever, Mass, Name, Speed, ThrottleLever},
};

type TrainControlQuery<'a> = (
    Entity,
    &'a Name,
    &'a Speed,
    &'a Mass,
    &'a AirPressure,
    &'a mut ThrottleLever,
    &'a mut BrakeLever,
);

const MAX_SPEED_WHEN_REVERSING: f32 = 8.0 /* km/h */ / 3.6;

#[coverage(off)]
fn train_controls(
    mut selected_engine: Local<Option<Entity>>,
    mut trains: Query<TrainControlQuery>,
    mut contexts: EguiContexts,
    mut camera: Query<&mut camera::GameCameraState>,
) {
    if trains.is_empty() {
        return;
    }

    if selected_engine.is_none() {
        *selected_engine = Some(trains.iter().next().unwrap().0);
    }

    if let Some(entity) = *selected_engine {
        let trains_count = trains.iter().len();

        let mut options = vec![];
        for train in trains.iter() {
            options.push((train.0, train.1 .0.to_owned()));
        }

        if let Ok((entity, name, speed, mass, air_pressure, mut throttle_lever, mut brake_lever)) =
            trains.get_mut(entity)
        {
            egui::TopBottomPanel::bottom("info").show(
                contexts.ctx_mut(),
                #[coverage(off)]
                |ui| {
                    ui.horizontal(
                        #[coverage(off)]
                        |ui| {
                            if trains_count > 1 {
                                egui::ComboBox::from_label("")
                                    .selected_text(&name.0)
                                    .show_ui(
                                        ui,
                                        #[coverage(off)]
                                        |ui| {
                                            for (entity, name) in options.into_iter() {
                                                ui.selectable_value(
                                                    &mut selected_engine as &mut Option<Entity>,
                                                    Some(entity),
                                                    name,
                                                );
                                            }
                                        },
                                    );
                            }

                            ui.label(format!("{:.2} km/h", speed.as_kmh()));
                            ui.separator();
                            ui.label(format!(
                                "Throttle: {:.0}%",
                                throttle_lever.percentage * 100.0
                            ));
                            ui.add(
                                egui::Slider::new(&mut throttle_lever.percentage, 0.0..=1.0)
                                    .show_value(false),
                            );
                            ui.separator();
                            ui.label(format!("Brake: {:.0}%", brake_lever.percentage * 100.0));
                            ui.add(
                                egui::Slider::new(&mut brake_lever.percentage, 0.0..=1.0)
                                    .show_value(false),
                            );
                            ui.separator();
                            ui.label(format!("{:.2} t", mass.0 / 1000.0));
                            ui.separator();
                            ui.label(format!("{:.2} bar", air_pressure.0));
                            ui.separator();
                            let can_change_direction = speed.0.abs() < MAX_SPEED_WHEN_REVERSING
                                && throttle_lever.percentage == 0.0;
                            if ui
                                .small_button(format!("{:?}", throttle_lever.direction))
                                .clicked()
                                && can_change_direction
                            {
                                throttle_lever.direction = throttle_lever.direction.opposite();
                            }

                            if ui.small_button("Follow").clicked() {
                                let mut camera = camera.single_mut();
                                camera.follow = Some(entity);
                            }
                        },
                    );
                },
            );
        } else {
            *selected_engine = None;
        }
    }
}

pub struct TrainControlsPlugin;

impl Plugin for TrainControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, train_controls);
    }
}
