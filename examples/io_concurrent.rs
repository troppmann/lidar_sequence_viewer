use std::{path::Path, fs};

use bevy::{prelude::*, tasks::{Task, IoTaskPool}};
use futures_lite::future;
use lidar_viewer::io::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(lidar_viewer::plugins::ObserverPlugin)
        .insert_resource(State{sequence: None})
        .add_startup_system(init_io_sequence)
        .add_system(handle_read_frames_task)
        .run();
}
#[derive(Component)]
struct ReadFrameTask {
    task: Task<Frame>,
    frame_number: usize,
}
struct IoSequence{
    folder: String,
    length: usize,
    frames: Vec<Option<Frame>>,
}
#[derive(Resource)]
struct State{
    sequence: Option<IoSequence>
}

fn get_io_sequence_from_dir(path: String) -> Result<IoSequence, Box<dyn std::error::Error>>{
    let paths = fs::read_dir(&path)?;
    let frame_files = paths.into_iter() .filter_map(|x| x.ok().map(|entry| entry.path())).filter(|path| match path.extension() {
        Some(x) => x == "bin",
        None => false,
    });
    let length = frame_files.count();
    Ok(IoSequence{
        folder: path,
        length,
        frames: std::iter::repeat_with(|| None).take(length).collect(),
    })
}


fn init_io_sequence(commands: Commands, mut state: ResMut<State>){
    let sequence = get_io_sequence_from_dir(String::from("../SemanticKITTI/dataset/sequences/04/velodyne/")).unwrap();
    read_sequence(commands, &sequence);
    state.sequence = Some(sequence);
}

fn read_sequence(mut commands: Commands, sequence: &IoSequence) {
    let thread_pool = IoTaskPool::get();
    for i in 0..sequence.length{
        // todo path string
        let  path = format!("{}/{:0>6}.bin", sequence.folder, i);
        let task = thread_pool.spawn(async move {
            read_frame(Path::new(&path)).unwrap()
        });
        commands.spawn(ReadFrameTask{task, frame_number: i});
    }
}
fn handle_read_frames_task(mut commands:Commands, 
                    mut read_frame_tasks: Query<(Entity, &mut ReadFrameTask)>,
                    mut state: ResMut<State>){
    let mut count = 0;
    for (entity, mut task) in &mut read_frame_tasks {
        if let Some(frame) = future::block_on(future::poll_once(&mut task.task)) {
            if let Some(sequence) = &mut state.sequence {
                sequence.frames[task.frame_number] = Some(frame);
            }
            commands.entity(entity).despawn();
        }
        count += 1;
    }
    if count > 0 {
        println!("ReadFrameTask in System: {}", count);
    }
}