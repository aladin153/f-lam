use crate::MailBox;
use esp32_nimble::utilities::mutex::Mutex;
use esp_idf_sys::{self as _};
use std::sync::Arc;
use std::thread;

use super::NormalModeAnimation;
use crate::io::angel_eye::AngelEye;
use crate::utils::colors::BLACK;
use crate::utils::timeout::ValueWithTimeout;
use esp_idf_hal::sys::esp_random;
use smart_leds::hsv::hsv2rgb;
use smart_leds::hsv::Hsv;
use smart_leds::SmartLedsWrite;

impl NormalModeAnimation for AngelEye {
    fn static_color(this: Arc<Mutex<Self>>, msg: Arc<Mutex<MailBox>>) {
        println!("Static Color Animation");
        log::info!("Static Color Animation");
        thread::spawn(move || {
            println!("Normal Mode Thread Start");
            let total_led_number = (*this).lock().total_led_nb;
            loop {
                (*this).lock().normal_mode_color = (*msg).lock().normal_mode_color.to_rgb8();

                if (*msg).lock().left_turn_signal == ValueWithTimeout::Timeout
                    && (*msg).lock().right_turn_signal == ValueWithTimeout::Timeout
                {
                    let pixels = if (*msg).lock().low_beam {
                        std::iter::repeat((*msg).lock().normal_mode_color.to_rgb8())
                            .take(total_led_number)
                    } else {
                        std::iter::repeat(BLACK).take(total_led_number)
                    };

                    (*this).lock().driver.write(pixels).unwrap(); // Todo : Add Wrapper
                }

                esp_idf_hal::delay::FreeRtos::delay_ms(30);
            }
        });
    }

    fn hsv_rainbow(this: Arc<Mutex<Self>>, msg: Arc<Mutex<MailBox>>) {
        println!("HSV Rainbow Animation");
        log::info!("HSV Rainbow Animation");
        thread::spawn(move || {
            // TODO : To be moved to another method -----> Just for test
            let mut hue = unsafe { esp_random() } as u8;
            loop {
                if (*msg).lock().left_turn_signal == ValueWithTimeout::Timeout
                    && (*msg).lock().right_turn_signal == ValueWithTimeout::Timeout
                {
                    let pixels = std::iter::repeat(hsv2rgb(Hsv {
                        hue,
                        sat: 255,
                        val: 8,
                    }))
                    .take(45);
                    (*this).lock().driver.write(pixels).unwrap(); // Todo : Add Wrapper

                    esp_idf_hal::delay::FreeRtos::delay_ms(100);

                    hue = hue.wrapping_add(10);
                }
            }
        });
    }
}
