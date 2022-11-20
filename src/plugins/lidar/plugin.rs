use std::path::Path;

use bevy::{prelude::*, render::view::NoFrustumCulling};

use crate::io::*;

use super::instancing::*;

pub struct LidarPlugin;

impl Plugin for LidarPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerState::default())
            .add_startup_system(init_sequence)
            .add_system(player)
            .add_plugin(InstancingPlugin);
    }
}
#[derive(Resource)]
pub struct PlayerState {
    start_time: f64,
    pub sequence: Option<Sequence>,
    mesh: Option<Handle<Mesh>>,
    pub actual_frame: usize,
    pub start_frame: usize, 
    has_frame_changes: bool,
    pub paused: bool,
    pub fullscreen: bool,
}


impl Default for PlayerState {
    fn default() -> Self {
        Self {
            start_time: 0.0,
            sequence: None,
            mesh: None,
            actual_frame: 0,
            start_frame: 0,
            has_frame_changes: false,
            paused: true,
            fullscreen: false,
        }
    }
}

impl PlayerState {
    pub fn update(&mut self, time_in_seconds: f64){
        if let Some(sequence) = &self.sequence{
            let passed_time = time_in_seconds - self.start_time;
            let old_index = self.actual_frame; 
            self.actual_frame = ((passed_time * 10.0) as usize + self.start_frame).min(sequence.frame_count - 1).max(0);
            self.has_frame_changes = old_index != self.actual_frame;
        }
    }
    pub fn toggle_play(&mut self, time_in_seconds: f64){
        self.paused = !self.paused;
        println!("{}", self.paused);
        self.start_frame = self.actual_frame;
        self.start_time = time_in_seconds;
    }
    pub fn request_frame(&mut self, frame: usize, time_in_seconds: f64){
        self.start_frame = frame;
        self.start_time = time_in_seconds;
        self.has_frame_changes = true;
    }
}


fn init_sequence(
    mut state: ResMut<PlayerState>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let path = Path::new("../SemanticKITTI/dataset/sequences/04/velodyne/");
    state.sequence = Some(read_sequence_from_dir(path).unwrap());
    state.start_time = time.elapsed_seconds_f64();
    state.start_frame = 0;
    state.has_frame_changes = true;
    state.mesh = Some(meshes.add(Mesh::from(shape::Cube { size: 0.04 })))
}

fn spawn_frame(commands: &mut Commands, frame: &Frame, mesh: Handle<Mesh>) {
    commands.spawn((
        mesh,
        SpatialBundle::VISIBLE_IDENTITY,
        InstanceMaterialData(
            frame
                .0
                .iter()
                .map(|point| InstanceData {
                    position: point.position,
                    scale: 1.0,
                    color: Color::rgb_u8(247, 127, 0).as_linear_rgba_f32(),
                })
                .collect(),
        ),
        NoFrustumCulling,
    ));
}

fn player(
    mut commands: Commands,
    time: Res<Time>,
    mut state: ResMut<PlayerState>,
    query: Query<Entity, With<InstanceMaterialData>>,
) {
    if !state.paused{
        state.update(time.elapsed_seconds_f64());
    }
    
    if state.has_frame_changes {
        query.for_each(|entity| commands.entity(entity).despawn());
        if let Some(sequence) = &state.sequence {
            spawn_frame(
                &mut commands,
                &sequence.frames[state.actual_frame],
                state.mesh.as_ref().unwrap().clone(),
            )
        }
        state.has_frame_changes = false;
    }
}
