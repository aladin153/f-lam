use crate::{bluetooth::handler::handle_ble_data, MailBox};
use esp32_nimble::utilities::mutex::Mutex; // todo
use esp_idf_hal::delay::FreeRtos;
use esp_idf_sys::{self as _};
use std::sync::Arc;

use esp32_nimble::{utilities::BleUuid, uuid128, BLEDevice, NimbleProperties};
use std::str;

const SERVICE_UUID: BleUuid = uuid128!("55b399a3-2688-4166-bb59-f1d0ddfdf8e6");
const STATIC_CHARACTERISTIC_UUID: BleUuid = uuid128!("d4e0e0d0-1a2b-11e9-ab14-d663bd873d93");
const NOTIFYING_CHARACTERISTIC_UUID: BleUuid = uuid128!("a3c87500-8ed3-4bdf-8a39-a01bebede295");
const WRITABLE_CHARACTERISTIC_UUID: BleUuid = uuid128!("3c9a3f00-8ed3-4bdf-8a39-a01bebede295");
const ADVERTISING_NAME: &str = "Alaeddine ZAYEN";

#[derive(Clone)]
pub struct Ble {
    pub data: [u8; 50], // TODO
}

impl Ble {
    pub fn new() -> Self {
        Self {
            data: [0; 50], // Todo
        }
    }

    pub fn init(&self, mailbox: Arc<Mutex<MailBox>>) {
        ::log::info!("Init ble");

        let ble_device = BLEDevice::take();

        let server = ble_device.get_server();
        server.on_connect(|server, desc| {
            ::log::info!("Client connected");

            server
                .update_conn_params(desc.conn_handle, 24, 48, 0, 60)
                .unwrap();

            ::log::info!("Multi-connect support: start advertising");
            ble_device.get_advertising().start().unwrap();
        });
        server.on_disconnect(|_desc, reason| {
            ::log::info!("Client disconnected ({:X})", reason);
        });
        let service = server.create_service(SERVICE_UUID);

        // A static characteristic.
        let static_characteristic = service
            .lock()
            .create_characteristic(STATIC_CHARACTERISTIC_UUID, NimbleProperties::READ);
        static_characteristic
            .lock()
            .set_value("Hello, world!".as_bytes());

        // A characteristic that notifies every second.
        let notifying_characteristic = service.lock().create_characteristic(
            NOTIFYING_CHARACTERISTIC_UUID,
            NimbleProperties::READ | NimbleProperties::NOTIFY,
        );
        notifying_characteristic.lock().set_value(b"Initial value.");

        // A writable characteristic.
        let writable_characteristic = service.lock().create_characteristic(
            WRITABLE_CHARACTERISTIC_UUID,
            NimbleProperties::READ | NimbleProperties::WRITE,
        );
        writable_characteristic
            .lock()
            .on_read(move |_, _| {
                ::log::info!("Read from writable characteristic.");
            })
            .on_write(move |args| {
                println!("Wrote to writable characteristic: {:?}", args.recv_data);

                handle_ble_data(args.recv_data, mailbox.clone()); // TODO : Rework Handler
            });

        let ble_advertising = ble_device.get_advertising();

        ble_advertising
            .name(ADVERTISING_NAME)
            .add_service_uuid(SERVICE_UUID);

        ble_advertising.start().expect("ALADIN UNWRAP"); // TODO
    }
}

impl Default for Ble {
    fn default() -> Self {
        Self::new()
    }
}

/// # Safety
/// Initialize ble and wait for received data
pub unsafe extern "C" fn ble_task(test: *mut core::ffi::c_void) {
    println!("Bluetooth Low Energy Entered");
    FreeRtos::delay_ms(2000);

    let ptr2 = test as *mut Arc<Mutex<MailBox>>; // TODO
    let p2: &Arc<Mutex<MailBox>> = &*ptr2; // TODO

    let ble: Ble = Ble::new();
    ble.init(p2.clone()); // Contains Loop
    loop {
        FreeRtos::delay_ms(1000);
    }
}
