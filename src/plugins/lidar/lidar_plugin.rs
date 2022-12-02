use std::path::Path;

use bevy::{
    prelude::*,
    render::view::NoFrustumCulling,
    tasks::{IoTaskPool, Task},
};
use futures_lite::future;

use crate::io::*;

use super::instancing::*;

pub struct LidarPlugin;

impl Plugin for LidarPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerState::default())
            .add_startup_system(init_sequence)
            .add_system(player)
            .add_system(buffer_next_frames)
            .add_system(handle_read_frames_task)
            .add_plugin(InstancingPlugin);
    }
}
#[derive(Resource)]
pub struct PlayerState {
    start_time: Option<f64>,
    sequence: Option<Sequence>,
    mesh: Option<Handle<Mesh>>,
    wait_for_buffering: bool,
    actual_frame: usize,
    buffer_frame: usize,
    max_frame: usize,
    start_frame: usize,
    last_rendered_frame: usize,
    has_frame_request: bool,
    paused: bool,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            start_time: None,
            sequence: None,
            mesh: None,
            wait_for_buffering: false,
            actual_frame: 0,
            start_frame: 0,
            buffer_frame: 0,
            max_frame: 0,
            last_rendered_frame: usize::MAX,
            has_frame_request: false,
            paused: true,
        }
    }
}

impl PlayerState {
    const MINIMUM_BUFFERED_FRAMES: usize = 30;
    const MAX_BUFFER_RANGE: usize = 300;
    const BUFFER_SLIDING_WINDOW: usize = 5;
    const MEMORY_RANGE: usize = 500;
    const SENSOR_FRAMES_PER_SECONDS: f64 = 10.0;
    pub fn is_paused(&self) -> bool {
        self.paused
    }
    pub fn get_frame(&self) -> usize {
        self.actual_frame
    }
    pub fn get_buffer_frame(&self) -> usize {
        self.buffer_frame
    }
    pub fn get_max_frame(&self) -> usize{
        self.max_frame
    }
    pub fn set_sequence(&mut self, sequence: Sequence){
        self.max_frame = sequence.frame_count -1;
        self.sequence = Some(sequence);
        self.start_frame = 0;
        self.actual_frame = 0;
        self.buffer_frame = 0;
    }
    pub fn request_frame(&mut self, frame: usize) {
        self.has_frame_request = true;
        self.start_frame = frame;
        self.actual_frame = frame;
        self.buffer_frame = frame;
        self.start_time = None;
        self.free_memory_after_request();
    }
    pub fn toggle_play(&mut self) {
        self.paused = !self.paused;
        self.start_frame = self.actual_frame;
        self.start_time = None;
    }
    pub fn play(&mut self) {
        self.paused = false;
        self.start_frame = self.actual_frame;
        self.start_time = None;
    }
    pub fn pause(&mut self) {
        self.paused = true;
    }
    fn update(&mut self, time_in_seconds: f64) {
        let passed_time = time_in_seconds - *self.start_time.get_or_insert(time_in_seconds);
        self.actual_frame = ((passed_time * PlayerState::SENSOR_FRAMES_PER_SECONDS) as usize + self.start_frame)
            .min(self.max_frame)
            .max(0);
    }
    fn free_memory_after_request(&mut self){
        if let Some(sequence) = &mut self.sequence {
            let min = self.actual_frame.saturating_sub(PlayerState::MEMORY_RANGE);
            let max = self.actual_frame.saturating_add(PlayerState::MEMORY_RANGE);
            let memory_range = min..max;
            sequence.frames.iter_mut().zip(&mut sequence.load_states).enumerate()
                .filter(|(iter, _)|!memory_range.contains(iter))
                .for_each(|(_, (frame, load_state))| {
                    *frame = None;
                    *load_state = LoadState::NotRequested;
            });
        }
    } 
    fn free_memory_after_frame_update(&mut self){
        if let Some(sequence) = &mut self.sequence {
            let frame_to_delete = self.actual_frame.saturating_sub(PlayerState::MEMORY_RANGE);
            if frame_to_delete != 0 {
                    sequence.frames[frame_to_delete] = None;
                    sequence.load_states[frame_to_delete] = LoadState::NotRequested;
            }
        }
    }
}

