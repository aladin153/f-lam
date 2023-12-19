use crate::MailBox;
use esp32_nimble::utilities::mutex::Mutex;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_sys::{self as _};
use std::sync::Arc;

use crate::anim::NormalModeAnimation;
use crate::anim::TurnLightAnimation;
use crate::utils::timeout::ValueWithTimeout;
use smart_leds::colors::*;
use smart_leds::*;
use ws2812_esp32_rmt_driver::{LedPixelEsp32Rmt, Ws2812Esp32Rmt};

const RMT_CHANNEL_NUM: u8 = 0;
const WS2812B_PIN: u32 = 27;

#[derive(Debug, PartialEq)]
pub enum TurnSignalStatus {
    BothOff,
    LeftOffRightOn,
    LeftOffRightNormal,
    LeftOnRightOff,
    LeftOnRightNormal,
    HazardLight,
    LeftNormalRightOff,
    LeftNormalRightOn,
    BothNormal,
}

#[allow(clippy::type_complexity)]
pub struct AngelEye {
    // Chanel
    // Pin Number
    pub normal_mode_color: RGB<u8>, // TODO : Get parameters from Config
    pub blinking_color_on: RGB<u8>,
    pub blinking_color_off: RGB<u8>,
    pub total_led_nb: usize, // TODO : Get parameters from Calib
    // Some Configs to be added
    pub turn_light_anim: fn(Arc<Mutex<AngelEye>>, Arc<Mutex<MailBox>>),
    pub normal_mode_anim: fn(Arc<Mutex<AngelEye>>, Arc<Mutex<MailBox>>),
    pub driver: LedPixelEsp32Rmt<
        RGB<u8>,
        ws2812_esp32_rmt_driver::driver::color::LedPixelColorImpl<3, 1, 0, 2, 255>,
    >,
}

impl AngelEye {
    pub fn new() -> Self {
        let angel_eye_driver: LedPixelEsp32Rmt<
            RGB<u8>,
            ws2812_esp32_rmt_driver::driver::color::LedPixelColorImpl<3, 1, 0, 2, 255>,
        > = Ws2812Esp32Rmt::new(RMT_CHANNEL_NUM, WS2812B_PIN).unwrap();
        Self {
            driver: angel_eye_driver,
            total_led_nb: 45, // TODO : Get parameters from Calib
            normal_mode_color: RED,
            blinking_color_on: ORANGE,
            blinking_color_off: BLACK,
            turn_light_anim: <crate::io::angel_eye::AngelEye as TurnLightAnimation>::oem_bliking, // TODO : From Config (Load the saved animation)  as ????
            normal_mode_anim: <crate::io::angel_eye::AngelEye as NormalModeAnimation>::static_color, // TODO : From Config as ??????
        }
    }

    pub fn get_turn_signal_status(
        &self,
        left: ValueWithTimeout,
        right: ValueWithTimeout,
    ) -> TurnSignalStatus {
        match (left, right) {
            (ValueWithTimeout::On, ValueWithTimeout::On) => TurnSignalStatus::HazardLight, // TODO : Check Priority
            (ValueWithTimeout::Off, ValueWithTimeout::Off) => TurnSignalStatus::BothOff,
            (ValueWithTimeout::Off, ValueWithTimeout::On) => TurnSignalStatus::LeftOffRightOn,
            (ValueWithTimeout::Off, ValueWithTimeout::Timeout) => {
                TurnSignalStatus::LeftOffRightNormal
            }
            (ValueWithTimeout::On, ValueWithTimeout::Off) => TurnSignalStatus::LeftOnRightOff,
            (ValueWithTimeout::On, ValueWithTimeout::Timeout) => {
                TurnSignalStatus::LeftOnRightNormal
            }
            (ValueWithTimeout::Timeout, ValueWithTimeout::Off) => {
                TurnSignalStatus::LeftNormalRightOff
            }
            (ValueWithTimeout::Timeout, ValueWithTimeout::On) => {
                TurnSignalStatus::LeftNormalRightOn
            }
            (ValueWithTimeout::Timeout, ValueWithTimeout::Timeout) => TurnSignalStatus::BothNormal,
        }
    }

    pub fn play_turn_animation(&self, this: Arc<Mutex<AngelEye>>, msg: Arc<Mutex<MailBox>>) {
        (self.turn_light_anim)(this, msg);
    }

    pub fn play_normal_mode_animation(&self, this: Arc<Mutex<AngelEye>>, msg: Arc<Mutex<MailBox>>) {
        (self.normal_mode_anim)(this, msg);
    }
}

impl Default for AngelEye {
    fn default() -> Self {
        Self::new()
    }
}

/// # Safety
/// Initialize RMT driver and control angel eyes
pub unsafe extern "C" fn angel_eye_task(test: *mut core::ffi::c_void) {
    println!("Angel Eye Entered");

    let ptr2 = test as *mut Arc<Mutex<MailBox>>; // TODO
    let p2: &Arc<Mutex<MailBox>> = &*ptr2; // TODO

    let angel_eye = Arc::new(Mutex::new(AngelEye::new()));

    log::info!("Starting angel eyes animation threads !!!");

    (*angel_eye)
        .lock()
        .play_turn_animation(angel_eye.clone(), p2.clone());

    (*angel_eye)
        .lock()
        .play_normal_mode_animation(angel_eye.clone(), p2.clone());

    loop {
        FreeRtos::delay_ms(30);
    }
}
