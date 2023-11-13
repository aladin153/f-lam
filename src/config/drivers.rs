use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;

pub struct Drivers<'a> {
    pub board_led: Option<PinDriver<'a, Gpio2, Output>>,
}

impl<'a> Drivers<'a> {
    pub fn new() -> Self {
        Self { board_led: None }
    }

    pub fn init(&mut self) {
        let peripherals = Peripherals::take().unwrap_or_else(|error| {
            panic!("Peripherals initialization issue : {:?}", error);
        });

        let led_driver = PinDriver::output(peripherals.pins.gpio2);

        match led_driver {
            Ok(gpio2_led) => self.board_led = Some(gpio2_led),
            Err(error) => {
                log::error!("Unable to initialize gpio2 as an Output : {:?}", error);
                // TODO : Error State
            }
        };
    }
}

impl<'a> Default for Drivers<'a> {
    fn default() -> Self {
        Self::new()
    }
}
