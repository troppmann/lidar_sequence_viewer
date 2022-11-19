use bevy::prelude::*;
use bevy_egui::*;

use super::lidar::PlayerState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin).add_system(setup);
    }
}

fn setup(mut egui_context: ResMut<EguiContext>,mut player: ResMut<PlayerState>) {
    egui::TopBottomPanel::bottom("Playbar").show(egui_context.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            if ui.button(if player.paused {"Paused"} else {"Play"}).clicked(){
                player.paused = !player.paused;
            }
            let label_space = 50.0; 
            ui.spacing_mut().slider_width = ui.available_width() - label_space;
            let max_frame = player.sequence.as_ref().map_or(0, |s| s.frame_count);
            if ui.add(egui::Slider::new(&mut player.actual_frame, 0..=max_frame).show_value(false)).changed(){
                println!("Hello Wolrd");
            }
            ui.label(player.actual_frame.to_string());
        });
    });
}
