use boba_core::{BobaResources, PearlRegistry, StageCollection};
use winit::{
    error::OsError,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

#[derive(Default)]
pub struct MilkTeaApp {
    pub registry: PearlRegistry,
    pub startup_stages: StageCollection,
    pub main_stages: StageCollection,
    pub resources: BobaResources,
}

impl MilkTeaApp {
    pub fn run(mut self) -> Result<(), OsError> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Milk Tea Window")
            .build(&event_loop)?;
        let main_window_id = window.id();

        self.startup_stages
            .run(&mut self.registry, &mut self.resources);

        event_loop.run(move |event, _, control_flow| {
            control_flow.set_poll();

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == main_window_id => match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    _ => (),
                },
                Event::MainEventsCleared => {
                    self.main_stages
                        .run(&mut self.registry, &mut self.resources);
                }
                _ => (),
            }
        })
    }
}
