//use anyhow::Ok;

use esp_idf_svc::log::EspLogger;
use esp_idf_svc::nvs::*;
use postcard::{from_bytes, to_vec};

use serde::{Deserialize, Serialize, __private::Ok};

use log::info;

use crate::utils::calib::Calib;

#[derive(Serialize, Deserialize, Debug)]
struct StructToBeStored<'a> {
    some_bytes: &'a [u8],
    a_str: &'a str,
    a_number: i16,
}

pub trait Storage {
    fn save_element(&self) -> anyhow::Result<()>;
    fn read_element(&self) -> anyhow::Result<()>;
    fn save_all_data(&self, nvs: &mut EspNvs<NvsDefault>) -> anyhow::Result<()>;
    fn read_all_saved_data(&self) -> anyhow::Result<()>;
}

impl Storage for Calib {
    fn save_element(&self) -> anyhow::Result<()> {
        todo!()
    }

    fn read_element(&self) -> anyhow::Result<()> {
        todo!()
    }

    fn save_all_data(&self, nvs: &mut EspNvs<NvsDefault>) -> anyhow::Result<()> {
        let key_raw_struct: &str = "test_raw_struct";
        {
            let key_raw_struct_data = StructToBeStored {
                some_bytes: &[1, 2, 3, 4],
                a_str: "I'm a str inside a struct!",
                a_number: 42,
            };

            match nvs.set_raw(
                key_raw_struct,
                &to_vec::<StructToBeStored, 100>(&key_raw_struct_data).unwrap(),
            ) {
                Ok(_) => info!("Key {} updated", key_raw_struct),
                Err(e) => info!("key {} not updated {:?}", key_raw_struct, e),
            };
        }

        Ok(())
    }

    fn read_all_saved_data(&self) -> anyhow::Result<()> {
        todo!()
    }
}

/*  let key_raw_u8 = "test_raw_u8";
{
    let key_raw_u8_data: &[u8] = &[42];

    match nvs.set_raw(key_raw_u8, key_raw_u8_data) {
        Ok(_) => info!("Key updated"),
        // You can find the meaning of the error codes in the output of the error branch in:
        // https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/error-codes.html
        Err(e) => info!("Key not updated {:?}", e),
    };
}

{
    let key_raw_u8_data: &mut [u8] = &mut [u8::MAX];

    match nvs.get_raw(key_raw_u8, key_raw_u8_data) {
        Ok(v) => match v {
            Some(vv) => info!("{:?} = {:?}", key_raw_u8, vv),
            None => todo!(),
        },
        Err(e) => info!("Couldn't get key {} because{:?}", key_raw_u8, e),
    };
}

let key_raw_str: &str = "test_raw_str";
{
    let key_raw_str_data = "Hello from the NVS (I'm raw)!";

    match nvs.set_raw(
        key_raw_str,
        &to_vec::<&str, 100>(&key_raw_str_data).unwrap(),
    ) {
        Ok(_) => info!("Key {} updated", key_raw_str),
        Err(e) => info!("Key {} not updated {:?}", key_raw_str, e),
    };
}

{
    let key_raw_str_data: &mut [u8] = &mut [0; 100];

    match nvs.get_raw(key_raw_str, key_raw_str_data) {
        Ok(v) => {
            if let Some(the_str) = v {
                info!("{:?} = {:?}", key_raw_str, from_bytes::<&str>(the_str));
            }
        }
        Err(e) => info!("Couldn't get key {} because {:?}", key_raw_str, e),
    };
}

let key_raw_struct: &str = "test_raw_struct";
{
    let key_raw_struct_data = StructToBeStored {
        some_bytes: &[1, 2, 3, 4],
        a_str: "I'm a str inside a struct!",
        a_number: 42,
    };

    match nvs.set_raw(
        key_raw_struct,
        &to_vec::<StructToBeStored, 100>(&key_raw_struct_data).unwrap(),
    ) {
        Ok(_) => info!("Key {} updated", key_raw_str),
        Err(e) => info!("key {} not updated {:?}", key_raw_str, e),
    };
}

{
    let key_raw_struct_data: &mut [u8] = &mut [0; 100];

    match nvs.get_raw(key_raw_struct, key_raw_struct_data) {
        Ok(v) => {
            if let Some(the_struct) = v {
                info!(
                    "{:?} = {:?}",
                    key_raw_str,
                    from_bytes::<StructToBeStored>(the_struct)
                )
            }
        }
        Err(e) => info!("Couldn't get key {} because {:?}", key_raw_str, e),
    };
}

Ok(()) */
