use bevy::prelude::*;
use bevy_egui::EguiContext;
use crate::plugins::lidar::*;
use super::{ui_plugin::UiState, inspector::Inspector};


pub fn handle_shortcuts(
    input: Res<Input<KeyCode>>,
    mut player: ResMut<PlayerState>, 
    mut ui_state: ResMut<UiState>,
    mut inspector: ResMut<Inspector>,
    mut egui_ctx: ResMut<EguiContext>,
){
    if egui_ctx.ctx_mut().memory().focus().is_some() {
        return;
    }
    if input.just_pressed(KeyCode::F) || input.just_pressed(KeyCode::F12) {
        ui_state.fullscreen.request();
    }
    if input.just_pressed(KeyCode::I) {
        inspector.visible = !inspector.visible;
    }
    if input.just_pressed(KeyCode::Space) {
        player.toggle_play();
    }
    if input.pressed(KeyCode::Left) {
        player.previous_frame();      
    }
    if input.pressed(KeyCode::Right) {
        player.next_frame();      
    }
    if input.pressed(KeyCode::O) {
        ui_state.folder_dialog.request();
    }
    let shift = input.any_pressed([KeyCode::LShift, KeyCode::RShift]);
    if input.pressed(KeyCode::L) {
        match shift {
            true => player.discard_labels(),
            false => ui_state.label_folder_dialog.request(),
        }
    }
}