use bevy::prelude::*;
use bevy_egui::*;
use crate::{io::{Point, Label}, plugins::{PlayerConfig, lidar::PlayerState}};

#[derive(Resource, Default)]
pub struct Inspector{
    pub visible: bool,
    pub point: Option<Point>,
    pub label: Option<Label>,
}

impl Inspector{   
    pub fn draw(
        mut egui_context: ResMut<EguiContext>,
        mut inspector: ResMut<Self>,
        config: Res<PlayerConfig>, 
    ){
        let ctx = egui_context.ctx_mut();
        let point = inspector.point.clone();
        let label = inspector.label.clone();
        egui::Window::new("Inspector-WIdow").open(&mut inspector.visible).title_bar(false).resizable(false).show(ctx, |ui| {
            if let Some(point) = point {
                ui.label(format!("X: {}, Y: {}, Z: {}", point.position.x, point.position.y, point.position.z));
                match label {
                    Some(label) => {
                        let name = config.persistent.label_map.get(&label.label).map(|info| info.name.clone()).unwrap_or_default();
                        ui.label(format!("{}({})", name, label.label));
                        ui.label(format!("Instance ID: {}", label.instance_id));
                    },
                    None => {
                        ui.label("-");
                    },
                }
            } else {
                ui.label("-");
            }
        });
    }
    pub fn detect_point_under_curser(
        player: Res<PlayerState>,
        cameras: Query<(&Camera, &GlobalTransform)>,
        windows: Res<Windows>,
        mut inspector: ResMut<Self>
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
            inspector.point = Some(frame.points[index].clone());
            inspector.label = frame.labels.as_ref().map(|labels| labels[index]);
        } else {
            inspector.point = None;
            inspector.label = None;
        }

    }

}