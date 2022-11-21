use bevy::prelude::*;
use bevy_egui::{*, egui::{Vec2, Visuals, Color32, RichText}};

use super::lidar::PlayerState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin).add_startup_system(setup).add_system(control_bar);
        // app.insert_resource(PlayerState::default());
    }
}
struct UiHandles{
    play_button: Handle<Image>,
    pause_button: Handle<Image>,
    fullscreen_enter_button: Handle<Image>,
    fullscreen_exit_button: Handle<Image>,
}


impl FromWorld for UiHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        Self {
            play_button: asset_server.load("ui/textures/play.png"),
            pause_button: asset_server.load("ui/textures/pause.png"),
            fullscreen_enter_button: asset_server.load("ui/textures/fullscreen_enter.png"),
            fullscreen_exit_button: asset_server.load("ui/textures/fullscreen_exit.png"),
        }
    }
}

fn setup(mut egui_context: ResMut<EguiContext>,){
    egui_context.ctx_mut().set_visuals(Visuals{
        dark_mode: true,
        ..default()
    });
}


fn control_bar(
    mut egui_context: ResMut<EguiContext>,
    mut player: ResMut<PlayerState>, 
    time: Res<Time>, 
    mut windows: ResMut<Windows>,
    images: Local<UiHandles>,
) {
    let frame = egui::Frame {fill: Color32::from_rgba_premultiplied(10, 10, 10, 200), ..egui::Frame::default() };
    let play_button = egui_context.add_image(match player.paused {
        true => images.play_button.clone_weak(),
        false => images.pause_button.clone_weak(),
    });
    let fullscreen_button = egui_context.add_image(match player.fullscreen {
        true => images.fullscreen_exit_button.clone_weak(),
        false => images.fullscreen_enter_button.clone_weak(),
    });
    egui::TopBottomPanel::bottom("ControlBar").frame(frame).show(egui_context.ctx_mut(), |ui| {
        let elapsed = time.elapsed_seconds_f64();
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            if ui.add(egui::ImageButton::new(play_button, Vec2::new(20.0, 20.0)).frame(false)).clicked() {
                player.toggle_play(elapsed)
            }
            let label_space = 85.0; 
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
            ui.add_sized(bevy_egui::egui::Vec2::new(40.0,20.0), 
                        egui::Label::new(RichText::new(player.actual_frame.to_string()).color(Color32::WHITE).text_style(egui::TextStyle::Button)));
            if ui.add(egui::ImageButton::new(fullscreen_button, Vec2::new(20.0, 20.0)).frame(false)).clicked() {
                player.fullscreen = !player.fullscreen;
                let window = windows.primary_mut();
                window.set_mode(if player.fullscreen {WindowMode::BorderlessFullscreen} else {WindowMode::Windowed});
            }
        });
        ui.add_space(5.0);
    });
}
