use boba_core::Event;
use winit::event::DeviceId;

pub struct MouseMotion {
    pub device_id: DeviceId,
    pub delta_x: f64,
    pub delta_y: f64,
}

impl Event for MouseMotion {
    type Data<'a> = &'a Self;
}
