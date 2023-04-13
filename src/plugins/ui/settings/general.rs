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
            ui.label("Camera Speed");
            if ui.add(egui::Slider::new(&mut config.persistent.camera_speed, 0.0..=100.0)).changed() {
                config.save();
            }
        });
    });
}
