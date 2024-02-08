use crate::utils::colors::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub normal_mode_color: Color,
    //pub devil_eye_color: Color,
    pub blinking_color_on: Color,
    pub blinking_color_off: Color,
    pub turn_light_anim: u8,
    pub normal_mode_anim: u8,
}

impl Config {
    pub fn new() -> Self {
        Self {
            normal_mode_color: Color::from(BLACK),
            //devil_eye_color: Color::from(BLACK),
            blinking_color_on: Color::from(BLACK),
            blinking_color_off: Color::from(BLACK),
            turn_light_anim: 0,
            normal_mode_anim: 0,
        }
    }
}
// TODO : Add Method : Load Config from NVS

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
