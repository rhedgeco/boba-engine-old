use boba_core::BobaApp;
use winit::{
    error::OsError,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::{events::MilkTeaResize, MilkTeaWindows};

pub struct MilkTeaRunner;

impl MilkTeaRunner {
    pub fn run(mut app: BobaApp) -> Result<(), OsError> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop)?;
        let main_window_id = window.id();

        // add window manager to app resources
        app.resources().add(MilkTeaWindows::new(window));

        // run startup stages
        app.run_startup_stages();

        // run main loop
        event_loop.run(move |event, _, control_flow| {
            control_flow.set_poll();

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == main_window_id => match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    WindowEvent::Resized(physical_size) => {
                        app.trigger_event(MilkTeaResize::new(*physical_size))
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        app.trigger_event(MilkTeaResize::new(**new_inner_size))
                    }
                    _ => (),
                },
                Event::MainEventsCleared => app.run_stages(),
                _ => (),
            }
        });
    }
}
