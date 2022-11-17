use std::path::Path;

use bevy::prelude::*;

use crate::io::read_frame;

pub struct LidarPlugin;

impl Plugin for LidarPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(example_spawn);
    }
}

fn example_spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(StandardMaterial {
        base_color: Color::rgb_u8(247, 127, 0),
        unlit: true,
        ..default()
    });
    let mesh = meshes.add(Mesh::from(shape::Icosphere {
        radius: 0.04,
        subdivisions: 1,
    }));

    let path = Path::new("../SemanticKITTI/dataset/sequences/00/velodyne/000000.bin");
    let frame = read_frame(path).unwrap();
    for point in frame.0.iter().take(200000) {
        commands.spawn(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::from_xyz(point.x, point.z, point.y),
            ..default()
        });
    }
}
