use std::path::Path;

use bevy::{prelude::*, render::view::NoFrustumCulling};

use crate::io::read_frame;

use super::instancing::*;

pub struct LidarPlugin;

impl Plugin for LidarPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_frame)
            .add_plugin(InstancingPlugin);
    }
}

fn spawn_frame(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Cube {size: 0.04}));
    let path = Path::new("../SemanticKITTI/dataset/sequences/00/velodyne/000000.bin");
    let frame = read_frame(path).unwrap();
    commands.spawn((
        mesh.clone(),
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



