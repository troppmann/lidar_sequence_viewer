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
        //let point = inspector.point.clone();
        let label = inspector.label.clone();
        egui::Window::new("Inspector").open(&mut inspector.visible).resizable(true).show(ctx, |ui| {
            egui::Grid::new("InspectorGird").num_columns(2).min_col_width(100.0).show(ui, |ui| {                
                //fields
                let missing_value = String::from("");
              //let mut x = 0.0;
              //let mut y = 0.0;
              //let mut z = 0.0;
                let mut label_name = missing_value.clone();
                let mut label_id = missing_value.clone();
                let mut label_instance_id = missing_value;

              //if let Some(point) = point {
              //    x = point.position.x;
              //    y = point.position.y;
              //    z = point.position.z;
              //}
                if let Some(label) = label {
                    label_name = config.persistent.label_map.get(&label.label).map(|info| info.name.clone()).unwrap_or_default();
                    label_id = label.label.to_string();
                    label_instance_id = label.instance_id.to_string();
                }

              //ui.label("Position");
              //ui.horizontal(|ui| {
              //    ui.label(RichText::new(format!("{:+04.0}", x)).monospace());
              //    ui.label(RichText::new(format!("{:+04.0}", y)).monospace());
              //    ui.label(RichText::new(format!("{:+04.0}", z)).monospace());
              //});
              //ui.end_row();
                ui.label("Label:");
                ui.label(label_name);
                ui.end_row();
                ui.label("ID:");
                ui.label(label_id);
                ui.end_row();
                ui.label("Instance ID:");
                ui.label(label_instance_id);
                ui.end_row();
            });
        });
    }
    pub fn detect_point_under_curser(
        player: Res<PlayerState>,
        cameras: Query<(&Camera, &GlobalTransform)>,
        windows: Res<Windows>,
        mut inspector: ResMut<Self>
    ) 
    {
        if !inspector.visible{
            return;
        }
        let Some(frame) = player.get_frame_content() else {
            return;
        };
        let Some(window) = windows.get_primary() else {
            return;
        };
        let Some(mouse_position) = window.cursor_position() else {
            return;
        };
        let (camera, transform) = cameras.single();
        let Some(ray) = camera.viewport_to_world(transform, mouse_position) else {
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