use std::path::Path;

use bevy::{prelude::*, render::view::NoFrustumCulling};

use crate::io::*;

use super::instancing::*;

pub struct LidarPlugin;

impl Plugin for LidarPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayData::default())
            .add_startup_system(init_sequence)
            .add_system(player)
            .add_plugin(InstancingPlugin);
    }
}
#[derive(Resource)]
struct PlayData {
    counter: i64,
    start_time: f64,
    sequence: Option<Sequence>,
    mesh: Option<Handle<Mesh>>,
    actual_frame: usize,
    has_frame_changes: bool,
}

impl Default for PlayData {
    fn default() -> Self {
        Self {
            counter: 0,
            start_time: 0.0,
            sequence: None,
            mesh: None,
            actual_frame: 0,
            has_frame_changes: false,
        }
    }
}

impl PlayData {
    pub fn update(&mut self, time_in_seconds: f64){

        if let Some(sequence) = &self.sequence{
            let passed_time = time_in_seconds - self.start_time;
            let old_index = self.actual_frame; 
            self.actual_frame = ((passed_time * 10.0) as usize).min(sequence.frame_count - 1).max(0);
            self.has_frame_changes = old_index != self.actual_frame;
        }
    }
}


fn init_sequence(
    mut playdata: ResMut<PlayData>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let path = Path::new("../SemanticKITTI/dataset/sequences/04/velodyne/");
    playdata.sequence = Some(read_sequence_from_dir(path).unwrap());
    playdata.start_time = time.elapsed_seconds_f64();
    playdata.mesh = Some(meshes.add(Mesh::from(shape::Cube { size: 0.04 })))
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
    mut playdata: ResMut<PlayData>,
    query: Query<Entity, With<InstanceMaterialData>>,
) {
    playdata.counter += 1;
    playdata.update(time.elapsed_seconds_f64());
    if playdata.has_frame_changes {
        query.for_each(|entity| commands.entity(entity).despawn());
        if let Some(sequence) = &playdata.sequence {
            spawn_frame(
                &mut commands,
                &sequence.frames[playdata.actual_frame],
                playdata.mesh.as_ref().unwrap().clone(),
            )
        }
    }
}
