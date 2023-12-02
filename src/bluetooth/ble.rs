
use esp32_nimble::{utilities::BleUuid, uuid128, BLEDevice, NimbleProperties};
use std::str;

const SERVICE_UUID: BleUuid = uuid128!("55b399a3-2688-4166-bb59-f1d0ddfdf8e6");
const STATIC_CHARACTERISTIC_UUID: BleUuid = uuid128!("d4e0e0d0-1a2b-11e9-ab14-d663bd873d93");
const NOTIFYING_CHARACTERISTIC_UUID: BleUuid = uuid128!("a3c87500-8ed3-4bdf-8a39-a01bebede295");
const WRITABLE_CHARACTERISTIC_UUID: BleUuid = uuid128!("3c9a3f00-8ed3-4bdf-8a39-a01bebede295");
const ADVERTISING_NAME: &str = "Alaeddine ZAYEN";

pub struct Ble {}

impl Ble {
    pub fn ble_data_callback(&mut self, data: &[u8]) {
        log::info!("handle_ble_callback");
        let data = str::from_utf8(data);
        let res = data.unwrap();
        log::info!("BLE Received Data = {}", data.unwrap());
        if res.eq("LEDON") {
            self.drivers.board_led.set_high().expect("Error");
        } else if res.eq("LEDOFF") {
            self.drivers.board_led.set_low().expect("error");
        }
    }

    pub fn new(drivers: Drivers) -> Self {
        Self { drivers }
    }
    pub fn init(mut self) {
        // TODO
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
                ::log::info!("Wrote to writable characteristic: {:?}", args.recv_data);
                self.ble_data_callback(args.recv_data);
            });

        let ble_advertising = ble_device.get_advertising();
        ble_advertising
            .name(ADVERTISING_NAME)
            .add_service_uuid(SERVICE_UUID);

        ble_advertising.start().unwrap();
    }
}
