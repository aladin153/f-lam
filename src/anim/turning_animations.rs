use crate::MailBox;
use esp32_nimble::utilities::mutex::Mutex; // todo
use esp_idf_sys::{self as _};
use std::sync::Arc;
use std::thread;

use super::*;
use crate::io::angel_eye::AngelEye;
use crate::io::angel_eye::TurnSignalStatus;
use smart_leds::SmartLedsWrite;

// todo

impl TurnLightAnimation for AngelEye {
    fn oem_bliking(this: Arc<Mutex<AngelEye>>, msg: Arc<Mutex<MailBox>>) {
        log::info!("OEM Blinking Animation");

        thread::spawn(move || {
            let color_off = (*this).lock().blinking_color_off;
            let color_on = (*this).lock().blinking_color_on;
            let total_led_number = (*this).lock().total_led_nb;
            let pref_color = (*this).lock().normal_mode_color;
            loop {
                let left = (*msg).lock().left_turn_signal;
                let right = (*msg).lock().right_turn_signal;

                let status = (*this).lock().get_turn_signal_status(left, right);

                // TODO !!!!!: add Write Wrapper
                if status == TurnSignalStatus::BothOff {
                    let pixels = std::iter::repeat(color_off).take(total_led_number);
                    (*this).lock().driver.write(pixels).unwrap();
                } else if status == TurnSignalStatus::HazardLight {
                    //log::info!("Hazard Lights ON");
                    let pixels = std::iter::repeat(color_on).take(total_led_number);
                    (*this).lock().driver.write(pixels).unwrap();
                } else if status == TurnSignalStatus::LeftNormalRightOff {
                    //log::info!("Left Normal Right Off");
                    let pixels = if (*msg).lock().low_beam {
                        std::iter::repeat(pref_color).take(total_led_number / 2)
                    } else {
                        std::iter::repeat(color_off).take(total_led_number / 2)
                    };

                    (*this)
                        .lock()
                        .driver
                        .write(
                            pixels.chain(std::iter::repeat(color_off).take(total_led_number / 2)),
                        )
                        .unwrap();
                } else if status == TurnSignalStatus::LeftNormalRightOn {
                    //log::info!("Left Normal Right On");

                    let pixels = if (*msg).lock().low_beam {
                        std::iter::repeat(pref_color).take(total_led_number / 2)
                    } else {
                        std::iter::repeat(color_off).take(total_led_number / 2)
                    };

                    (*this)
                        .lock()
                        .driver
                        .write(pixels.chain(std::iter::repeat(color_on).take(total_led_number / 2)))
                        .unwrap();
                } else if status == TurnSignalStatus::LeftOffRightNormal {
                    //log::info!("Left Off Right Normal");

                    let right_iter = if (*msg).lock().low_beam {
                        std::iter::repeat(pref_color).take(total_led_number / 2)
                    } else {
                        std::iter::repeat(color_off).take(total_led_number / 2)
                    };

                    (*this)
                        .lock()
                        .driver
                        .write(
                            std::iter::repeat(color_off)
                                .take(total_led_number / 2)
                                .chain(right_iter),
                        )
                        .unwrap();
                } else if status == TurnSignalStatus::LeftOffRightOn {
                    //log::info!("Left Off Right On");
                    let pixels = std::iter::repeat(color_off)
                        .take(total_led_number / 2)
                        .chain(std::iter::repeat(color_on).take(total_led_number / 2));
                    (*this).lock().driver.write(pixels).unwrap();
                } else if status == TurnSignalStatus::LeftOnRightNormal {
                    //log::info!("Left On Right Normal");

                    let right_iter = if (*msg).lock().low_beam {
                        std::iter::repeat(pref_color).take(total_led_number / 2)
                    } else {
                        std::iter::repeat(color_off).take(total_led_number / 2)
                    };

                    (*this)
                        .lock()
                        .driver
                        .write(
                            std::iter::repeat(color_on)
                                .take(total_led_number / 2)
                                .chain(right_iter),
                        )
                        .unwrap();
                } else if status == TurnSignalStatus::LeftOnRightOff {
                    //log::info!("Left On Right Off");
                    let pixels = std::iter::repeat(color_on)
                        .take(total_led_number / 2)
                        .chain(std::iter::repeat(color_off).take(total_led_number / 2));
                    (*this).lock().driver.write(pixels).unwrap();
                } else {
                    //log::info!("Condition handled by Normal Mode Animation");
                }

                esp_idf_hal::delay::FreeRtos::delay_ms(20);
            }
        });
    }
}
