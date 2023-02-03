use crate::plugins::{lidar::PlayerState, *};

use super::super::ui_plugin::UiState;
use bevy::prelude::*;
use bevy_egui::{*, egui::*};

pub fn window(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut config: ResMut<PlayerConfig>,
    mut player: ResMut<PlayerState>,
) {
    let ctx = egui_context.ctx_mut();
    egui::Window::new("Label-Settings")
        .open(&mut ui_state.color_settings_visible)
        .resizable(true)
        .default_size([10.0, 500.0])
        .show(ctx, |ui| {

            ui.horizontal(|ui| {
                //TODO: Remove save button make it auto-save
                if ui.button("Save").clicked() {
                    config.save();
                }
                //TODO: Change to reset icon with tooltip
                if ui.button("Reset").clicked() {
                    config.reset_label_map();
                    player.request_update();
                }
            });
            ui.separator();
            egui::Grid::new("Label")
                .striped(true)
                .num_columns(3)
                .spacing([10.0, 4.0])
                .show(ui, |ui| {
                    ui.label("Label");
                    ui.add_sized([200.0,20.0], egui::Label::new("Name"));
                    ui.label("Color");
            });
            ui.separator();
            ScrollArea::vertical().always_show_scroll(true).show(ui, |ui| {
                egui::Grid::new("Label")
                    .striped(true)
                    .num_columns(3)
                    .spacing([10.0, 4.0])
                    .show(ui, |ui| {
                        let mut request_color_update = false;
                        for (label, info) in config.persistent.label_map.iter_mut() {
                            ui.label(label.to_string());
                            ui.add_sized(
                                [200.0, 20.0],
                                egui::TextEdit::singleline(&mut info.name).desired_width(200.0),
                            );
                            if ui.color_edit_button_srgb(&mut info.color).changed() {
                                request_color_update = true;
                            }
                            ui.end_row();
                        }
                        if request_color_update {
                            config.update_label_map();
                            player.request_update();
                        }
                    });
            });
        });
}
