use crate::utils::colors::{Color, BLACK};
use crate::MailBox;
use esp32_nimble::utilities::mutex::Mutex;
use esp_idf_hal::ledc::config::TimerConfig;
use esp_idf_hal::ledc::LedcTimerDriver;
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::ledc::LedcDriver;
use esp_idf_svc::hal::prelude::*;

use std::borrow::{Borrow, BorrowMut};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum LedWiringConfig {
    // To do add Default
    CommonAnode,
    CommonCathode,
}

#[derive(Debug, Clone)]
pub struct DevilEye {
    pub config: LedWiringConfig,
    pub color: Color,
    pub brightness: u8,
}

impl DevilEye {
    pub fn new() -> Self {
        DevilEye {
            config: LedWiringConfig::CommonAnode, // TODO
            color: Color::from(BLACK),
            brightness: 100,
        }
    }

    pub fn init_drivers(mut self, data: Arc<Mutex<MailBox>>) {
        println!("Setting up PWM output channels");

        let mut binding = (*data).lock();
        let peripherals: &mut Peripherals = binding.peripherals.borrow_mut();

        let config = TimerConfig::new().frequency(25.kHz().into());
        unsafe {
            let timer = Arc::new(
                LedcTimerDriver::new(peripherals.ledc.timer0.clone_unchecked(), &config)
                    .unwrap_or_else(|error| {
                        panic!("Peripherals initialization issue : {:?}", error);
                    }),
            );
            let mut red_channel = LedcDriver::new(
                peripherals.ledc.channel0.clone_unchecked(),
                timer.clone(),
                peripherals.pins.gpio4.clone_unchecked(),
            )
            .unwrap_or_else(|error| {
                panic!("Peripherals initialization issue : {:?}", error);
            });

            let mut green_channel = LedcDriver::new(
                peripherals.ledc.channel1.clone_unchecked(),
                timer.clone(),
                peripherals.pins.gpio5.clone_unchecked(),
            )
            .unwrap_or_else(|error| {
                panic!("Peripherals initialization issue : {:?}", error);
            });

            let mut blue_channel = LedcDriver::new(
                peripherals.ledc.channel2.clone_unchecked(),
                timer,
                peripherals.pins.gpio2.clone_unchecked(),
            )
            .unwrap_or_else(|error| {
                panic!("Peripherals initialization issue : {:?}", error);
            });

            println!("Spawning PWM threads");

            log::error!("Channel 0 Max duty = {}", red_channel.get_max_duty());
            log::error!("Channel 1 Max duty = {}", green_channel.get_max_duty());
            log::error!("Channel 2 Max duty = {}", blue_channel.get_max_duty());

            let _ = red_channel.set_duty(red_channel.get_max_duty());
            let _ = green_channel.set_duty(green_channel.get_max_duty());
            let _ = blue_channel.set_duty(blue_channel.get_max_duty());
            FreeRtos::delay_ms(5000); // TODO
            let mut i = 0.0;
            loop {
                let _ = red_channel.set_duty((red_channel.get_max_duty() as f64 * i) as u32);
                let _ = green_channel.set_duty((green_channel.get_max_duty() as f64 * i) as u32);
                let _ = blue_channel.set_duty((blue_channel.get_max_duty() as f64 * i) as u32);
                i += 0.1;
                if i >= 1.0 {
                    i = 0.0;
                }
                FreeRtos::delay_ms(200); // TODO
            }
        }
    }
}

/// # Safety
/// Initialize PWM outputs and control devil eyes
pub unsafe extern "C" fn devil_eye_task(test: *mut core::ffi::c_void) {
    println!("Devil Eye Task Entered");
    FreeRtos::delay_ms(2000);

    let ptr2 = test as *mut Arc<Mutex<MailBox>>; // TODO
    let p2: &Arc<Mutex<MailBox>> = &*ptr2; // TODO

    let devil_eye: DevilEye = DevilEye::new();
    devil_eye.init_drivers(p2.clone()); // Contains Loop
}

impl Default for DevilEye {
    fn default() -> Self {
        Self::new()
    }
}
