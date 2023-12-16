use crate::utils::colors::Color;
use crate::MailBox;
use esp32_nimble::utilities::mutex::Mutex;
use std::sync::Arc;

pub fn handle_ble_data(ble_data: &[u8], mailbox: Arc<Mutex<MailBox>>) {
    log::info!("Ble Received Data Handler");
    log::info!("{} Bytes Received. Data : {:#?}", ble_data.len(), ble_data);

    let data_len = ble_data[0] as usize;
    if data_len != ble_data.len() {
        println!("Error: Missing Ble Data");
        log::error!("Error: Missing Ble Data");
    } else {
        // No Miising Bytes

        // TODO : Define Communication Protocol
        (*mailbox).lock().normal_mode_color = Color {
            r: ble_data[2],
            g: ble_data[3],
            b: ble_data[4],
        };

        // Save Mailbox Configuration into NVS (Save the last selected color)  // TODO
        (*mailbox).lock().saving_request = true;
    }

    (*mailbox).lock().ble_data0 = ble_data[0];
    (*mailbox).lock().ble_data1 = ble_data[1];
    (*mailbox).lock().ble_data2 = ble_data[2];
    (*mailbox).lock().ble_data3 = ble_data[3];
}
