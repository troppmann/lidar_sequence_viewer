use bevy::prelude::*;
use bevy_egui::{*, egui::{Vec2, Color32, RichText, Key, style::Margin, Stroke, epaint::Shadow, KeyboardShortcut, Modifiers}};

use crate::plugins::lidar;
use super::{request::*, shortcut, task, video_slider::*, image::*};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(EguiPlugin)
            .add_startup_system(setup)
            .insert_resource(UiState::default())
            .add_system(task::handle_load_folder_task)
            .add_system(control_bar.before(handle_requests))
            .add_system(shortcut::handle_shortcuts.before(handle_requests))
            .add_system(menu_bar.before(handle_requests))
            .add_system(handle_requests);
    }
}


fn setup(mut egui_context: ResMut<EguiContext>,){
    let ctx = egui_context.ctx_mut();
    let mut style: egui::Style = (*ctx.style()).clone();
    style.visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
    let selected_color  =  Color32::from_rgb(60,60,60);
    style.visuals.widgets.hovered.bg_fill = selected_color;
    style.visuals.widgets.active.bg_fill = selected_color;
    style.visuals.widgets.hovered.bg_stroke = Stroke::NONE;
    style.visuals.widgets.active.bg_stroke = Stroke::NONE;
    style.visuals.window_stroke = Stroke::NONE;
    style.visuals.window_fill = Color32::from_rgb(20, 20, 20);
    style.visuals.popup_shadow = Shadow::NONE;
    style.visuals.override_text_color = Some(Color32::from_rgb(240,240,240));
    ctx.set_style(style);
}
#[derive(Resource, Default)]
pub struct UiState{
    pub folder_dialog: DialogRequest,
    pub label_folder_dialog: DialogRequest,
    pub fullscreen: ToggleRequest,
}
fn menu_bar(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut player_state: ResMut<lidar::PlayerState>,
) {
    let ctx = egui_context.ctx_mut();
    let frame = egui::Frame {
        fill: Color32::from_rgba_premultiplied(10, 10, 10, 200),
        inner_margin: Margin{left: 6.0, top: 2.0, bottom: 2.0, ..Margin::default()}, 
        ..egui::Frame::default() 
    };
    egui::TopBottomPanel::top("menu bar").show_separator_line(false).frame(frame).show(ctx, |ui|{
        ui.horizontal(|ui| {
            ui.menu_button("File", |ui| {
                if ui.add_enabled(!ui_state.folder_dialog.is_open(), egui::Button::new("Open Sequence Folder...").shortcut_text("O").wrap(false)).clicked() {
                    ui_state.folder_dialog.request(); 
                }
                ui.separator();
                if ui.button("Exit").clicked() {

                }        
            });

            ui.menu_button("View", |ui| {
                if ui.add(egui::Button::new("Fullscreen").shortcut_text("F").wrap(false)).clicked() {
                    ui_state.fullscreen.request();   
                }
            });
            ui.menu_button("Playback", |ui| {
                //TODO replace button new with image_and_text
                if ui.add(egui::Button::new(if player_state.is_paused() {"Play"} else {"Pause"}).shortcut_text("Space").wrap(false)).clicked() {
                    player_state.toggle_play();
                }
            });
            ui.menu_button("Label", |ui| {
                if ui.add_enabled(!ui_state.label_folder_dialog.is_open(), egui::Button::new("Open Label Folder...").shortcut_text("L").wrap(false)).clicked() {
                    ui_state.label_folder_dialog.request(); 
                }
                if ui.add(egui::Button::new("Discard").shortcut_text(ctx.format_shortcut(&KeyboardShortcut{key: Key::L, modifiers: Modifiers::SHIFT})).wrap(false)).clicked() {
                    player_state.discard_labels();
                }
            })
        });
    });
}

