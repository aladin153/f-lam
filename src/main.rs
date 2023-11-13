pub mod config;
use config::drivers::*;
use esp_idf_hal::delay::FreeRtos;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");
    log::info!("Alaeddine ZAYEN!");

    let mut drivers: Drivers = Drivers::new();
    drivers.init();

    loop {
        drivers
            .board_led
            .as_mut()
            .unwrap()
            .set_high()
            .expect("Error");
        FreeRtos::delay_ms(1000);
        drivers
            .board_led
            .as_mut()
            .unwrap()
            .set_low()
            .expect("error");
        FreeRtos::delay_ms(1000);
    }
}
