use std::marker::PhantomData;

use boba_core::{BobaResources, BobaStage, PearlRegistry, StageCollection};

use winit::{
    error::OsError,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::{
    stages::{MilkTeaSize, OnMilkTeaResize},
    MilkTeaPlugin,
};

pub trait MilkTeaAdapter: MilkTeaPlugin + 'static {
    fn build(window: Window) -> Self;
    fn raw_window(&self) -> &Window;
}

pub struct Bobarista<Renderer>
where
    Renderer: MilkTeaAdapter,
{
    pub registry: PearlRegistry,
    pub startup_stages: StageCollection,
    pub main_stages: StageCollection,
    pub resources: BobaResources,

    _renderer: PhantomData<Renderer>,
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

impl<Renderer> Bobarista<Renderer>
where
    Renderer: MilkTeaAdapter,
{
    pub fn run(mut self) -> Result<(), OsError> {
        env_logger::init();

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

            let Ok(window) = self.resources.get::<Renderer>() else {
                panic!("WindowRenderer '{}' has been removed from resources", std::any::type_name::<Renderer>());
            };

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.raw_window().id() => match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    WindowEvent::Resized(size) => {
                        drop(window);
                        let mut resize = OnMilkTeaResize::new(MilkTeaSize::new(size.width, size.height));
                        resize.run(&mut self.registry, &mut self.resources).unwrap();
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        drop(window);
                        let mut resize = OnMilkTeaResize::new(MilkTeaSize::new(new_inner_size.width, new_inner_size.height));
                        resize.run(&mut self.registry, &mut self.resources).unwrap();
                    }
                    _ => (),
                },
                Event::MainEventsCleared => {
                    drop(window);
                    self.main_stages
                        .run(&mut self.registry, &mut self.resources);
                }
                _ => (),
            }
        })
    }
}
