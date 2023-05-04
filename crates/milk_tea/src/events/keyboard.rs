use boba_core::Event;
use winit::event::{self, DeviceId, ElementState, VirtualKeyCode};

pub type KeyCode = VirtualKeyCode;

pub struct KeyboardInput {
    window_name: String,
    device_id: DeviceId,
    input: event::KeyboardInput,
    is_synthetic: bool,
}

impl Event for KeyboardInput {
    type Data<'a> = &'a Self;
}

impl KeyboardInput {
    pub(crate) fn new(
        window_name: String,
        device: DeviceId,
        input: event::KeyboardInput,
        synthetic: bool,
    ) -> Self {
        Self {
            window_name,
            device_id: device,
            input,
            is_synthetic: synthetic,
        }
    }

    pub fn window_name(&self) -> &str {
        &self.window_name
    }

    pub fn device_id(&self) -> DeviceId {
        self.device_id
    }

    pub fn scancode(&self) -> u32 {
        self.input.scancode
    }

    pub fn keycode(&self) -> Option<KeyCode> {
        self.input.virtual_keycode
    }

    pub fn is_pressed(&self) -> bool {
        self.input.state == ElementState::Pressed
    }

    pub fn is_released(&self) -> bool {
        self.input.state == ElementState::Released
    }

    pub fn is_synthetic(&self) -> bool {
        self.is_synthetic
    }
}
