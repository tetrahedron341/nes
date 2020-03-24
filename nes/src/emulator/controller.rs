use nes_core::controller::ControllerState;
use std::sync::RwLock;

pub struct Controller {
    pub buttons: RwLock<ControllerState>
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            buttons: RwLock::new(ControllerState::empty())
        }
    }
}

impl nes_core::controller::NESController for Controller {
    fn poll_controller(&self) -> ControllerState {
        *self.buttons.read().unwrap()
    }
}