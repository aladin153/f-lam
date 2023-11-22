use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;

pub struct Drivers {
    pub board_led: PinDriver<'static, Gpio2, Output>,
    pub input: PinDriver<'static, Gpio12, Input>,
}

unsafe impl Sync for Drivers {
    // TODO
}

unsafe impl Send for Drivers {}

impl Drivers {
    pub fn new() -> Self {
        let peripherals = Peripherals::take().unwrap_or_else(|error| {
            panic!("Peripherals initialization issue : {:?}", error);
        });

        let led_driver = PinDriver::output(peripherals.pins.gpio2).unwrap_or_else(|error| {
            panic!("Unable to initialize gpio2 as an Output : {:?}", error);
        });

        let mut input_driver = PinDriver::input(peripherals.pins.gpio12).unwrap_or_else(|error| {
            panic!("Unable to initialize gpio12 as an Input : {:?}", error);
        });
        input_driver.set_pull(Pull::Down).unwrap_or_else(|error| {
            panic!("Unable to apply Pull Down config to GPIO12 : {:?}", error);
        });

        Self {
            board_led: led_driver,
            input: input_driver,
        }
    }
}

impl Default for Drivers {
    fn default() -> Self {
        Self::new()
    }
}
