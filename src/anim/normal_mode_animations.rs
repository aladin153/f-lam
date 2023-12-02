use super::NormalModeAnimation;
use crate::io::angel_eye::AngelEye;
use crate::utils::timeout::ValueWithTimeout;
use crate::LightInputSignals;
use smart_leds::colors::BLACK;
use smart_leds::SmartLedsWrite;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

impl NormalModeAnimation for AngelEye {
    fn static_color(this: Arc<Mutex<Self>>, msg: Arc<RwLock<LightInputSignals>>) {
        log::info!("Static Color Animation");
        thread::spawn(move || {
            let user_pref_normal_color = (*this).lock().unwrap().normal_mode_color;
            let total_led_number = (*this).lock().unwrap().total_led_nb;
            loop {
                log::info!("Normal Mode Thread Loop");
                if let Ok(read_guard) = msg.read() {
                    if (*read_guard).left().status() == ValueWithTimeout::Timeout
                        && (*read_guard).right().status() == ValueWithTimeout::Timeout
                    {
                        let pixels = if (*read_guard).low_beam() {
                            std::iter::repeat(user_pref_normal_color).take(total_led_number)
                        } else {
                            std::iter::repeat(BLACK).take(total_led_number)
                        };

                        (*this).lock().unwrap().driver.write(pixels).unwrap();
                    }
                }

                esp_idf_hal::delay::FreeRtos::delay_ms(2000); // TODO : Change Value
            }
        });
    }
}
