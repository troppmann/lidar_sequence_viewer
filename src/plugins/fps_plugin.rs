use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct FpsWindowTitlePlugin;

impl Plugin for FpsWindowTitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_system(update_window_title);
    }
}

pub fn update_window_title(diagnostics: Res<Diagnostics>, mut query_window: Query<&mut Window>) {
    let mut window = query_window.single_mut();
    if let Some(fps) = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.average())
    {
        window.title = format!("Lidar Viewer FPS {:.2}", fps);
    }
}
