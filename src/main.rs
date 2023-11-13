use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");
    log::info!("Alaeddine ZAYEN!");

    let peripherals = Peripherals::take().unwrap();
    let mut led =
        PinDriver::output(peripherals.pins.gpio2).expect("Error: Unable to set gpio2 as an Output");

    loop {
        log::info!("Setting gpio2 to 1!");
        led.set_high().expect("Error: Unable to set gpio2 to high");
        FreeRtos::delay_ms(1000);
        log::info!("Setting gpio2 to 0 !");
        led.set_low().expect("Error: Unable to set gpio2 to low");
        FreeRtos::delay_ms(1000);
    }
}
