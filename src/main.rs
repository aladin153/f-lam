pub mod anim;
pub mod bluetooth;
pub mod io;
mod utils;
use crate::io::inputs;
use esp32_nimble::utilities::mutex::Mutex;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_sys::xTaskCreatePinnedToCore;
use esp_idf_sys::{self as _};
use smart_leds::colors::RED;
use smart_leds_trait::RGB8;
use std::ffi::CString;
use std::sync::Arc;
use utils::timeout::ValueWithTimeout;

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

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let arc_aladin = Arc::new(Mutex::new(MailBox::new()));

    //To C const void*.
    let ptr: *const Arc<Mutex<MailBox>> = &arc_aladin;
    let voidptr = ptr as *mut core::ffi::c_void;

    let light_input_pc_name = CString::new("Task 3").unwrap();
    let ble_pc_name = CString::new("Task 4").unwrap();
    let angel_eye_pc_name = CString::new("Task 5").unwrap();

    unsafe {
        xTaskCreatePinnedToCore(
            Some(inputs::light_input_task),
            light_input_pc_name.as_ptr(),
            5000,
            voidptr,
            8,
            std::ptr::null_mut(),
            1,
        );
    }

    unsafe {
        xTaskCreatePinnedToCore(
            Some(bluetooth::ble::ble_task),
            ble_pc_name.as_ptr(),
            8000,
            voidptr,
            8,
            std::ptr::null_mut(),
            0,
        );
    }

    unsafe {
        xTaskCreatePinnedToCore(
            Some(io::angel_eye::angel_eye_task),
            angel_eye_pc_name.as_ptr(),
            8000,
            voidptr,
            10,
            std::ptr::null_mut(),
            1,
        );
    }

    loop {
        FreeRtos::delay_ms(100);
    }
}
