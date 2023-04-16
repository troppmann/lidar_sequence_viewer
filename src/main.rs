#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod io;
pub mod math;
pub mod plugins;

use std::io::Cursor;

use bevy::{prelude::*, winit::WinitWindows, window::PrimaryWindow};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use plugins::*;
use winit::window::Icon;


fn main() {
    App::new()
        .add_startup_systems((setup_window, set_window_icon))
        .add_plugins(
            DefaultPlugins
                .build()
                .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin),
        )
        .add_plugin(ConfigPlugin)
        .add_plugin(LidarPlugin)
        .add_plugin(FpsWindowTitlePlugin)
        .add_plugin(ObserverPlugin)
        .add_plugin(UiPlugin)
        .run();
}

fn setup_window(mut query_window: Query<&mut Window>) {
    let mut window = query_window.single_mut();
    window.title = "Lidar Viewer".to_string();
    window.position = WindowPosition::Centered(MonitorSelection::Current);
}

fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let primary = windows.get_window(primary_entity).unwrap();
    let icon_buf = Cursor::new(include_bytes!(
        "../assets/logo/logo32.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}