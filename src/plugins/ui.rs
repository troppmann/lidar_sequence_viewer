use bevy::prelude::*;
use bevy_egui::*;

use super::lidar::PlayerState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin).add_system(control_bar);
    }
}

fn control_bar(
    mut egui_context: ResMut<EguiContext>,
    mut player: ResMut<PlayerState>, 
    time: Res<Time>, 
    mut windows: ResMut<Windows>
) {
    egui::TopBottomPanel::bottom("Playbar").show(egui_context.ctx_mut(), |ui| {
        let elapsed = time.elapsed_seconds_f64();
        ui.horizontal(|ui| {
            if ui.add_sized(bevy_egui::egui::Vec2::new(100.0,20.0), egui::Button::new(if player.paused {"Play"} else {"Paused"})).clicked() {
                player.toggle_play(elapsed)
            }
            let label_space = 150.0; 
            ui.spacing_mut().slider_width = ui.available_width() - label_space;
            let max_frame = player.sequence.as_ref().map_or(0, |s| s.frame_count - 1).max(0);
            let slider_response = ui.add(egui::Slider::new(&mut player.actual_frame, 0..=max_frame).show_value(false));
            if slider_response.drag_started(){
                player.drag_paused = true;
            }
            if slider_response.drag_released(){
                player.drag_paused = false;
                player.start_time = elapsed;
            }
            if slider_response.changed(){
                let frame = player.actual_frame;
                player.request_frame(frame, time.elapsed_seconds_f64());
            }
            ui.add_sized(bevy_egui::egui::Vec2::new(40.0,20.0), egui::Label::new(player.actual_frame.to_string()));
            if ui.add_sized(bevy_egui::egui::Vec2::new(100.0,20.0), egui::Button::new(if player.fullscreen {"Window"} else {"Fullscreen"})).clicked() {
                player.fullscreen = !player.fullscreen;
                let window = windows.primary_mut();
                window.set_mode(if player.fullscreen {WindowMode::BorderlessFullscreen} else {WindowMode::Windowed});
            }
        });
    });
}
