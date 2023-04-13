use std::collections::BTreeMap;

use bevy::{
    prelude::{Color, Plugin, Resource},
    utils::HashMap,
};
use serde::{Deserialize, Serialize};

type ColorRgbU8 = [u8; 3];
type ColorRgbaF32 = [f32; 4];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelInfo {
    pub name: String,
    pub color: ColorRgbU8,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub label_map: BTreeMap<u16, LabelInfo>,
    pub default_color: [u8; 3],
    pub background_color: [u8; 3],
    pub file_path: Option<String>,
    pub camera_fov: f32,
    pub camera_speed: f32,
    pub object_size: f32,
}
impl From<(&str, [u8; 3])> for LabelInfo {
    fn from(value: (&str, [u8; 3])) -> Self {
        Self {
            name: value.0.to_string(),
            color: value.1,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            label_map: BTreeMap::from([
                (0, ("unlabeled", [0, 0, 0]).into()),
                (1, ("outlier", [0, 0, 255]).into()),
                (10, ("car", [245, 150, 100]).into()),
                (11, ("bicycle", [245, 230, 100]).into()),
                (13, ("bus", [250, 80, 100]).into()),
                (15, ("motorcycle", [150, 60, 30]).into()),
                (16, ("on-rails", [255, 0, 0]).into()),
                (18, ("truck", [180, 30, 80]).into()),
                (20, ("other-vehicle", [255, 0, 0]).into()),
                (30, ("person", [30, 30, 255]).into()),
                (31, ("bicyclist", [200, 40, 255]).into()),
                (32, ("motorcyclist", [90, 30, 150]).into()),
                (40, ("road", [255, 0, 255]).into()),
                (44, ("parking", [255, 150, 255]).into()),
                (48, ("sidewalk", [75, 0, 75]).into()),
                (49, ("other-ground", [75, 0, 175]).into()),
                (50, ("building", [0, 200, 255]).into()),
                (51, ("fence", [50, 120, 255]).into()),
                (52, ("other-structure", [0, 150, 255]).into()),
                (60, ("lane-marking", [170, 255, 150]).into()),
                (70, ("vegetation", [0, 175, 0]).into()),
                (71, ("trunk", [0, 60, 135]).into()),
                (72, ("terrain", [80, 240, 150]).into()),
                (80, ("pole", [150, 240, 255]).into()),
                (81, ("traffic-sign", [0, 0, 255]).into()),
                (99, ("other-object", [255, 255, 50]).into()),
                (252, ("moving-car", [245, 150, 100]).into()),
                (256, ("moving-bicyclist", [255, 0, 0]).into()),
                (253, ("moving-person", [200, 40, 255]).into()),
                (254, ("moving-motorcyclist", [30, 30, 255]).into()),
                (255, ("moving-on-rails", [90, 30, 150]).into()),
                (257, ("moving-bus", [250, 80, 100]).into()),
                (258, ("moving-truck", [180, 30, 80]).into()),
                (259, ("moving-other-vehicle", [255, 0, 0]).into()),
            ]),
            default_color: [180, 100, 25],
            background_color: [0, 41, 61],
            file_path: None,
            camera_fov: 90.0,
            camera_speed: 10.0,
            object_size: 0.004,
        }
    }
}

#[derive(Resource, Default)]
pub struct PlayerConfig {
    pub persistent: Config,
    pub actual_color_map: HashMap<u16, ColorRgbaF32>,
    pub default_color: ColorRgbaF32,
}

impl PlayerConfig {
    const APP_NAME: &str = "lidar_viewer";
    pub fn load(&mut self) {
        let config: Config = match confy::load(PlayerConfig::APP_NAME, None) {
            Ok(config) => config,
            Err(error) => {
                eprintln!("{error}");
                return;
            }
        };
        self.persistent = config;
        self.update_label_map();
    }
    pub fn save(&self) {
        if let Err(error) = confy::store(PlayerConfig::APP_NAME, None, &self.persistent) {
            eprintln!("{error}");
        };
    }
    pub fn reset_label_map(&mut self) {
        self.persistent.label_map = Self::default().persistent.label_map;
        self.update_label_map();
    }
    pub fn update_label_map(&mut self) {
        let entries = self.persistent.label_map.iter().map(|(label, info)| {
            (
                *label,
                PlayerConfig::convert_rgba_from_u8_to_f32(&info.color),
            )
        });
        self.actual_color_map = entries.collect();
        self.default_color =
            PlayerConfig::convert_rgba_from_u8_to_f32(&self.persistent.default_color);
    }
    fn convert_rgba_from_u8_to_f32(color: &ColorRgbU8) -> ColorRgbaF32 {
        Color::rgb_u8(color[0], color[1], color[2]).as_linear_rgba_f32()
    }
}

pub struct ConfigPlugin;
impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(PlayerConfig::default());
    }
}
