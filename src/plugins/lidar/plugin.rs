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
    start_time: Option<f64>,
    pub sequence: Option<Sequence>,
    mesh: Option<Handle<Mesh>>,
    actual_frame: usize,
    start_frame: usize,
    last_rendered_frame: usize, 
    drag_paused: bool,
    pub paused: bool,
    pub fullscreen: bool,
}


impl Default for PlayerState {
    fn default() -> Self {
        Self {
            start_time: None,
            sequence: None,
            mesh: None,
            actual_frame: 0,
            start_frame: 0,
            last_rendered_frame: usize::MAX,
            drag_paused: false,
            paused: true,
            fullscreen: false,
        }
    }
}

impl PlayerState {
    fn update(&mut self, time_in_seconds: f64){
        if let Some(sequence) = &self.sequence{
            let passed_time = time_in_seconds - *self.start_time.get_or_insert(time_in_seconds);
            self.actual_frame = ((passed_time * 10.0) as usize + self.start_frame).min(sequence.frame_count - 1).max(0);
        }
    }
    pub fn get_frame(&self) -> usize {
        self.actual_frame
    }
    pub fn toggle_play(&mut self){
        self.paused = !self.paused;
        self.start_frame = self.actual_frame;
        self.start_time = None;
    }
    pub fn request_frame(&mut self, frame: usize){
        self.start_frame = frame;
        self.actual_frame = frame;
        self.start_time = None;
    }
    pub fn drag_start(&mut self){
        self.drag_paused = true;
    }
    pub fn drag_end(&mut self){
        self.drag_paused = false;
        self.start_time = None;
    }

}


fn init_sequence(
    mut state: ResMut<PlayerState>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let path = Path::new("../SemanticKITTI/dataset/sequences/04/velodyne/");
    state.sequence = Some(read_sequence_from_dir(path).unwrap());
    state.start_time = None;
    state.start_frame = 0;
    state.mesh = Some(meshes.add(Mesh::from(shape::Cube { size: 0.04 })))
}

fn player(
    mut commands: Commands,
    time: Res<Time>,
    mut state: ResMut<PlayerState>,
    query: Query<Entity, With<InstanceMaterialData>>,
) {
    if !state.paused && !state.drag_paused{
        state.update(time.elapsed_seconds_f64());
    }
    if state.last_rendered_frame != state.actual_frame {
        query.for_each(|entity| commands.entity(entity).despawn());
        if let Some(sequence) = &state.sequence {
            spawn_frame(
                &mut commands,
                &sequence.frames[state.actual_frame],
                state.mesh.as_ref().unwrap().clone(),
            )
        }
        state.last_rendered_frame = state.actual_frame;
    }
    
}

fn spawn_frame(commands: &mut Commands, frame: &Frame, mesh: Handle<Mesh>) {
    commands.spawn((
        mesh,
        SpatialBundle::VISIBLE_IDENTITY,
        InstanceMaterialData(
            frame.0.iter().map(|point| InstanceData {
                    position: point.position,
                    scale: 1.0,
                    color: Color::rgb_u8(247, 127, 0).as_linear_rgba_f32(),
                }).collect(),
        ),
        NoFrustumCulling,
    ));
}