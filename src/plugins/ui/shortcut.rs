use bevy::prelude::*;
use crate::plugins::lidar::*;
use super::ui_plugin::UiState;


pub fn handle_shortcuts(
    input: Res<Input<KeyCode>>,
    mut player: ResMut<PlayerState>, 
    mut ui_state: ResMut<UiState>,
){
    if input.just_pressed(KeyCode::F) || input.just_pressed(KeyCode::F12) {
        ui_state.fullscreen.request();
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
    if input.pressed(KeyCode::L) {
        ui_state.label_folder_dialog.request();
    }
}