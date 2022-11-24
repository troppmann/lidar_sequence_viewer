use bevy::prelude::*;
use bevy_egui::{*, egui::{Vec2, Visuals, Color32, RichText}};

use crate::plugins::lidar::PlayerState;

use super::video_slider::VideoSlider;


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
#[derive(Default)]
struct UiState{
    paused_state_before_drag: bool,
    dragging: bool,
    fullscreen: bool,
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
    mut windows: ResMut<Windows>,
    images: Local<UiHandles>,
    mut state: Local<UiState>,
) {
    let frame = egui::Frame {fill: Color32::from_rgba_premultiplied(10, 10, 10, 200), ..egui::Frame::default() };
    let show_play_button = state.dragging && state.paused_state_before_drag || !state.dragging && player.is_paused(); 
    let play_button = egui_context.add_image(match show_play_button {
        true => images.play_button.clone_weak(),
        false => images.pause_button.clone_weak(),
    });
    let fullscreen_button = egui_context.add_image(match state.fullscreen {
        true => images.fullscreen_exit_button.clone_weak(),
        false => images.fullscreen_enter_button.clone_weak(),
    });
    egui::TopBottomPanel::bottom("ControlBar").frame(frame).show(egui_context.ctx_mut(), |ui| {
        ui.add_space(10.0);
        let mut frame = player.get_frame();
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            if ui.add(egui::ImageButton::new(play_button, Vec2::new(20.0, 20.0)).frame(false)).clicked() {
                player.toggle_play();
            }
            let label_space = 85.0; 
            ui.spacing_mut().slider_width = ui.available_width() - label_space;
            let max_frame = player.sequence.as_ref().map_or(0, |s| s.frame_count - 1).max(0);
            let slider_response = ui.add(VideoSlider::new(&mut frame, 0..=max_frame).slider_color(Color32::from_rgb(250, 11, 11)).show_value(false));
            if slider_response.drag_started(){
                state.paused_state_before_drag = player.is_paused();
                state.dragging = true;
                player.pause()
            }
            if slider_response.drag_released(){
                if !state.paused_state_before_drag {
                    player.play();
                }
                state.dragging = false;
            }
            if slider_response.changed(){
                player.request_frame(frame);
            }
            ui.add_sized(bevy_egui::egui::Vec2::new(40.0,20.0), 
                        egui::Label::new(RichText::new(frame.to_string()).color(Color32::WHITE).text_style(egui::TextStyle::Button)));
            if ui.add(egui::ImageButton::new(fullscreen_button, Vec2::new(20.0, 20.0)).frame(false)).clicked() {
                state.fullscreen = !state.fullscreen;
                let window = windows.primary_mut();
                window.set_mode(if state.fullscreen {WindowMode::BorderlessFullscreen} else {WindowMode::Windowed});
            }
        });
        ui.add_space(5.0);
    });
}
