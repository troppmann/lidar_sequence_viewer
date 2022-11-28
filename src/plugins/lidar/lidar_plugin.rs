use std::path::Path;

use bevy::{prelude::*, render::view::NoFrustumCulling, tasks::{Task, IoTaskPool}};
use futures_lite::future;

use crate::io::*;

use super::instancing::*;

pub struct LidarPlugin;

impl Plugin for LidarPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerState::default())
            .add_startup_system(init_sequence)
            .add_system(player)
            .add_system(handle_read_frames_task)
            .add_system(buffer_next_frames)
            .add_plugin(InstancingPlugin);
    }
}
#[derive(Resource)]
pub struct PlayerState {
    start_time: Option<f64>,
    pub sequence: Option<Sequence>,
    mesh: Option<Handle<Mesh>>,
    is_loading: bool,
    actual_frame: usize,
    start_frame: usize,
    last_rendered_frame: usize, 
    paused: bool,
}


impl Default for PlayerState {
    fn default() -> Self {
        Self {
            start_time: None,
            sequence: None,
            mesh: None,
            is_loading: false,
            actual_frame: 0,
            start_frame: 0,
            last_rendered_frame: usize::MAX,
            paused: true,
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
    pub fn is_paused(&self)->bool{
        self.paused
    }
    pub fn get_frame(&self) -> usize {
        self.actual_frame
    }
    pub fn request_frame(&mut self, frame: usize){
        self.start_frame = frame;
        self.actual_frame = frame;
        self.start_time = None;
    }
    pub fn toggle_play(&mut self){
        self.paused = !self.paused;
        self.start_frame = self.actual_frame;
        self.start_time = None;
    }
    pub fn play(&mut self){
        self.paused = false;
        self.start_frame = self.actual_frame;
        self.start_time = None;
    }
    pub fn pause(&mut self){
        self.paused = true;
    }
}

fn init_sequence(
    mut state: ResMut<PlayerState>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let path = String::from("../SemanticKITTI/dataset/sequences/00/velodyne/");
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
    if !state.paused {
        state.update(time.elapsed_seconds_f64());
    }
    if state.last_rendered_frame != state.actual_frame {
        if let Some(sequence) = &state.sequence {
            if let Some(frame) = &sequence.frames[state.actual_frame] {
                //change frame content
                query.for_each(|entity| commands.entity(entity).despawn());
                spawn_frame(
                    &mut commands,
                    frame,
                    state.mesh.as_ref().unwrap().clone(),
                );
                state.last_rendered_frame = state.actual_frame;
                state.is_loading = false;
            } else {
                state.is_loading = true;
            }
        }
    }
    
}

#[derive(Component)]
struct ReadFrameTask {
    task: Task<Frame>,
    frame_number: usize,
}

fn buffer_next_frames(mut commands: Commands, mut state: ResMut<PlayerState>){
    let thread_pool = IoTaskPool::get();
    let actual_frame = state.actual_frame;
    if let Some(sequence) = &mut state.sequence{
        sequence.load_state.iter_mut().enumerate()
        .skip(actual_frame).take(30)
        .for_each(|(iter, state)|{
            if *state == LoadState::NotRequested {
                let  path = format!("{}/{:0>6}.bin", sequence.folder, iter);
                let task = thread_pool.spawn(async move {
                    read_frame(Path::new(&path)).unwrap()
                });
                commands.spawn(ReadFrameTask{task, frame_number: iter});
                *state = LoadState::Requested;
            }
        });

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


fn handle_read_frames_task(mut commands:Commands, 
                    mut read_frame_tasks: Query<(Entity, &mut ReadFrameTask)>,
                    mut state: ResMut<PlayerState>){
    for (entity, mut task) in &mut read_frame_tasks {
        let max_frame = state.actual_frame + 30;
        if !(state.actual_frame..max_frame).contains(&task.frame_number) {
            commands.entity(entity).despawn();
            if let Some(sequence) = &mut state.sequence {
                sequence.load_state[task.frame_number] = LoadState::NotRequested;
            }
            continue;
        }
        if let Some(frame) = future::block_on(future::poll_once(&mut task.task)) {
            if let Some(sequence) = &mut state.sequence {
                sequence.frames[task.frame_number] = Some(frame);
                sequence.load_state[task.frame_number] = LoadState::Loaded;
            }
            commands.entity(entity).despawn();
        }
    }
}