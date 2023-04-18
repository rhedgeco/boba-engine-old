use winit::event::{self, DeviceId};

pub struct KeyboardInput {
    device_id: DeviceId,
    input: event::KeyboardInput,
    is_synthetic: bool,
}

impl KeyboardInput {
    pub fn new(device: DeviceId, input: event::KeyboardInput, synthetic: bool) -> Self {
        Self {
            device_id: device,
            input,
            is_synthetic: synthetic,
        }
    }

    pub fn device(&self) -> DeviceId {
        self.device_id
    }

    pub fn input(&self) -> event::KeyboardInput {
        self.input
    }

    pub fn is_synthetic(&self) -> bool {
        self.is_synthetic
    }
}
