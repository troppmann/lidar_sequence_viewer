pub mod io;
pub mod math;
pub mod plugins;

use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use plugins::*;

fn main() {
    App::new()
        .add_startup_system(setup_window)
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