#[derive(Default)]
struct ControlBarState{
    paused_state_before_drag: bool,
    dragging: bool,
}
fn control_bar(
    mut egui_context: ResMut<EguiContext>,
    mut player: ResMut<lidar::PlayerState>, 
    images: Local<UiHandles>,
    mut control_bar_state: Local<ControlBarState>,
    mut ui_state: ResMut<UiState>,
) {
    let frame = egui::Frame {fill: Color32::from_rgba_premultiplied(10, 10, 10, 200), ..egui::Frame::default() };
    let show_play_button = control_bar_state.dragging && control_bar_state.paused_state_before_drag || !control_bar_state.dragging && player.is_paused(); 
    let play_button = egui_context.add_image(match show_play_button {
        true => images.play_button.clone_weak(),
        false => images.pause_button.clone_weak(),
    });
    let fullscreen_button = egui_context.add_image(match ui_state.fullscreen.get_state() {
        true => images.fullscreen_exit_button.clone_weak(),
        false => images.fullscreen_enter_button.clone_weak(),
    });
    let next_frame_button = egui_context.add_image(images.next_frame_button.clone_weak());
    let previous_frame_button = egui_context.add_image(images.previous_frame_button.clone_weak());
    egui::TopBottomPanel::bottom("ControlBar").show_separator_line(false).frame(frame).show(egui_context.ctx_mut(), |ui| {
        let mut frame = player.get_frame();
        let max_frame = player.get_max_frame();
        ui.horizontal(|ui| {
            let padding = 10.0; 
            ui.add_space(padding);
            ui.spacing_mut().slider_width = ui.available_width() - padding;
            let slider_response = ui.add(VideoSlider::new(&mut frame, 0..=max_frame)
                .buffer_value(&player.get_buffer_frame())
                .buffer_hint_color(Color32::from_rgb(111, 111, 111))
                .slider_color(Color32::from_rgb(250, 11, 11))
                .show_value(false));
            if slider_response.drag_started(){
                control_bar_state.paused_state_before_drag = player.is_paused();
                control_bar_state.dragging = true;
                player.pause()
            }
            if slider_response.drag_released(){
                if !control_bar_state.paused_state_before_drag {
                    player.play();
                }
                control_bar_state.dragging = false;
            }
            if slider_response.changed(){
                player.request_frame(frame);
            }
        });
        ui.horizontal(|ui| {
            let button_size = Vec2::new(20.0, 20.0);
            ui.add_space(15.0);
            if ui.add(egui::ImageButton::new(previous_frame_button, button_size).frame(false)).clicked() || ui.input().key_pressed(Key::ArrowLeft) {
                player.previous_frame();
            }
            if ui.add(egui::ImageButton::new(play_button, button_size).frame(false)).clicked() {
                player.toggle_play();
            }
            if ui.add(egui::ImageButton::new(next_frame_button, button_size).frame(false)).clicked() {
                player.next_frame();
            }
            ui.add_sized(bevy_egui::egui::Vec2::new(40.0,20.0), 
            egui::Label::new(RichText::new(format!("{} / {}", frame, max_frame)).color(Color32::WHITE).text_style(egui::TextStyle::Button)));
            let padding = 35.0;
            ui.add_space(ui.available_width() - padding);
            if ui.add(egui::ImageButton::new(fullscreen_button, button_size).frame(false)).clicked() {
                ui_state.fullscreen.request();
            }
        });
        ui.add_space(5.0);
    });
}

fn handle_requests(
    mut ui_state: ResMut<UiState>,
    mut windows: ResMut<Windows>,
    mut commands: Commands,
){
    ui_state.fullscreen.on_request(|state| {
        let window = windows.primary_mut();
        let mode = match state {
            true => WindowMode::BorderlessFullscreen, 
            false => WindowMode::Windowed,
        };
        window.set_mode(mode);
    });
    ui_state.folder_dialog.on_request(|| {
        task::spawn_load_folder_task(&mut commands, task::FolderTaskType::Seqeunce);
    });
    ui_state.label_folder_dialog.on_request(|| {
        task::spawn_load_folder_task(&mut commands, task::FolderTaskType::Label);
    });
}