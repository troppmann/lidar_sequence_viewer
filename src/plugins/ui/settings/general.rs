use bevy::prelude::*;
use bevy_egui::*;

use super::super::ui_plugin::UiState;
use crate::plugins::{config::PlayerConfig, lidar::PlayerState};

pub fn window(
    mut egui_context: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut config: ResMut<PlayerConfig>,
    mut player: ResMut<PlayerState>,
    mut clear_color: ResMut<ClearColor>,
    mut query_projection: Query<&mut Projection>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let ctx = egui_context.ctx_mut();
    egui::Window::new("General-Settings").open(&mut ui_state.general_settings_visible).resizable(true).vscroll(true).show(ctx, |ui| {
        egui::Grid::new("General-Grid").striped(true).num_columns(2).show(ui, |ui| {
            ui.label("Background Color");
            if ui.color_edit_button_srgb(&mut config.persistent.background_color).changed() {
                let color_rgb_u8 = config.persistent.background_color;
                clear_color.0 = Color::rgb_u8(color_rgb_u8[0], color_rgb_u8[1], color_rgb_u8[2]);
                config.save();
            }
            ui.end_row();
            ui.label("Default Label Color");
            if ui.color_edit_button_srgb(&mut config.persistent.default_color).changed() {
                config.update_label_map();
                player.request_update();
                config.save();
            }
            ui.end_row();
            ui.end_row();
            ui.label("Point Size");
            if ui.add(egui::DragValue::new(&mut config.persistent.point_size).clamp_range(0.0..=100.0).speed(0.01)).changed() {
                player.set_mesh(meshes.add(Mesh::from(shape::Cube { size: config.persistent.point_size })));
                player.request_update();
                config.save();
            }
            ui.end_row();
            ui.label("Sensor FPS");
            if ui.add(egui::DragValue::new(&mut config.persistent.sensor_fps).clamp_range(0.0..=10000.0).speed(1.0)).changed() {
                player.set_sensor_fps(config.persistent.sensor_fps);
                player.request_update();
                config.save();
            }
            ui.end_row();
            ui.end_row();
            ui.label("Camera Speed");
            if ui.add(egui::Slider::new(&mut config.persistent.camera_speed, 0.0..=100.0)).changed() {
                config.save();
            }
            ui.end_row();
            ui.label("Camera FOV");
            if ui.add(egui::Slider::new(&mut config.persistent.camera_fov_degreas, 0.0..=180.0)).changed() {
                for mut projection in query_projection.iter_mut(){
                    *projection = Projection::Perspective(PerspectiveProjection {
                        fov: config.persistent.camera_fov_degreas.to_radians(),
                        ..default()
                    });
                }
                config.save();
            }
            ui.end_row();
        });
    });
}
