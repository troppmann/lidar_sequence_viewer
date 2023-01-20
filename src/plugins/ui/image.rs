use bevy::prelude::*;

pub struct UiHandles{
    pub play_button: Handle<Image>,
    pub pause_button: Handle<Image>,
    pub fullscreen_enter_button: Handle<Image>,
    pub fullscreen_exit_button: Handle<Image>,
    pub next_frame_button: Handle<Image>,
    pub previous_frame_button: Handle<Image>,
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
        }
    }
}