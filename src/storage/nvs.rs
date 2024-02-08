// TODO Fix all the logs.
// Code Cleanup ( return errors, error handling in main)
// Define and implement state machine on main function

use crate::storage::config::Config;
use crate::{storage::calib::Calib, utils::error::CommonError};
use anyhow::{self};
use esp_idf_svc::nvs::*;
use postcard::{from_bytes, to_vec};
use serde::__private::Ok;
use std::mem;

pub trait Nvs {
    const STRUCT_SIZE: usize;
    const KEY_RAW_STRUCT: &'static str;
    fn save_all_data(&self, nvs: &mut EspNvs<NvsDefault>) -> anyhow::Result<(), CommonError>;
    fn read_all_saved_data(
        &mut self,
        nvs: &mut EspNvs<NvsDefault>,
    ) -> anyhow::Result<(), CommonError>;
}

impl Nvs for Calib {
    const STRUCT_SIZE: usize = mem::size_of::<Calib>();
    const KEY_RAW_STRUCT: &'static str = "calib_struct";
    fn save_all_data(&self, nvs: &mut EspNvs<NvsDefault>) -> anyhow::Result<(), CommonError> {
        match nvs.set_raw(
            Calib::KEY_RAW_STRUCT,
            &to_vec::<Calib, { Calib::STRUCT_SIZE }>(self).unwrap(),
        ) {
            Ok(_) => Ok(()),
            Err(_e) => Err(CommonError::Test1), // TODO
        }
    }

    fn read_all_saved_data(
        &mut self,
        nvs: &mut EspNvs<NvsDefault>,
    ) -> anyhow::Result<(), CommonError> {
        // TODO

        println!("Read Calib from NVS"); // TODO
        let key_raw_struct_data: &mut [u8] = &mut [0; Calib::STRUCT_SIZE];

        match nvs.get_raw(Calib::KEY_RAW_STRUCT, key_raw_struct_data) {
            Ok(v) => {
                if let Some(the_struct) = v {
                    println!(
                        "{:?} = {:?}",
                        Calib::KEY_RAW_STRUCT,
                        from_bytes::<Calib>(the_struct)
                    );

                    let mut x = from_bytes::<Calib>(the_struct);
                    println!("here is the struct X: {:?}", x);
                    if let Ok(calib) = &mut x {
                        // TODO self = calib directly
                        self.angel_eye_total_led = calib.angel_eye_total_led;
                        self.turn_signal_timeout = calib.turn_signal_timeout;
                        return Ok(());
                    } else {
                        return Err(CommonError::Test1); // TODO
                    }
                }
            }
            Err(e) => println!("Couldn't get key {} because {:?}", Calib::KEY_RAW_STRUCT, e),
        };
        Err(CommonError::Test1) // TODO
    }
}

impl Nvs for Config {
    const STRUCT_SIZE: usize = mem::size_of::<Config>();
    const KEY_RAW_STRUCT: &'static str = "config_struct";
    fn save_all_data(&self, nvs: &mut EspNvs<NvsDefault>) -> anyhow::Result<(), CommonError> {
        // TODO

        match nvs.set_raw(
            Config::KEY_RAW_STRUCT,
            &to_vec::<Config, { Config::STRUCT_SIZE }>(self).unwrap(),
        ) {
            Ok(_) => Ok(()),
            Err(_e) => Err(CommonError::Test1), // TODO
        }
        //nvs.set_raw(
        //    Config::KEY_RAW_STRUCT,
        //    &to_vec::<Config, { Config::STRUCT_SIZE }>(self).unwrap()
        //)?;
    }

    fn read_all_saved_data(
        &mut self,
        nvs: &mut EspNvs<NvsDefault>,
    ) -> anyhow::Result<(), CommonError> {
        // TODO

        println!("Read Config from NVS"); // TODO
        let key_raw_struct_data: &mut [u8] = &mut [0; Config::STRUCT_SIZE];
        println!("Aladin"); // TODO
        match nvs.get_raw(Config::KEY_RAW_STRUCT, key_raw_struct_data) {
            Ok(v) => {
                println!("ALADIN OK"); // TODO
                if let Some(the_struct) = v {
                    println!("ALADIN 2"); // TODO
                    println!(
                        "{:?} = {:?}",
                        Config::KEY_RAW_STRUCT,
                        from_bytes::<Config>(the_struct)
                    );
                    println!("ALADIN 3"); // TODO
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
                        return Err(CommonError::Test1); //  TODO
                    }
                }
            }
            Err(e) => println!(
                "Couldn't get key {} because {:?}",
                Config::KEY_RAW_STRUCT,
                e
            ),
        };
        println!("ALADIN KO"); // TODO
        Err(CommonError::Test1) // TODO
    }
}
