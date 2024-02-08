pub mod anim;
pub mod bluetooth;
pub mod io;
pub mod storage;
mod utils;
use crate::io::inputs;
use crate::storage::nvs::Nvs;
use crate::utils::mailbox::MailBox;
use esp32_nimble::utilities::mutex::Mutex;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_sys::xTaskCreatePinnedToCore;
use esp_idf_sys::{self as _};
use std::ffi::CString;
use std::sync::Arc;
use storage::config::Config;

use esp_idf_svc::log::EspLogger;
use esp_idf_svc::nvs::*;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    // Nvm Initialization !!!!!!!!    TODO
    EspLogger::initialize_default();

    let nvs_default_partition: EspNvsPartition<NvsDefault> = EspDefaultNvsPartition::take()?;

    let test_namespace = "NVS";
    let mut nvs = match EspNvs::new(nvs_default_partition, test_namespace, true) {
        Ok(nvs) => {
            log::info!("Got namespace {:?} from default partition", test_namespace);
            nvs
        }
        Err(e) => panic!("Could't get namespace {:?}", e),
    };

    // Load Configuration from NVS.
    let mut config = Config::new();
    // TODO ALADIN ZAYEN
    if let Ok(_config_from_nvs) = config.read_all_saved_data(&mut nvs) {
        println!("Config Successefuly Readed, Updating the Mailbox !!!!!");
        // TODO Update the config from NVS
        log::error!("Main function : Config from NVS = {:#?}", config);
    } else {
        println!("Unable to read Configuartion from NVS partition. Using Default Config");
        // TODO
    }

    let arc_aladin = Arc::new(Mutex::new(MailBox::new(&config)));

    // Load Calibration from NVS, otherwise enter First Install Mode
    //let calib = Calib::new();
    //let y = calib.read_all_saved_data(&mut nvs);
    //if let Ok(calib_from_nvs) = y {
    //    println!("Calib Successefuly Readed !!!!!");
    //} else {
    //    println!("Unable to read Calibration from NVS partition. Entering first install mode");
    //    log::error!("Entering first install Mode !!!!!!");
    //}

    //To C const void*.
    let ptr: *const Arc<Mutex<MailBox>> = &arc_aladin;
    let voidptr = ptr as *mut core::ffi::c_void;

    let light_input_pc_name = CString::new("Task 3").unwrap();
    let ble_pc_name = CString::new("Task 4").unwrap();
    let angel_eye_pc_name = CString::new("Task 5").unwrap();
    let devil_eye_pc_name = CString::new("Task 6").unwrap();

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

    unsafe {
        xTaskCreatePinnedToCore(
            Some(io::devil_eye::devil_eye_task),
            devil_eye_pc_name.as_ptr(),
            8000,
            voidptr,
            10,
            std::ptr::null_mut(),
            1,
        );
    }

    loop {
        if arc_aladin.lock().saving_request {
            println!("Saving Request detected from the main function");

            // TODO : Add function Mailbox -> Config
            config.normal_mode_color = arc_aladin.lock().normal_mode_color.clone();

            let _x = config.save_all_data(&mut nvs);

            log::error!(
                "Read Config after Save NVS = {:#?}",
                config.read_all_saved_data(&mut nvs)
            );

            arc_aladin.lock().saving_request = false;
        }
        FreeRtos::delay_ms(200);
    }
}
