use bevy::prelude::*;

pub struct LidarPlugin;

impl Plugin for LidarPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(hello);
    }
}

fn hello() {
    println!("Hello World")
}
