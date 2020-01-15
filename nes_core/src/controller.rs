use bitflags::bitflags;

bitflags!{
    pub struct ControllerState: u8 {
        const A = 1 << 0;
        const B = 1 << 1;
        const SELECT = 1 << 2;
        const START = 1 << 3;
        const UP = 1 << 4;
        const DOWN = 1 << 5;
        const LEFT = 1 << 6;
        const RIGHT = 1 << 7;
    }
}

pub trait NESController {
    fn poll_controller(&self) -> ControllerState;
}

impl<C: NESController> NESController for &C {
    fn poll_controller(&self) -> ControllerState {
        (**self).poll_controller()
    }
}

pub struct DummyController();
impl NESController for DummyController {
    fn poll_controller(&self) -> ControllerState {
        ControllerState::empty()
    }
}