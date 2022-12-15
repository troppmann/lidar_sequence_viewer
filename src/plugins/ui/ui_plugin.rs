use bevy::{prelude::*, tasks::{IoTaskPool, Task}};
use bevy_egui::{*, egui::{Vec2, Visuals, Color32, RichText, Key, style::Margin, Stroke}};
use futures_lite::future;
use rfd::{AsyncFileDialog, FileHandle};

use crate::{plugins::lidar::PlayerState, io};

use super::video_slider::VideoSlider;


pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(EguiPlugin)
            .add_startup_system(setup)
            .insert_resource(MenuState::default())
            .add_system(control_bar)
            .add_system(handle_load_folder_task)
            .add_system(menu_bar);
    }
}
struct UiHandles{
    play_button: Handle<Image>,
    pause_button: Handle<Image>,
    fullscreen_enter_button: Handle<Image>,
    fullscreen_exit_button: Handle<Image>,
    next_frame_button: Handle<Image>,
    previous_frame_button: Handle<Image>,
    folder_button: Handle<Image>,
}
#[derive(Default)]
struct ControlBarState{
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
            next_frame_button: asset_server.load("ui/textures/next_frame.png"),
            previous_frame_button: asset_server.load("ui/textures/previous_frame.png"),
            folder_button: asset_server.load("ui/textures/folder.png"),
        }
    }
}

fn setup(mut egui_context: ResMut<EguiContext>,){
    egui_context.ctx_mut().set_visuals(Visuals{
        dark_mode: false,
        ..default()
    });
}

#[derive(Component)]
pub struct LoadFolderTask(Task<Option<FileHandle>>);

#[derive(Resource, Default)]
pub struct MenuState{
    opened_folder_dialog: bool,
}

fn menu_bar(
    mut egui_context: ResMut<EguiContext>,
    images: Local<UiHandles>,
    mut commands: Commands,
    mut state: ResMut<MenuState>,
) {
    let ctx = egui_context.ctx_mut();
    let mut style: egui::Style = (*ctx.style()).clone();
    style.visuals.button_frame = false;
    let bg_color = Color32::from_rgb(40,40,40);
    style.visuals.widgets.inactive.bg_fill = bg_color;    
    style.visuals.widgets.hovered.bg_fill = bg_color;    
    style.visuals.widgets.active.bg_fill = bg_color;    
    let stroke = Stroke::new(1.0, Color32::GRAY);
    style.visuals.widgets.hovered.bg_stroke = stroke;    

    ctx.set_style(style);
    let frame = egui::Frame {fill: Color32::TRANSPARENT, inner_margin: Margin::same(10.0), ..egui::Frame::default() };
    let folder_button = egui_context.add_image(images.folder_button.clone_weak());
    egui::TopBottomPanel::top("menu bar").frame(frame).show(egui_context.ctx_mut(), |ui|{
        if ui.add_enabled(!state.opened_folder_dialog, egui::ImageButton::new(folder_button, Vec2::new(30.0, 30.0))).clicked() ||
            ui.input().key_pressed(Key::O) && !state.opened_folder_dialog {
            let task_pool = IoTaskPool::get();
            let task = task_pool.spawn(async move {
                AsyncFileDialog::new()
                        .set_directory("/")
                        .pick_folder()
                        .await
                
            });
            commands.spawn(LoadFolderTask(task));
            state.opened_folder_dialog = true;
        }
    });
}

fn handle_load_folder_task(
    mut commands: Commands,
    mut read_frame_tasks: Query<(Entity, &mut LoadFolderTask)>,
    mut menu_state: ResMut<MenuState>,
    mut player_state: ResMut<PlayerState>,
) {
    for (entity, mut task) in &mut read_frame_tasks {
        if let Some(folder_task) = future::block_on(future::poll_once(&mut task.0)) {
            if let Some(folder) = folder_task{
                match io::read_sequence_from_dir(folder.path().into()) {
                    Ok(sequence) => player_state.set_sequence(sequence),
                    Err(e) => println!("{}", e),
                }
            }
            menu_state.opened_folder_dialog = false;
            commands.entity(entity).despawn();
        }
    }
}

fn control_bar(
    mut egui_context: ResMut<EguiContext>,
    mut player: ResMut<PlayerState>, 
    mut windows: ResMut<Windows>,
    images: Local<UiHandles>,
    mut state: Local<ControlBarState>,
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
    let next_frame_button = egui_context.add_image(images.next_frame_button.clone_weak());
    let previous_frame_button = egui_context.add_image(images.previous_frame_button.clone_weak());
    egui::TopBottomPanel::bottom("ControlBar").frame(frame).show(egui_context.ctx_mut(), |ui| {
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
        });
        ui.horizontal(|ui| {
            ui.add_space(15.0);
            if ui.add(egui::ImageButton::new(previous_frame_button, Vec2::new(20.0, 20.0)).frame(false)).clicked() || ui.input().key_pressed(Key::ArrowLeft) {
                player.request_frame(frame.saturating_sub(1));
            }
            if ui.add(egui::ImageButton::new(play_button, Vec2::new(20.0, 20.0)).frame(false)).clicked() ||
            ui.input().key_pressed(Key::Space) {
                player.toggle_play();
            }
            if ui.add(egui::ImageButton::new(next_frame_button, Vec2::new(20.0, 20.0)).frame(false)).clicked() || ui.input().key_pressed(Key::ArrowRight){
                player.request_frame(frame + 1);
            }
            ui.add_sized(bevy_egui::egui::Vec2::new(40.0,20.0), 
            egui::Label::new(RichText::new(format!("{} / {}", frame, max_frame)).color(Color32::WHITE).text_style(egui::TextStyle::Button)));
            let padding = 35.0;
            ui.add_space(ui.available_width() - padding);
            if ui.add(egui::ImageButton::new(fullscreen_button, Vec2::new(20.0, 20.0)).frame(false)).clicked() ||
                ui.input().key_pressed(Key::F) {
                state.fullscreen = !state.fullscreen;
                let window = windows.primary_mut();
                window.set_mode(if state.fullscreen {WindowMode::BorderlessFullscreen} else {WindowMode::Windowed});
            }
        });
        ui.add_space(5.0);
    });
}
