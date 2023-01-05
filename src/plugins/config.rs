use bevy::{
    prelude::{Color, Resource},
    utils::HashMap,
};
use serde::{Deserialize, Serialize};

type ColorRgbaU8 = [u8; 4];
type ColorRgbaF32 = [f32; 4];
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub color_map: HashMap<u32, ColorRgbaU8>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            color_map: HashMap::from([
                (0, [0, 0, 0, 0]),
                (1, [0, 0, 255, 0]),
                (10, [245, 150, 100, 0]),
                (11, [245, 230, 100, 0]),
                (13, [250, 80, 100, 0]),
                (15, [150, 60, 30, 0]),
                (16, [255, 0, 0, 0]),
                (18, [180, 30, 80, 0]),
                (20, [255, 0, 0, 0]),
                (30, [30, 30, 255, 0]),
                (31, [200, 40, 255, 0]),
                (32, [90, 30, 150, 0]),
                (40, [255, 0, 255, 0]),
                (44, [255, 150, 255, 0]),
                (48, [75, 0, 75, 0]),
                (49, [75, 0, 175, 0]),
                (50, [0, 200, 255, 0]),
                (51, [50, 120, 255, 0]),
                (52, [0, 150, 255, 0]),
                (60, [170, 255, 150, 0]),
                (70, [0, 175, 0, 0]),
                (71, [0, 60, 135, 0]),
                (72, [80, 240, 150, 0]),
                (80, [150, 240, 255, 0]),
                (81, [0, 0, 255, 0]),
                (99, [255, 255, 50, 0]),
                (252, [245, 150, 100, 0]),
                (256, [255, 0, 0, 0]),
                (253, [200, 40, 255, 0]),
                (254, [30, 30, 255, 0]),
                (255, [90, 30, 150, 0]),
                (257, [250, 80, 100, 0]),
                (258, [180, 30, 80, 0]),
                (259, [255, 0, 0, 0]),
            ]),
        }
    }
}

#[derive(Resource, Default)]
pub struct PlayerConfig {
    pub config: Option<Config>,
    pub actual_color_map: HashMap<u32, ColorRgbaF32>,
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
        self.config = Some(config);
        self.update_color_map();
    }
    pub fn save(&mut self) {
        if let Some(config) = self.config.as_ref() {
            if let Err(error) = confy::store(PlayerConfig::APP_NAME, None, config) {
                eprintln!("{error}");
            };
        }
    }
    pub fn update_color_map(&mut self) {
        if let Some(config) = &self.config {
            let entries = config
                .color_map
                .iter()
                .map(|(label, color)| (*label, PlayerConfig::convert_rgba_from_u8_to_f32(color)));
            self.actual_color_map = entries.collect();
        }
    }
    fn convert_rgba_from_u8_to_f32(color: &ColorRgbaU8) -> ColorRgbaF32 {
        Color::rgba_u8(color[0], color[1], color[2], color[3]).as_linear_rgba_f32()
    }
}