fn init_sequence(mut state: ResMut<PlayerState>, mut meshes: ResMut<Assets<Mesh>>) {
    let path = String::from("../SemanticKITTI/dataset/sequences/00/velodyne/");
    let sequence = read_sequence_from_dir(path).unwrap();
    state.set_sequence(sequence);
    state.mesh = Some(meshes.add(Mesh::from(shape::Cube { size: 0.04 })))
}

fn player(
    mut commands: Commands,
    time: Res<Time>,
    mut state: ResMut<PlayerState>,
    query: Query<Entity, With<InstanceMaterialData>>,
) {
    if !state.paused && !state.wait_for_buffering{
        state.update(time.elapsed_seconds_f64());
    }
    if state.wait_for_buffering {
        state.wait_for_buffering = state.buffer_frame < usize::min(state.actual_frame + PlayerState::MINIMUM_BUFFERED_FRAMES,state.max_frame); 
    }
    if state.last_rendered_frame != state.actual_frame {
        if let Some(sequence) = &state.sequence {
            if let Some(frame) = &sequence.frames[state.actual_frame] {
                //change frame content
                query.for_each(|entity| commands.entity(entity).despawn());
                spawn_frame(&mut commands, frame, state.mesh.as_ref().unwrap().clone());
                state.last_rendered_frame = state.actual_frame;
            } else {
                state.wait_for_buffering = true;
                state.start_time = None;
            }
        }
        state.free_memory_after_frame_update();
    }
}

#[derive(Component)]
struct ReadFrameTask {
    task: Task<Frame>,
    frame_number: usize,
}

fn buffer_next_frames(mut commands: Commands, mut state: ResMut<PlayerState>) {
    let thread_pool = IoTaskPool::get();
    let mut buffer_frame = state.buffer_frame;
    let max_buffer_frame = usize::min(state.actual_frame + PlayerState::MAX_BUFFER_RANGE, state.max_frame);
    if buffer_frame == max_buffer_frame {
        return;
    }
    if let Some(sequence) = &mut state.sequence {
        sequence
            .load_states.iter_mut().enumerate()
            .skip(buffer_frame).skip_while(|(iter, load_state)| {
                let skip = **load_state == LoadState::Loaded && *iter < max_buffer_frame;
                if skip {
                    buffer_frame = *iter;
                }
                skip
            })
            .take(PlayerState::BUFFER_SLIDING_WINDOW)
            .for_each(|(iter, state)| {
                if *state == LoadState::NotRequested {
                    let path = format!("{}/{:0>6}.bin", sequence.folder, iter);
                    let task =
                        thread_pool.spawn(async move { read_frame(Path::new(&path)).unwrap() });
                    commands.spawn(ReadFrameTask {
                        task,
                        frame_number: iter,
                    });
                    *state = LoadState::Requested;
                }
            });
        state.buffer_frame = buffer_frame;
    }
}

fn handle_read_frames_task(
    mut commands: Commands,
    mut read_frame_tasks: Query<(Entity, &mut ReadFrameTask)>,
    mut state: ResMut<PlayerState>,
) {
    let frame_request = state.has_frame_request;
    if let Some(sequence) = &mut state.sequence {
        for (entity, mut task) in &mut read_frame_tasks {
            if let Some(frame) = future::block_on(future::poll_once(&mut task.task)) {
                sequence.frames[task.frame_number] = Some(frame);
                sequence.load_states[task.frame_number] = LoadState::Loaded; 
                commands.entity(entity).despawn();
            } else if frame_request {
                commands.entity(entity).despawn();
                sequence.load_states[task.frame_number] = LoadState::NotRequested; 
            }
        }
    }
    if frame_request {
        state.has_frame_request = false;
        state.buffer_frame = state.actual_frame;
    }
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