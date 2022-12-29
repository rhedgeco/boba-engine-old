use std::marker::PhantomData;

use boba_core::{BobaResources, PearlRegistry, StageCollection};
use winit::{
    error::OsError,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder, WindowId},
};

pub trait RenderAdapter: 'static {
    fn build(window: Window) -> Self;
    fn window_id(&self) -> WindowId;
}

#[derive(Default)]
pub struct MilkTeaApp<Renderer>
where
    Renderer: RenderAdapter,
{
    pub registry: PearlRegistry,
    pub startup_stages: StageCollection,
    pub main_stages: StageCollection,
    pub resources: BobaResources,

    _renderer: PhantomData<Renderer>,
}

impl<Renderer> MilkTeaApp<Renderer>
where
    Renderer: RenderAdapter,
{
    pub fn run(mut self) -> Result<(), OsError> {
        // Create main event loop and winit window
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Milk Tea Window")
            .build(&event_loop)?;

        // add windows to resources
        self.resources.add(Renderer::build(window));

        // run the startup stages
        self.startup_stages
            .run(&mut self.registry, &mut self.resources);

        // run the main event loop
        event_loop.run(move |event, _, control_flow| {
            control_flow.set_poll();

            let Some(window) = self.resources.get::<Renderer>() else {
                panic!("WindowRenderer '{}' has been removed from resources", std::any::type_name::<Renderer>());
            };

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.window_id() => match event {
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
