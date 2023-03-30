use std::{
    marker::PhantomData,
    time::{Duration, Instant},
};

use boba_hybrid::{events::EventRegistry, AppManager, World};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::events::{MilkTeaEvent, Update};

pub trait Renderer: Sized + 'static {
    fn build(window: Window) -> Self;
    fn update_size(&mut self);
    fn render(&mut self, world: &mut World, events: &mut EventRegistry);
}

pub struct MilkTea<R: Renderer> {
    title: String,
    size: (u32, u32),

    _renderer: PhantomData<*const R>,
}

impl<R: Renderer> Default for MilkTea<R> {
    fn default() -> Self {
        Self {
            title: "Milk Tea Window".into(),
            size: (1280, 720),

            _renderer: PhantomData,
        }
    }
}

impl<R: Renderer> MilkTea<R> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<R: Renderer> AppManager for MilkTea<R> {
    fn run(self, mut world: World, mut events: EventRegistry) -> anyhow::Result<()> {
        env_logger::init();

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(self.size.0, self.size.1))
            .with_title(&self.title)
            .build(&event_loop)?;
        let mut renderer = R::build(window);

        let mut timer = DeltaTimer::new();
        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent { ref event, .. } => {
                match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    WindowEvent::Resized(_)
                    | WindowEvent::ScaleFactorChanged {
                        scale_factor: _,
                        new_inner_size: _,
                    } => renderer.update_size(),
                    _ => (),
                }

                match MilkTeaEvent::from_window_event(event) {
                    Some(event) => {
                        events.trigger(&event, &mut world);
                    }
                    _ => (),
                }
            }
            Event::MainEventsCleared => {
                let delta_time = timer.measure().as_secs_f64();
                events.trigger(&Update::new(delta_time), &mut world);
                renderer.render(&mut world, &mut events);
            }
            _ => (),
        });
    }
}

struct DeltaTimer {
    instant: Option<Instant>,
}

impl DeltaTimer {
    pub fn new() -> Self {
        Self { instant: None }
    }

    pub fn measure(&mut self) -> Duration {
        match &mut self.instant {
            None => {
                self.instant = Some(Instant::now());
                Duration::new(0, 0)
            }
            Some(instant) => {
                let elapsed = instant.elapsed();
                *instant = Instant::now();
                elapsed
            }
        }
    }
}
