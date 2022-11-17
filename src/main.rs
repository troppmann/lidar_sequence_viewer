pub mod io;
pub mod math;
pub mod plugins;

use bevy::prelude::*;
use plugins::LidarPlugin;
use plugins::ObserverPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ObserverPlugin)
        .add_plugin(LidarPlugin)
        .run();
}
