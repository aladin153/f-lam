use crate::io::angel_eye::AngelEye;
use crate::MailBox;
use esp32_nimble::utilities::mutex::Mutex; // todo
use esp_idf_sys::{self as _};

use std::sync::Arc;

pub mod normal_mode_animations;
pub mod turning_animations;
pub trait TurnLightAnimation {
    fn oem_bliking(this: Arc<Mutex<AngelEye>>, msg: Arc<Mutex<MailBox>>);
}

pub trait NormalModeAnimation {
    fn static_color(this: Arc<Mutex<AngelEye>>, msg: Arc<Mutex<MailBox>>);
    fn hsv_rainbow(this: Arc<Mutex<AngelEye>>, msg: Arc<Mutex<MailBox>>);
}
