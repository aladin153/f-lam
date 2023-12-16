use crate::utils::timeout::ValueWithTimeout;
use smart_leds::colors::*;
use smart_leds::RGB8;
pub struct MailBox {
    // TODO
    pub data: bool,
    pub left_side_signal: bool,
    pub right_side_signal: bool,
    pub low_beam: bool,
    pub left_turn_signal: ValueWithTimeout,
    pub right_turn_signal: ValueWithTimeout,
    pub ble_data0: u8,
    pub ble_data1: u8,
    pub ble_data2: u8,
    pub ble_data3: u8,
    pub normal_mode_color: RGB8,
}

impl MailBox {
    pub fn new() -> Self {
        Self {
            data: false,
            left_side_signal: false,
            right_side_signal: false,
            low_beam: false,
            left_turn_signal: ValueWithTimeout::Off,
            right_turn_signal: ValueWithTimeout::Off,
            ble_data0: 0,
            ble_data1: 0,
            ble_data2: 0,
            ble_data3: 0,
            normal_mode_color: RED, // TODO From Config
        }
    }
}

impl Default for MailBox {
    fn default() -> Self {
        Self::new()
    }
}
