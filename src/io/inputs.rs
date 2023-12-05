use crate::utils::timeout::BinayLevel;
use crate::utils::timeout::Timeout;
use crate::MailBox;
use esp32_nimble::utilities::mutex::Mutex;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_sys::{self as _};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct LightInputSignals {
    left_turn_light: Timeout<bool>,
    right_turn_light: Timeout<bool>,
    left_side_light: bool,
    right_side_light: bool,
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
            left_turn_light: Timeout::new(BinayLevel::Low),
            right_turn_light: Timeout::new(BinayLevel::Low),
            left_side_light: false,
            right_side_light: false,
            low_beam: false,
        }
    }

    pub fn left_turn_light(&self) -> Timeout<bool> {
        self.left_turn_light
    }

    pub fn right_turn_light(&self) -> Timeout<bool> {
        self.right_turn_light
    }

    pub fn left_side_light(&self) -> bool {
        self.left_side_light
    }

    pub fn right_side_light(&self) -> bool {
        self.right_side_light
    }

    pub fn low_beam(&self) -> bool {
        self.low_beam
    }

    // TODO : Verify that the used pinout are not reserved for CAN bus
    // TODO : Error Handling
    // TODO : Pinout Struct
    pub fn init_drivers(mut self, data: Arc<Mutex<MailBox>>) {
        // TODO
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

        // TODO :Add Sidelight
        let mut left_side_signal =
            PinDriver::input(peripherals.pins.gpio15).unwrap_or_else(|error| {
                // TODO : Not CAN pins or Angel Eye???
                panic!("Unable to initialize gpio15 as an Input : {:?}", error);
            });
        left_side_signal
            .set_pull(Pull::Down)
            .unwrap_or_else(|error| {
                panic!("Unable to apply Pull Down config to GPIO15 : {:?}", error);
            });

        let mut right_side_signal =
            PinDriver::input(peripherals.pins.gpio16).unwrap_or_else(|error| {
                // TODO : Not CAN pins or Angel Eye???
                panic!("Unable to initialize gpio14 as an Input : {:?}", error);
            });
        right_side_signal
            .set_pull(Pull::Down)
            .unwrap_or_else(|error| {
                panic!("Unable to apply Pull Down config to GPIO16 : {:?}", error);
            });

        loop {
            self.right_turn_light
                .step(turn_right_signal.is_high(), 0, 20);
            self.left_turn_light.step(turn_left_signal.is_high(), 0, 20);
            self.left_side_light = left_side_signal.is_high();
            self.right_side_light = right_side_signal.is_high();
            self.low_beam = low_beam_signal.is_high();

            (*data).lock().right_turn_signal = self.right_turn_light.status();
            (*data).lock().left_turn_signal = self.left_turn_light.status();
            //(*data).lock().
            (*data).lock().low_beam = self.low_beam;
            FreeRtos::delay_ms(30); // TODO
        }
    }
}

impl Default for LightInputSignals {
    fn default() -> Self {
        Self::new()
    }
}

/// # Safety
/// Light input task that reads all the light gpio
pub unsafe extern "C" fn light_input_task(test: *mut core::ffi::c_void) {
    println!("Light Input Signals Entered");
    FreeRtos::delay_ms(2000);

    let ptr2 = test as *mut Arc<Mutex<MailBox>>; // TODO
    let p2: &Arc<Mutex<MailBox>> = &*ptr2; // TODO

    let light_inputs: LightInputSignals = LightInputSignals::new();
    light_inputs.init_drivers(p2.clone()); // Contains Loop
}
