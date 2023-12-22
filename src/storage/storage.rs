// TODO Fix all the logs.
// Code Cleanup ( define const values, return errors, error handling in main)
// Define and implement state machine on main function
// Replace fixed struct size with mem::size_of

use crate::storage::calib::Calib;
use crate::storage::config::Config;
use anyhow::{self};
use esp_idf_svc::nvs::*;
use postcard::{from_bytes, to_vec};
use serde::__private::Ok;
use std::mem;

use log::info;

pub trait Storage {
    const STRUCT_SIZE: usize;
    fn save_all_data(&self, nvs: &mut EspNvs<NvsDefault>) -> anyhow::Result<()>;
    fn read_all_saved_data(&mut self, nvs: &mut EspNvs<NvsDefault>) -> anyhow::Result<(), ()>;
}

impl Storage for Calib {
    const STRUCT_SIZE: usize = mem::size_of::<Calib>();
    //let x = Calib.STRUCT_SIZE + 5;
    fn save_all_data(&self, nvs: &mut EspNvs<NvsDefault>) -> anyhow::Result<()> {
        // TODO

        let key_raw_struct: &str = "calib_struct"; // TODO : const

        match nvs.set_raw(
            key_raw_struct,
            &to_vec::<Calib, { Calib::STRUCT_SIZE }>(&self).unwrap(),
        ) {
            Ok(_) => info!("Key {} updated", key_raw_struct),
            Err(e) => info!("key {} not updated {:?}", key_raw_struct, e),
        };

        Ok(())
    }

    fn read_all_saved_data(&mut self, nvs: &mut EspNvs<NvsDefault>) -> anyhow::Result<(), ()> {
        // TODO

        println!("Read Calib from NVS"); // TODO
        let key_raw_struct: &str = "calib_struct"; // TODO COnst
        let key_raw_struct_data: &mut [u8] = &mut [0; Calib::STRUCT_SIZE];

        match nvs.get_raw(key_raw_struct, key_raw_struct_data) {
            Ok(v) => {
                if let Some(the_struct) = v {
                    println!(
                        "{:?} = {:?}",
                        key_raw_struct,
                        from_bytes::<Calib>(the_struct)
                    );

                    let mut x = from_bytes::<Calib>(the_struct);
                    println!("here is the struct X: {:?}", x);
                    if let Ok(calib) = &mut x {
                        // TODO self = calib directly
                        self.angel_eye_total_led = calib.angel_eye_total_led.clone();
                        self.turn_signal_timeout = calib.turn_signal_timeout.clone();
                        return Ok(());
                    } else {
                        return Err(());
                    }
                }
            }
            Err(e) => println!("Couldn't get key {} because {:?}", key_raw_struct, e),
        };
        Err(())
    }
}

impl Storage for Config {
    const STRUCT_SIZE: usize = mem::size_of::<Config>();
    fn save_all_data(&self, nvs: &mut EspNvs<NvsDefault>) -> anyhow::Result<()> {
        // TODO

        let key_raw_struct: &str = "config_struct";

        match nvs.set_raw(
            key_raw_struct,
            &to_vec::<Config, { Config::STRUCT_SIZE }>(&self).unwrap(), // TODO Replace 100 with const
        ) {
            Ok(_) => info!("Key {} updated", key_raw_struct),
            Err(e) => info!("key {} not updated {:?}", key_raw_struct, e),
        };

        Ok(())
    }

    fn read_all_saved_data(&mut self, nvs: &mut EspNvs<NvsDefault>) -> anyhow::Result<(), ()> {
        // TODO

        println!("Read Config from NVS"); // TODO
        let key_raw_struct: &str = "config_struct"; // TODO COnst
        let key_raw_struct_data: &mut [u8] = &mut [0; Config::STRUCT_SIZE];

        match nvs.get_raw(key_raw_struct, key_raw_struct_data) {
            Ok(v) => {
                if let Some(the_struct) = v {
                    println!(
                        "{:?} = {:?}",
                        key_raw_struct,
                        from_bytes::<Config>(the_struct)
                    );

                    let mut x = from_bytes::<Config>(the_struct);
                    println!("here is the struct X: {:?}", x);
                    if let Ok(config) = &mut x {
                        // TODO self = config directly
                        self.normal_mode_color = config.normal_mode_color.clone();
                        self.blinking_color_on = config.blinking_color_on.clone();
                        self.blinking_color_off = config.blinking_color_off.clone();
                        self.turn_light_anim = config.turn_light_anim;
                        self.normal_mode_anim = config.normal_mode_anim;
                        return Ok(());
                    } else {
                        return Err(());
                    }
                }
            }
            Err(e) => println!("Couldn't get key {} because {:?}", key_raw_struct, e),
        };
        Err(())
    }
}
