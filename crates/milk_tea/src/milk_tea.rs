use std::time::{Duration, Instant};

use boba_core::{pearls::map::BobaPearls, BobaResources};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder, WindowId},
};

use crate::events::Update;

type MilkTeaResult = anyhow::Result<()>;

pub trait RendererBuilder {
    type Renderer: Renderer;
    fn build(self, window: Window) -> Self::Renderer;
}

pub trait Renderer: Sized + 'static {
    fn update_size(&mut self);
    fn render(&mut self, id: WindowId, pearls: &mut BobaPearls, resources: &mut BobaResources);
}

pub struct MilkTeaWindow {
    title: String,
    size: (u32, u32),
}

impl Default for MilkTeaWindow {
    fn default() -> Self {
        Self {
            title: "Milk Tea Window".into(),
            size: (1280, 720),
        }
    }
}

impl MilkTeaWindow {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(
        self,
        mut pearls: BobaPearls,
        mut resouces: BobaResources,
        renderer: impl RendererBuilder,
    ) -> MilkTeaResult {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(self.size.0, self.size.1))
            .with_title(&self.title)
            .build(&event_loop)?;
        let mut renderer = renderer.build(window);

        let mut timer = DeltaTimer::new();
        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::CloseRequested => control_flow.set_exit(),
                WindowEvent::Resized(_)
                | WindowEvent::ScaleFactorChanged {
                    scale_factor: _,
                    new_inner_size: _,
                } => renderer.update_size(),
                _ => (),
            },
            Event::MainEventsCleared => {
                let delta_time = timer.measure().as_secs_f64();
                pearls.trigger(&Update::new(delta_time), &mut resouces);
            }
            Event::RedrawRequested(id) => {
                renderer.render(id, &mut pearls, &mut resouces);
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

pub struct MilkTeaHeadless {
    _private: (),
}

impl MilkTeaHeadless {
    pub fn run(mut pearls: BobaPearls, mut resources: BobaResources) -> ! {
        let mut timer = DeltaTimer::new();
        loop {
            let delta_time = timer.measure().as_secs_f64();
            pearls.trigger(&Update::new(delta_time), &mut resources);
        }
    }
}
