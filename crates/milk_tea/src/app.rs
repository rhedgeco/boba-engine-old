use std::marker::PhantomData;

use boba_core::{stages::BobaUpdate, BobaResources, BobaStage, PearlRegistry, StageCollection};

use winit::{
    dpi::PhysicalSize,
    error::OsError,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::{
    events::{MilkTeaEvent, MilkTeaSize},
    MilkTeaRenderAdapter, MilkTeaWindow,
};

pub struct Bobarista<RenderAdapter>
where
    RenderAdapter: MilkTeaRenderAdapter,
{
    pub registry: PearlRegistry,
    pub startup_stages: StageCollection,
    pub main_stages: StageCollection,
    pub resources: BobaResources,

    _renderer: PhantomData<RenderAdapter>,
}

impl<RenderAdapter> Default for Bobarista<RenderAdapter>
where
    RenderAdapter: MilkTeaRenderAdapter,
{
    fn default() -> Self {
        // create application
        let mut new = Self {
            registry: Default::default(),
            startup_stages: Default::default(),
            main_stages: Default::default(),
            resources: Default::default(),
            _renderer: Default::default(),
        };

        // add default stages
        new.main_stages.append(BobaUpdate::default());

        // return
        new
    }
}

impl<RenderAdapter> Bobarista<RenderAdapter>
where
    RenderAdapter: MilkTeaRenderAdapter,
{
    pub fn run(mut self) -> Result<(), OsError> {
        env_logger::init();

        // Create main event loop and winit window
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(1280, 720))
            .with_title("Milk Tea Window")
            .build(&event_loop)?;

        // wrap window in manager
        let mut window = MilkTeaWindow::<RenderAdapter>::new(window);

        // run the startup stages
        self.startup_stages
            .run(&mut self.registry, &mut self.resources);

        // run the main event loop
        event_loop.run(move |event, _, control_flow| {
            control_flow.set_poll();

            match event {
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    WindowEvent::Resized(size) => {
                        MilkTeaEvent::new(MilkTeaSize::new(size.width, size.height))
                            .run(&mut self.registry, &mut self.resources)
                            .unwrap();
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        MilkTeaEvent::new(MilkTeaSize::new(
                            new_inner_size.width,
                            new_inner_size.height,
                        ))
                        .run(&mut self.registry, &mut self.resources)
                        .unwrap();
                    }
                    WindowEvent::KeyboardInput {
                        device_id: _,
                        input,
                        is_synthetic: _,
                    } => {
                        MilkTeaEvent::new(input.clone())
                            .run(&mut self.registry, &mut self.resources)
                            .unwrap();
                    }
                    _ => (),
                },
                Event::MainEventsCleared => {
                    self.main_stages
                        .run(&mut self.registry, &mut self.resources);
                    window.render(&mut self.registry, &mut self.resources);
                }
                _ => (),
            }
        })
    }
}
