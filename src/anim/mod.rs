pub mod normal_mode_animations;
pub mod turning_animations;
use crate::LightInputSignals;
use std::sync::{Arc, Mutex, RwLock};

pub trait TurnLightAnimation {
    fn oem_bliking(this: Arc<Mutex<Self>>, msg: Arc<RwLock<LightInputSignals>>);
}

pub trait NormalModeAnimation {
    fn static_color(this: Arc<Mutex<Self>>, msg: Arc<RwLock<LightInputSignals>>);
}
