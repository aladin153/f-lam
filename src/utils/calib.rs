use crate::storage::storage;

pub struct Calib {
    angel_eye_total_led: usize, // TODO
    turn_signal_timeout: u8,
}

impl Calib {
    pub fn new() -> Self {
        Self {
            angel_eye_total_led: 0,
            turn_signal_timeout: 0,
        }
    }
}
