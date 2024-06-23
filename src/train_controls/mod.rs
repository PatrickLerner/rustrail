#[cfg(test)]
mod tests;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{
    camera,
    train::{Acceleration, BrakeLever, Distance, Mass, Name, Speed, ThrottleLever},
};

type TrainControlQuery<'a> = (
    Entity,
    &'a Name,
    &'a Speed,
    &'a Acceleration,
    &'a Distance,
    &'a Mass,
    &'a mut ThrottleLever,
    &'a mut BrakeLever,
    &'a Transform,
);

fn train_controls(
    mut entries: Query<TrainControlQuery>,
    mut contexts: EguiContexts,
    mut camera: Query<&mut camera::GameCameraState>,
) {
    for (
        entity,
        name,
        speed,
        acceleration,
        distance,
        mass,
        mut throttle_lever,
        mut brake_lever,
        transform,
    ) in entries.iter_mut()
    {
        egui::Window::new(format!("Train: {}", name.0))
            .id(egui::Id::new(entity))
            .show(contexts.ctx_mut(), |ui| {
                ui.label(format!("speed {:.1} km/h", speed.0 * 3.6,));
                ui.label(format!("acceleration {:.3} m/s^2", acceleration.0));
                ui.label(format!("distance {:.0} m", distance.0));
                ui.label(format!("mass {:.0} t", mass.total() / 1000.0));
                ui.label(format!("pos {:?}", transform.translation));

                ui.add(
                    egui::Slider::new(&mut throttle_lever.percentage, 0.0..=1.0)
                        .text("ThrottleLever"),
                );

                ui.add(
                    egui::Slider::new(&mut brake_lever.percentage, 0.0..=1.0).text("BrakeLever"),
                );

                let can_change_direction = speed.0 < 0.1 && speed.0 > -0.1;
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
            });
    }
}

pub struct TrainControlsPlugin;

impl Plugin for TrainControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, train_controls);
    }
}
