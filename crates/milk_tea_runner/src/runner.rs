use boba_core::BobaRunner;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::{stages::MilkTeaUpdate, MilkTeaWindows};

pub struct MilkTeaRunner {}

impl MilkTeaRunner {
    pub fn new() -> Self {
        Self {}
    }
}

impl BobaRunner for MilkTeaRunner {
    fn run(&mut self, mut app: boba_core::BobaApp) {
        env_logger::init();
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let main_id = window.id();

        app.stages().add(MilkTeaUpdate);
        app.resources().add(MilkTeaWindows::new(window));

        event_loop.run(move |event, _, control_flow| {
            control_flow.set_poll();

            match event {
                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::CloseRequested,
                } if window_id == main_id => control_flow.set_exit(),
                Event::MainEventsCleared => {
                    app.update();
                }
                _ => (),
            }
        });
    }
}
