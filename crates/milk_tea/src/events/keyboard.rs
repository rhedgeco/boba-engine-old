use winit::event::{self, DeviceId};

pub struct KeyboardInput {
    pub device: DeviceId,
    pub input: event::KeyboardInput,
    pub synthetic: bool,
}
