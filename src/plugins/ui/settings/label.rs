use crate::plugins::{lidar::PlayerState, *};

use super::super::ui_plugin::UiState;
use bevy::prelude::*;
use bevy_egui::{*, egui::*};

#[derive(Resource)]
pub struct NewLabel {
    id: u16,
    info: LabelInfo,
}
impl Default for NewLabel{
    fn default() -> Self {
        Self { id: 0, info: LabelInfo{color: [255,255,255], name: default()} }
    }
}

pub fn init_new_label(mut new_label: ResMut<NewLabel>, config: Res<PlayerConfig>,) {
    new_label.id = match config.persistent.label_map.last_key_value(){
        Some((key, _)) => *key + 1,
        None => 0,
    };
}

pub fn window(
    mut egui_context: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut config: ResMut<PlayerConfig>,
    mut player: ResMut<PlayerState>,
    mut new_label: ResMut<NewLabel>,
) {
    let ctx = egui_context.ctx_mut();
    egui::Window::new("Label-Settings").open(&mut ui_state.color_settings_visible).resizable(true).vscroll(true).show(ctx, |ui| {
        let mut request_color_update = false;
        let mut request_save = false;
        let mut indexes_to_remove = Vec::new();
        let remove_button_color = Color32::from_rgb(60, 60, 60);
        egui::Grid::new("Test-Grid").striped(true).num_columns(3).show(ui, |ui| {
            ui.label("");
            if ui.color_edit_button_srgb(&mut config.persistent.default_color).changed() {
                request_color_update = true;
                request_save = true;
            }
            ui.label("Default");
            ui.end_row();
            for (label, info) in config.persistent.label_map.iter_mut() {
                ui.horizontal(|ui| {
                    if ui.button(RichText::new("✖").color(remove_button_color)).clicked() {
                        indexes_to_remove.push(*label);
                        request_color_update = true;
                        request_save = true;
                    }
                    ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                        ui.label(label.to_string());
                    });
                });
                if ui.color_edit_button_srgb(&mut info.color).changed() {
                    request_color_update = true;
                    request_save = true;
                }
                if ui.text_edit_singleline(&mut info.name).changed() {
                    request_save = true;
                }
                ui.end_row();
            }
            ui.horizontal(|ui| {
                if ui.button("➕").clicked() {
                    config.persistent.label_map.insert(new_label.id, new_label.info.clone());
                    new_label.id = match config.persistent.label_map.last_key_value(){
                        Some((key, _)) => *key + 1,
                        None => 0,
                    };
                    request_color_update = true;
                    request_save = true;
                }
                ui.add(egui::widgets::DragValue::new(&mut new_label.id));
            });
            if ui.color_edit_button_srgb(&mut new_label.info.color).changed() {
                request_color_update = true;
                request_save = true;
            }
            ui.text_edit_singleline(&mut new_label.info.name);
            ui.end_row();
        });
        ui.horizontal(|ui| {
            if ui.button(RichText::from("↺").heading()).on_hover_text("Reset all labels").clicked() {
                config.reset_label_map();
                player.request_update();
                request_save = true;
            }
        });
        for key in indexes_to_remove{
            config.persistent.label_map.remove(&key);
        }
        if request_color_update {
            config.update_label_map();
            player.request_update();
        }
        if request_save {
            config.save()
        }
    });
}
