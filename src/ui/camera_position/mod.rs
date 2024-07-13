#[cfg(test)]
mod tests;

use crate::{camera::GameCameraState, landscape::OriginOffset};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use proj::Proj;

#[derive(Default, Resource)]
struct UiState {
    lat: String,
    lng: String,
}

#[coverage(off)]
fn system(
    mut contexts: EguiContexts,
    mut camera_state: Query<&mut GameCameraState>,
    mut state: Local<UiState>,
    origin_offset: Res<OriginOffset>,
) {
    egui::Window::new("Debug: Camera Position").show(
        contexts.ctx_mut(),
        #[coverage(off)]
        |ui| {
            ui.label("Lat");
            ui.text_edit_singleline(&mut state.lat);
            ui.label("Lng");
            ui.text_edit_singleline(&mut state.lng);

            if ui.small_button("Set Munich").clicked() {
                state.lat = "48.1386763686898".to_owned();
                state.lng = "11.575296120439964".to_owned();
            }

            if ui.small_button("Set Bensheim").clicked() {
                state.lat = "49.68134809269307".to_owned();
                state.lng = "8.61687829630227".to_owned();
            }

            if ui.small_button("Teleport").clicked() {
                if let Ok(lat) = state.lat.parse::<f64>() {
                    if let Ok(lng) = state.lng.parse::<f64>() {
                        let converter =
                            Proj::new_known_crs("EPSG:4326", "ESRI:53004", None).unwrap();
                        let result = converter.convert((lng, lat));
                        let (x, y) = result.unwrap();
                        let mut state = camera_state.single_mut();
                        state.center.x = (x - origin_offset.0 .0) as f32;
                        state.center.z = -(y - origin_offset.0 .1) as f32;
                    }
                }
            }
        },
    );
}

pub struct CameraPositionPlugin;

impl Plugin for CameraPositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, system);
    }
}
