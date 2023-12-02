use std::sync::{Arc, Mutex, RwLock};
pub mod anim;
pub mod bluetooth;
pub mod io;
mod utils;
use crate::io::angel_eye::AngelEye;
use io::inputs::LightInputSignals;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let light_inputs: LightInputSignals = LightInputSignals::new();

    let angel_eye = Arc::new(Mutex::new(AngelEye::new()));

    let inputs_rw_lock = Arc::new(RwLock::new(light_inputs.clone()));

    log::info!("Starting angel eyes animation threads !!!");

    (*angel_eye)
        .lock()
        .unwrap()
        .play_turn_animation(angel_eye.clone(), inputs_rw_lock.clone());

    (*angel_eye)
        .lock()
        .unwrap()
        .play_normal_mode_animation(angel_eye.clone(), inputs_rw_lock.clone());

    log::info!("Starting light inputs thread");

    light_inputs.start_thread(inputs_rw_lock);
}
