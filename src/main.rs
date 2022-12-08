pub mod io;
pub mod math;
pub mod plugins;

use bevy::prelude::*;
use plugins::*;

fn main() {
    App::new()
        .add_startup_system(setup_window)
        .add_plugins(DefaultPlugins)
        .add_plugin(FpsWindowTitlePlugin)
        .add_plugin(ObserverPlugin)
        .add_plugin(LidarPlugin)
        .add_plugin(UiPlugin)
        .run();
}

fn setup_window(mut windows: ResMut<Windows>){
    let window = windows.primary_mut();
    window.set_title("Lidar Viewer".to_string());
    window.center_window(MonitorSelection::Current);
    window.set_decorations(false);
}

