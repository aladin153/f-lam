pub mod anim;
pub mod bluetooth;
pub mod io;
pub mod storage;
mod utils;
use crate::io::inputs;
use crate::utils::mailbox::MailBox;
use esp32_nimble::utilities::mutex::Mutex;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_sys::xTaskCreatePinnedToCore;
use esp_idf_sys::{self as _};
use std::ffi::CString;
use std::sync::Arc;
use utils::calib::Calib;
use utils::config::Config;

use esp_idf_svc::log::EspLogger;
use esp_idf_svc::nvs::*;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let arc_aladin = Arc::new(Mutex::new(MailBox::new()));

    // Nvm Initialization !!!!!!!!    TODO
    EspLogger::initialize_default();

    let nvs_default_partition: EspNvsPartition<NvsDefault> = EspDefaultNvsPartition::take()?;

    let test_namespace = "test_ns";
    let mut nvs = match EspNvs::new(nvs_default_partition, test_namespace, true) {
        Ok(nvs) => {
            log::info!("Got namespace {:?} from default partition", test_namespace);
            nvs
        }
        Err(e) => panic!("Could't get namespace {:?}", e),
    };

    let _calib = Calib::new();
    let _config = Config::new();

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
