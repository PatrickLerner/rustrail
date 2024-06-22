use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{
    camera,
    train::{Acceleration, BrakeLever, Distance, Mass, Name, Speed, ThrottleLever},
};

fn train_controls(
    mut entries: Query<(
        Entity,
        &Name,
        &Speed,
        &Acceleration,
        &Distance,
        &Mass,
        &mut ThrottleLever,
        &mut BrakeLever,
        &Transform,
    )>,
    mut contexts: EguiContexts,
    mut camera: Query<&mut camera::PanOrbitState>,
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

                ui.add(
                    egui::Slider::new(&mut throttle_lever.percentage, 0.0..=1.0)
                        .text("ThrottleLever"),
                );

                ui.add(
                    egui::Slider::new(&mut brake_lever.percentage, 0.0..=1.0).text("BrakeLever"),
                );

                if ui
                    .small_button(format!("{:?}", throttle_lever.direction))
                    .clicked()
                {
                    if speed.0 < 0.1 && speed.0 > -0.1 {
                        throttle_lever.direction = throttle_lever.direction.opposite();
                    }
                }

                if ui.small_button("Focus").clicked() {
                    let mut camera = camera.single_mut();
                    camera.center.x = transform.translation.x;
                    camera.center.z = transform.translation.z;
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
