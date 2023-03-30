use winit::event::{
    DeviceId, ElementState, KeyboardInput, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent,
};

pub enum MilkTeaEvent {
    /// An event from the keyboard has been received.
    KeyboardInput {
        device_id: DeviceId,
        input: KeyboardInput,
        /// If `true`, the event was generated synthetically by winit
        /// in one of the following circumstances:
        ///
        /// * Synthetic key press events are generated for all keys pressed
        ///   when a window gains focus. Likewise, synthetic key release events
        ///   are generated for all keys pressed when a window goes out of focus.
        ///   ***Currently, this is only functional on X11 and Windows***
        ///
        /// Otherwise, this value is always `false`.
        is_synthetic: bool,
    },

    /// An mouse button press has been received.
    MouseInput {
        device_id: DeviceId,
        state: ElementState,
        button: MouseButton,
    },

    /// A mouse wheel movement or touchpad scroll occurred.
    MouseWheel {
        device_id: DeviceId,
        delta: MouseScrollDelta,
        phase: TouchPhase,
    },
}

impl MilkTeaEvent {
    #[allow(deprecated)]
    pub fn from_window_event(event: &WindowEvent) -> Option<Self> {
        match event {
            WindowEvent::KeyboardInput {
                device_id,
                input,
                is_synthetic,
            } => Some(MilkTeaEvent::KeyboardInput {
                device_id: device_id.clone(),
                input: input.clone(),
                is_synthetic: is_synthetic.clone(),
            }),
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
                modifiers: _,
            } => Some(MilkTeaEvent::MouseInput {
                device_id: device_id.clone(),
                state: state.clone(),
                button: button.clone(),
            }),
            WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
                modifiers: _,
            } => Some(MilkTeaEvent::MouseWheel {
                device_id: device_id.clone(),
                delta: delta.clone(),
                phase: phase.clone(),
            }),
            _ => None,
        }
    }
}
