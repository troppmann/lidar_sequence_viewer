use bevy::prelude::*;
use super::PlayerState;



pub fn detect_point_under_curser(
    player: Res<PlayerState>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    windows: Res<Windows>,
) 
{
    //Get access to actual frame
    let Some(frame) = player.get_frame_content() else {
        println!("No Frame");
        return;
    };
    //get mouse position
    let Some(window) = windows.get_primary() else {
        println!("No Window");
        return;
    };
    let Some(mouse_position) = window.cursor_position() else {
        println!("No Mouse position");
        return;
    };
    //get mouse world position and screen direction
    let (camera, transform) = cameras.single();
    let Some(ray) = camera.viewport_to_world(transform, mouse_position) else {
        println!("No Ray");
        return;
    };

    //Assume all points are sphere with a radius
    //TODO adjustable radius with precalculated square
    let mut min_distance = f32::MAX;
    let mut min_index = None;
    for (iter, point) in frame.points.iter().enumerate(){
        let sphere_pos_to_ray_pos = point.position - ray.origin;
        let projected_distance = sphere_pos_to_ray_pos.dot(ray.direction);
        if projected_distance < 0.0 {
            continue;
        }
        let distance_to_ray_squared = sphere_pos_to_ray_pos.length_squared() - projected_distance * projected_distance;
        const RADIUS_SQUARED: f32 = 0.04 * 0.04;
        if distance_to_ray_squared > RADIUS_SQUARED {
            continue;
        }
        if projected_distance < min_distance {
            min_distance = projected_distance;
            min_index = Some(iter);
        }
    }
    if let Some(index) = min_index {
        if let Some(labels) = &frame.labels{
            println!("{:?}", labels[index]);
        }
    }
}