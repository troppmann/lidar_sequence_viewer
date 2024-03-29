use std::path::PathBuf;

use bevy::{
    prelude::*,
    render::view::NoFrustumCulling,
    tasks::{IoTaskPool, Task},
    utils::HashMap,
};
use futures_lite::future;

use crate::{
    io::{self, *},
    plugins::PlayerConfig,
};

use super::instancing::*;

pub struct LidarPlugin;

impl Plugin for LidarPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerState::default())
            .add_startup_system(load_config)
            .add_system(player)
            .add_system(buffer_next_frames)
            .add_system(handle_read_frames_task)
            .add_plugin(InstancingPlugin);
    }
}
struct SpeedSettings {
    sensor_fps: f64,
    time_scale: f64,
}

#[derive(Resource)]
pub struct PlayerState {
    start_time: Option<f64>,
    sequence: Option<Sequence>,
    sequence_number: u32,
    mesh: Option<Handle<Mesh>>,
    wait_for_buffering: bool,
    actual_frame: usize,
    buffer_frame: usize,
    max_frame: usize,
    start_frame: usize,
    last_rendered_frame: usize,
    has_frame_request: bool,
    paused: bool,
    speed: SpeedSettings,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            start_time: None,
            sequence: None,
            sequence_number: 0,
            mesh: None,
            wait_for_buffering: false,
            actual_frame: 0,
            start_frame: 0,
            buffer_frame: 0,
            max_frame: 0,
            last_rendered_frame: usize::MAX,
            has_frame_request: false,
            paused: true,
            speed: SpeedSettings {
                sensor_fps: 10.0,
                time_scale: 1.0,
            },
        }
    }
}

impl PlayerState {
    const MINIMUM_BUFFERED_FRAMES: usize = 30;
    const MAX_BUFFER_RANGE: usize = 300;
    const BUFFER_SLIDING_WINDOW: usize = 5;
    const MEMORY_RANGE: usize = 500;
    pub fn is_paused(&self) -> bool {
        self.paused
    }
    pub fn get_frame(&self) -> usize {
        self.actual_frame
    }
    pub fn get_frame_content(&self) -> Option<&Frame> {
        Some(self.sequence.as_ref()?.frames[self.actual_frame].as_ref()?)
    }
    pub fn get_buffer_frame(&self) -> usize {
        self.buffer_frame
    }
    pub fn get_max_frame(&self) -> usize {
        self.max_frame
    }
    pub fn try_set_labels(&mut self, path: PathBuf) -> Result<(), SequenceReadError> {
        crate::io::is_valid_label_dir(path.clone(), self.max_frame + 1)?;
        self.set_label_intern(Some(path));
        Ok(())
    }
    pub fn discard_labels(&mut self) {
        self.set_label_intern(None);
    }
    fn set_label_intern(&mut self, label: Option<PathBuf>) {
        if let Some(sequence) = &mut self.sequence {
            sequence.label_folder = label;
            for frame in &mut sequence.frames {
                *frame = None;
            }
            for load_state in &mut sequence.load_states {
                *load_state = LoadState::NotRequested;
            }
            self.last_rendered_frame = usize::MAX;
            self.request_frame(self.actual_frame);
        }
    }
    pub fn set_time_scale(&mut self, time_scale: f64) {
        self.speed.time_scale = time_scale.max(0.0);
        self.start_time = None;
        self.start_frame = self.actual_frame;
    }
    pub fn set_sensor_fps(&mut self, fps: f64) {
        self.speed.sensor_fps = fps;
    }
    pub fn get_time_scale(&self) -> f64 {
        self.speed.time_scale
    }
    pub fn set_mesh(&mut self, mesh: Handle<Mesh>) {
        self.mesh = Some(mesh);
    }
    pub fn set_sequence(&mut self, sequence: Sequence) {
        self.max_frame = sequence.frame_count.saturating_sub(1);
        self.sequence_number += 1;
        self.sequence = Some(sequence);
        self.start_frame = 0;
        self.actual_frame = 0;
        self.last_rendered_frame = usize::MAX;
        self.buffer_frame = 0;
        self.paused = true;
    }
    pub fn request_frame(&mut self, frame: usize) {
        self.has_frame_request = true;
        self.start_frame = frame;
        self.actual_frame = frame;
        self.buffer_frame = frame;
        self.start_time = None;
        self.free_memory_after_request();
    }
    pub fn request_update(&mut self) {
        self.last_rendered_frame = usize::MAX;
    }
    pub fn next_frame(&mut self) {
        self.request_frame(self.actual_frame + 1);
    }
    pub fn previous_frame(&mut self) {
        self.request_frame(self.actual_frame.saturating_sub(1));
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
        self.actual_frame = ((passed_time * self.speed.sensor_fps * self.speed.time_scale)
            as usize
            + self.start_frame)
            .min(self.max_frame)
            .max(0);
    }
    fn free_memory_after_request(&mut self) {
        if let Some(sequence) = &mut self.sequence {
            let min = self.actual_frame.saturating_sub(PlayerState::MEMORY_RANGE);
            let max = self.actual_frame.saturating_add(PlayerState::MEMORY_RANGE);
            let memory_range = min..max;
            sequence
                .frames
                .iter_mut()
                .zip(&mut sequence.load_states)
                .enumerate()
                .filter(|(iter, _)| !memory_range.contains(iter))
                .for_each(|(_, (frame, load_state))| {
                    *frame = None;
                    *load_state = LoadState::NotRequested;
                });
        }
    }
    fn free_memory_after_frame_update(&mut self) {
        if let Some(sequence) = &mut self.sequence {
            let frame_to_delete = self.actual_frame.saturating_sub(PlayerState::MEMORY_RANGE);
            if frame_to_delete != 0 {
                sequence.frames[frame_to_delete] = None;
                sequence.load_states[frame_to_delete] = LoadState::NotRequested;
            }
        }
    }
}

