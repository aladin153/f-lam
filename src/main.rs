pub mod bluetooth;
pub mod config;
use bluetooth::ble::Ble;
use config::drivers::Drivers;
use smart_leds::colors::*;
use smart_leds_trait::SmartLedsWrite;
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;

pub struct FLAM {}

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");
    log::info!("Alaeddine ZAYEN!");

    let mut _drivers: Drivers = Drivers::new();

    let ble = Ble::new(_drivers);
    ble.init();

    let mut ws2812 = Ws2812Esp32Rmt::new(0, 27).unwrap();

    println!("Start NeoPixel: Flashing Animation!");

    loop {
        let pixels = std::iter::repeat(ORANGE).take(45);

        ws2812.write(pixels).unwrap();

        esp_idf_hal::delay::FreeRtos::delay_ms(1000);

        let pixels = std::iter::repeat(BLACK).take(45);

        ws2812.write(pixels).unwrap();

        esp_idf_hal::delay::FreeRtos::delay_ms(1000);
    }
}
