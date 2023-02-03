use crate::plugins::{lidar::PlayerState, *};

use super::super::ui_plugin::UiState;
use bevy::prelude::*;
use bevy_egui::{*, egui::*};

pub struct NewLabel {
    init: bool,
    id: u32,
    info: LabelInfo,
}
impl Default for NewLabel{
    fn default() -> Self {
        Self { init: false, id: 0, info: LabelInfo{color: [255,255,255], name: default()} }
    }
}

pub fn window(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut config: ResMut<PlayerConfig>,
    mut player: ResMut<PlayerState>,
    mut new_label: Local<NewLabel>,
) {
    let ctx = egui_context.ctx_mut();
    egui::Window::new("Label-Settings").open(&mut ui_state.color_settings_visible).resizable(true).vscroll(true).show(ctx, |ui| {
        let mut request_color_update = false;
        let mut indexes_to_remove = Vec::new();
        let remove_button_color = Color32::from_rgb(60, 60, 60);
        egui::Grid::new("Test-Grid").striped(true).num_columns(3).show(ui, |ui| {
            for (label, info) in config.persistent.label_map.iter_mut() {
                ui.horizontal(|ui| {
                    if ui.colored_label(remove_button_color, "✖").clicked() {
                        indexes_to_remove.push(*label);
                        request_color_update = true;
                    }
                    ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                        ui.label(label.to_string());
                    });
                });
                if ui.color_edit_button_srgb(&mut info.color).changed() {
                    request_color_update = true;
                }
                ui.text_edit_singleline(&mut info.name);
                ui.end_row();
            }
            //TODO move to startup system
            if !new_label.init {
                new_label.id = match config.persistent.label_map.last_key_value(){
                    Some((key, _)) => *key + 1,
                    None => 0,
                };
                new_label.init = true;
            }
            ui.horizontal(|ui| {
                if ui.button("➕").clicked() {
                    config.persistent.label_map.insert(new_label.id, new_label.info.clone());
                    request_color_update = true;
                    new_label.id = match config.persistent.label_map.last_key_value(){
                        Some((key, _)) => *key + 1,
                        None => 0,
                    };
                }
                ui.add(egui::widgets::DragValue::new(&mut new_label.id));
            });
            if ui.color_edit_button_srgb(&mut new_label.info.color).changed() {
                request_color_update = true;
            }
            ui.text_edit_singleline(&mut new_label.info.name);
            ui.end_row();
        });
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
        for key in indexes_to_remove{
            config.persistent.label_map.remove(&key);
        }
        if request_color_update {
            config.update_label_map();
            player.request_update();
        }
    });
}
