use super::super::ui_plugin::UiState;
use bevy::prelude::*;
use bevy_egui::{
    egui::*,
    *,
};

pub fn window(mut egui_context: ResMut<EguiContext>, mut ui_state: ResMut<UiState>) {
    let ctx = egui_context.ctx_mut();
    egui::Window::new("Color Settings")
        .open(&mut ui_state.color_settings_visible)
        .show(ctx, |ui| {
            ui.label("Hello World!");
        });
}
