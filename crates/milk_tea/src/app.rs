use std::marker::PhantomData;

use boba_core::{stages::BobaUpdate, BobaResources, BobaStage, PearlRegistry, StageCollection};

use winit::{
    error::OsError,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::{event_types::MilkTeaSize, MilkTeaEvent, MilkTeaPlugin};
pub trait MilkTeaAdapter: MilkTeaPlugin + 'static {
    fn build(window: Window) -> Self;
}

pub struct Bobarista<RenderAdapter>
where
    RenderAdapter: MilkTeaAdapter,
{
    pub registry: PearlRegistry,
    pub startup_stages: StageCollection,
    pub main_stages: StageCollection,
    pub resources: BobaResources,

    _renderer: PhantomData<RenderAdapter>,
}

impl<Renderer> Default for Bobarista<Renderer>
where
    Renderer: MilkTeaAdapter,
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

        // set up render plugin
        Renderer::setup(
            &mut new.registry,
            &mut new.startup_stages,
            &mut new.main_stages,
            &mut new.resources,
        );

        // return
        new
    }
}

impl<RenderAdapter> Bobarista<RenderAdapter>
where
    RenderAdapter: MilkTeaAdapter,
{
    pub fn run(mut self) -> Result<(), OsError> {
        env_logger::init();

        // Create main event loop and winit window
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Milk Tea Window")
            .build(&event_loop)?;

        // add windows to resources
        self.resources.add(RenderAdapter::build(window));

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
                }
                _ => (),
            }
        })
    }
}
