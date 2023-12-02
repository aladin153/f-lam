use crate::utils::timeout::BinayLevel;
use crate::utils::timeout::Timeout;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

#[derive(Debug, Clone)]
pub struct LightInputSignals {
    left: Timeout<bool>,
    right: Timeout<bool>,
    low_beam: bool,
}

#[derive(Debug, Clone)]
pub enum TurnLightStatus {
    On,
    Off,
    Timeout,
}

impl LightInputSignals {
    pub fn new() -> Self {
        Self {
            left: Timeout::new(BinayLevel::Low),
            right: Timeout::new(BinayLevel::Low),
            low_beam: false,
        }
    }

    pub fn left(&self) -> Timeout<bool> {
        self.left
    }

    pub fn right(&self) -> Timeout<bool> {
        self.right
    }

    pub fn low_beam(&self) -> bool {
        self.low_beam
    }

    // TODO : Verify that the used pinout are not reserved for CAN bus
    // TODO : Error Handling
    // TODO : Pinout Struct
    pub fn start_thread(mut self, data: Arc<RwLock<LightInputSignals>>) {
        let peripherals = Peripherals::take().unwrap_or_else(|error| {
            panic!("Peripherals initialization issue : {:?}", error);
        });

        let mut turn_left_signal =
            PinDriver::input(peripherals.pins.gpio12).unwrap_or_else(|error| {
                panic!("Unable to initialize gpio12 as an Input : {:?}", error);
            });
        turn_left_signal
            .set_pull(Pull::Down)
            .unwrap_or_else(|error| {
                panic!("Unable to apply Pull Down config to GPIO12 : {:?}", error);
            });

        let mut turn_right_signal =
            PinDriver::input(peripherals.pins.gpio13).unwrap_or_else(|error| {
                panic!("Unable to initialize gpio13 as an Input : {:?}", error);
            });
        turn_right_signal
            .set_pull(Pull::Down)
            .unwrap_or_else(|error| {
                panic!("Unable to apply Pull Down config to GPIO13 : {:?}", error);
            });

        let mut low_beam_signal =
            PinDriver::input(peripherals.pins.gpio14).unwrap_or_else(|error| {
                panic!("Unable to initialize gpio14 as an Input : {:?}", error);
            });
        low_beam_signal
            .set_pull(Pull::Down)
            .unwrap_or_else(|error| {
                panic!("Unable to apply Pull Down config to GPIO14 : {:?}", error);
            });

        thread::spawn(move || loop {
            if let Ok(mut write_guard) = data.write() {
                write_guard.left = self.left;
                write_guard.right = self.right;
                write_guard.low_beam = self.low_beam;
            }

            self.right.step(turn_right_signal.is_high(), 0, 500);
            self.left.step(turn_left_signal.is_high(), 0, 500);
            self.low_beam = low_beam_signal.is_high();
            esp_idf_hal::delay::FreeRtos::delay_ms(40);
        });
    }
}

impl Default for LightInputSignals {
    fn default() -> Self {
        Self::new()
    }
}
