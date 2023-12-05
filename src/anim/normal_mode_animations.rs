use crate::MailBox;
use esp32_nimble::utilities::mutex::Mutex;
use esp_idf_sys::{self as _};
use std::sync::Arc;
use std::thread;

use super::NormalModeAnimation;
use crate::io::angel_eye::AngelEye;
use crate::utils::timeout::ValueWithTimeout;
use smart_leds::colors::BLACK;
use smart_leds::SmartLedsWrite;

impl NormalModeAnimation for AngelEye {
    fn static_color(this: Arc<Mutex<Self>>, msg: Arc<Mutex<MailBox>>) {
        println!("Static Color Animation");
        log::info!("Static Color Animation");
        thread::spawn(move || {
            println!("Normal Mode Thread Start");
            let total_led_number = (*this).lock().total_led_nb;
            loop {
                (*this).lock().normal_mode_color = (*msg).lock().normal_mode_color;

                if (*msg).lock().left_turn_signal == ValueWithTimeout::Timeout
                    && (*msg).lock().right_turn_signal == ValueWithTimeout::Timeout
                {
                    let pixels = if (*msg).lock().low_beam {
                        std::iter::repeat((*msg).lock().normal_mode_color).take(total_led_number)
                    } else {
                        std::iter::repeat(BLACK).take(total_led_number)
                    };

                    (*this).lock().driver.write(pixels).unwrap(); // Todo : Add Wrapper
                }

                esp_idf_hal::delay::FreeRtos::delay_ms(30);
            }
        });
    }
}