fn load_config(
    mut state: ResMut<PlayerState>,
    mut meshes: ResMut<Assets<Mesh>>,
    config: Res<PlayerConfig>,
) {
    state.set_sensor_fps(config.persistent.sensor_fps);
    state.mesh = Some(meshes.add(Mesh::from(shape::Cube {
        size: config.persistent.point_size,
    })));
    if let Some(file_path) = &config.persistent.folder_path {
        match read_sequence_from_dir(file_path.into()) {
            Ok(sequence) => state.set_sequence(sequence),
            Err(error) => {
                rfd::MessageDialog::new()
                    .set_title("Error")
                    .set_description(&format!("Cannot read folder: {file_path}\n{error}"))
                    .set_buttons(rfd::MessageButtons::Ok)
                    .set_level(rfd::MessageLevel::Error)
                    .show();
            }
        }
    }
}

fn player(
    mut commands: Commands,
    time: Res<Time>,
    mut state: ResMut<PlayerState>,
    config: Res<PlayerConfig>,
    query: Query<Entity, With<InstanceMaterialData>>,
) {
    if !state.paused && !state.wait_for_buffering {
        state.update(time.elapsed_seconds_f64());
    }
    if state.wait_for_buffering {
        state.wait_for_buffering = state.buffer_frame
            < usize::min(
                state.actual_frame + PlayerState::MINIMUM_BUFFERED_FRAMES,
                state.max_frame,
            );
    }
    if state.last_rendered_frame != state.actual_frame {
        if let Some(sequence) = &state.sequence {
            if let Some(frame) = &sequence.frames[state.actual_frame] {
                //change frame content
                query.for_each(|entity| commands.entity(entity).despawn());
                spawn_frame(
                    &mut commands,
                    &config,
                    frame,
                    state.mesh.as_ref().unwrap().clone(),
                );
                state.last_rendered_frame = state.actual_frame;
            } else {
                state.wait_for_buffering = true;
                state.start_time = None;
            }
        }
        state.free_memory_after_frame_update();
    }
}

fn spawn_frame(commands: &mut Commands, config: &PlayerConfig, frame: &Frame, mesh: Handle<Mesh>) {
    commands.spawn((
        mesh,
        SpatialBundle::default(),
        InstanceMaterialData(if frame.labels.is_some() {
            frame
                .points
                .iter()
                .zip(frame.labels.as_ref().unwrap().iter())
                .map(|(point, label)| InstanceData {
                    position: point.position,
                    scale: 1.0,
                    color: label_to_color(label, &config.actual_color_map, &config.default_color),
                })
                .collect()
        } else {
            frame
                .points
                .iter()
                .map(|point| InstanceData {
                    position: point.position,
                    scale: 1.0,
                    color: config.default_color,
                })
                .collect()
        }),
        NoFrustumCulling,
    ));
}

#[derive(Component)]
struct ReadFrameTask {
    task: Task<Result<Frame, FrameReadError>>,
    frame_number: usize,
    sequence_number: u32,
}

fn buffer_next_frames(mut commands: Commands, mut state: ResMut<PlayerState>) {
    let thread_pool = IoTaskPool::get();
    let mut buffer_frame = state.buffer_frame;
    let max_buffer_frame = usize::min(
        state.actual_frame + PlayerState::MAX_BUFFER_RANGE,
        state.max_frame,
    );
    if buffer_frame == max_buffer_frame {
        return;
    }
    let sequence_number = state.sequence_number;
    if let Some(sequence) = &mut state.sequence {
        sequence
            .load_states
            .iter_mut()
            .enumerate()
            .skip(buffer_frame)
            .skip_while(|(iter, load_state)| {
                let skip = **load_state == LoadState::Loaded && *iter < max_buffer_frame;
                if skip {
                    buffer_frame = *iter;
                }
                skip
            })
            .take(PlayerState::BUFFER_SLIDING_WINDOW)
            .for_each(|(iter, load_state)| {
                if *load_state == LoadState::NotRequested {
                    let points_path = sequence.point_folder.join(format!("{:0>6}.bin", iter));
                    let labels_path = sequence
                        .label_folder
                        .as_ref()
                        .map(|path| path.join(format!("{:0>6}.label", iter)));
                    let task =
                        thread_pool.spawn(async move { read_frame(points_path, labels_path) });
                    commands.spawn(ReadFrameTask {
                        task,
                        frame_number: iter,
                        sequence_number,
                    });
                    *load_state = LoadState::Requested;
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
    let sequence_number = state.sequence_number;
    if let Some(sequence) = &mut state.sequence {
        for (entity, mut task) in &mut read_frame_tasks {
            if task.sequence_number != sequence_number {
                commands.entity(entity).despawn();
                continue;
            }
            if let Some(Ok(frame)) = future::block_on(future::poll_once(&mut task.task)) {
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

pub fn label_to_color(
    label: &io::Label,
    color_map: &HashMap<u16, [f32; 4]>,
    default_color: &[f32; 4],
) -> [f32; 4] {
    *color_map.get(&label.label.into()).unwrap_or(default_color)
}
