use nes_core::controller::ControllerState;
pub struct Controller {
    pub buttons: ControllerState,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            buttons: ControllerState::empty(),
        }
    }
}

impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}

impl nes_core::controller::NESController for Controller {
    fn poll_controller(&self) -> ControllerState {
        self.buttons
    }
}
