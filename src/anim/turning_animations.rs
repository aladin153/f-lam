use super::*;
use crate::io::angel_eye::AngelEye;
use crate::io::angel_eye::TurnSignalStatus;
use crate::LightInputSignals;
use smart_leds::SmartLedsWrite;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

impl TurnLightAnimation for AngelEye {
    fn oem_bliking(this: Arc<Mutex<AngelEye>>, msg: Arc<RwLock<LightInputSignals>>) {
        log::info!("OEM Blinking Animation");

        thread::spawn(move || {
            let color_off = (*this).lock().unwrap().blinking_color_off;
            let color_on = (*this).lock().unwrap().blinking_color_on;
            let total_led_number = (*this).lock().unwrap().total_led_nb;
            let pref_color = (*this).lock().unwrap().normal_mode_color;
            loop {
                log::info!("Turning Lights Thread");
                if let Ok(read_guard) = msg.read() {
                    let status = (*this).lock().unwrap().get_turn_signal_status(
                        (*read_guard).left().status(),
                        (*read_guard).right().status(),
                    );

                    // TODO !!!!!: add Write Wrapper
                    if status == TurnSignalStatus::BothOff {
                        log::info!("Turning Lights OFF");
                        let pixels = std::iter::repeat(color_off).take(total_led_number);
                        (*this).lock().unwrap().driver.write(pixels).unwrap();
                    } else if status == TurnSignalStatus::HazardLight {
                        log::info!("Hazard Lights ON");
                        let pixels = std::iter::repeat(color_on).take(total_led_number);
                        (*this).lock().unwrap().driver.write(pixels).unwrap();
                    } else if status == TurnSignalStatus::LeftNormalRightOff {
                        log::info!("Left Normal Right Off");
                        let pixels = if (*read_guard).low_beam() {
                            std::iter::repeat(pref_color).take(total_led_number / 2)
                        } else {
                            std::iter::repeat(color_off).take(total_led_number / 2)
                        };

                        (*this)
                            .lock()
                            .unwrap()
                            .driver
                            .write(
                                pixels
                                    .chain(std::iter::repeat(color_off).take(total_led_number / 2)),
                            )
                            .unwrap();
                    } else if status == TurnSignalStatus::LeftNormalRightOn {
                        log::info!("Left Normal Right On");

                        let pixels = if (*read_guard).low_beam() {
                            std::iter::repeat(pref_color).take(total_led_number / 2)
                        } else {
                            std::iter::repeat(color_off).take(total_led_number / 2)
                        };

                        (*this)
                            .lock()
                            .unwrap()
                            .driver
                            .write(
                                pixels
                                    .chain(std::iter::repeat(color_on).take(total_led_number / 2)),
                            )
                            .unwrap();
                    } else if status == TurnSignalStatus::LeftOffRightNormal {
                        log::info!("Left Off Right Normal");

                        let right_iter = if (*read_guard).low_beam() {
                            std::iter::repeat(pref_color).take(total_led_number / 2)
                        } else {
                            std::iter::repeat(color_off).take(total_led_number / 2)
                        };

                        (*this)
                            .lock()
                            .unwrap()
                            .driver
                            .write(
                                std::iter::repeat(color_off)
                                    .take(total_led_number / 2)
                                    .chain(right_iter),
                            )
                            .unwrap();
                    } else if status == TurnSignalStatus::LeftOffRightOn {
                        log::info!("Left Off Right On");
                        let pixels = std::iter::repeat(color_off)
                            .take(total_led_number / 2)
                            .chain(std::iter::repeat(color_on).take(total_led_number / 2));
                        (*this).lock().unwrap().driver.write(pixels).unwrap();
                    } else if status == TurnSignalStatus::LeftOnRightNormal {
                        log::info!("Left On Right Normal");

                        let right_iter = if (*read_guard).low_beam() {
                            std::iter::repeat(pref_color).take(total_led_number / 2)
                        } else {
                            std::iter::repeat(color_off).take(total_led_number / 2)
                        };

                        (*this)
                            .lock()
                            .unwrap()
                            .driver
                            .write(
                                std::iter::repeat(color_on)
                                    .take(total_led_number / 2)
                                    .chain(right_iter),
                            )
                            .unwrap();
                    } else if status == TurnSignalStatus::LeftOnRightOff {
                        log::info!("Left On Right Off");
                        let pixels = std::iter::repeat(color_on)
                            .take(total_led_number / 2)
                            .chain(std::iter::repeat(color_off).take(total_led_number / 2));
                        (*this).lock().unwrap().driver.write(pixels).unwrap();
                    } else {
                        log::info!("Condition handled by Normal Mode Animation");
                    }
                }
                esp_idf_hal::delay::FreeRtos::delay_ms(1000); // TODO Change Value
            }
        });
    }
}
