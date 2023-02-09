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
    println!("{:?}", ray);
    for point in &frame.points{
        
    }
    //iterate over all points
        //check collision
        //sort trough range
        //save latest point//index
}