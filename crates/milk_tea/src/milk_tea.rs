use std::time::{Duration, Instant};

use boba_core::{pearls::map::BobaPearls, BobaResources};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder, WindowId},
};

use crate::{
    events::{KeyboardInput, Update},
    MilkTeaCommand, MilkTeaCommands,
};

type MilkTeaResult = anyhow::Result<()>;

pub trait RendererBuilder {
    type Renderer: MilkTeaRenderer;
    fn build(self, window: Window) -> Self::Renderer;
}

pub trait MilkTeaRenderer: Sized + 'static {
    fn update_size(&mut self);
    fn set_size(&mut self, width: u32, height: u32);
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
        mut resources: BobaResources,
        renderer: impl RendererBuilder,
    ) -> MilkTeaResult {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(self.size.0, self.size.1))
            .with_title(&self.title)
            .build(&event_loop)?;
        let mut renderer = renderer.build(window);

        let mut timer = DeltaTimer::new();
        resources.insert(MilkTeaCommands::new());
        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::CloseRequested => control_flow.set_exit(),
                WindowEvent::Resized(_)
                | WindowEvent::ScaleFactorChanged {
                    scale_factor: _,
                    new_inner_size: _,
                } => renderer.update_size(),
                WindowEvent::KeyboardInput {
                    device_id,
                    input,
                    is_synthetic,
                } => {
                    let mut input = KeyboardInput::new(*device_id, *input, *is_synthetic);
                    pearls.trigger(&mut input, &mut resources);
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                let delta_time = timer.measure().as_secs_f64();
                let mut update = Update::new(delta_time);
                pearls.trigger(&mut update, &mut resources);

                match resources.get_mut::<MilkTeaCommands>() {
                    None => control_flow.set_exit(),
                    Some(commands) => {
                        for command in commands.drain() {
                            match command {
                                MilkTeaCommand::Exit => control_flow.set_exit(),
                                MilkTeaCommand::Resize(width, height) => {
                                    renderer.set_size(width, height)
                                }
                            }
                        }
                    }
                }
            }
            Event::RedrawRequested(id) => {
                renderer.render(id, &mut pearls, &mut resources);
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
    pub fn run(mut pearls: BobaPearls, mut resources: BobaResources) {
        let mut timer = DeltaTimer::new();
        loop {
            let delta_time = timer.measure().as_secs_f64();
            let mut update = Update::new(delta_time);
            pearls.trigger(&mut update, &mut resources);

            match resources.get_mut::<MilkTeaCommands>() {
                None => return,
                Some(commands) => {
                    for command in commands.drain() {
                        match command {
                            MilkTeaCommand::Exit => return,
                            _ => (),
                        }
                    }
                }
            }
        }
    }
}
