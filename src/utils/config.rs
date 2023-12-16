use smart_leds::{colors::BLACK, RGB};

pub struct Config {
    ble_advertising_name: String,
    normal_mode_color: RGB<u8>,
    blinking_color_on: RGB<u8>,
    blinking_color_off: RGB<u8>,
    turn_light_anim: u8,
    normal_mode_anim: u8,
}

impl Config {
    pub fn new() -> Self {
        Self {
            ble_advertising_name: String::from(""),
            normal_mode_color: BLACK,
            blinking_color_on: BLACK,
            blinking_color_off: BLACK,
            turn_light_anim: 0,
            normal_mode_anim: 0,
        }
    }
}
