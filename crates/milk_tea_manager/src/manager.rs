use boba_hybrid::{events::EventRegistry, AppManager, World};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::events::MilkTeaUpdate;

pub struct MilkTea {
    title: String,
    size: (u32, u32),
}

impl Default for MilkTea {
    fn default() -> Self {
        Self {
            title: "Milk Tea Window".into(),
            size: (1280, 720),
        }
    }
}

impl MilkTea {
    pub fn new(title: impl Into<String>, size: (u32, u32)) -> Self {
        Self {
            title: title.into(),
            size,
        }
    }
}

impl AppManager for MilkTea {
    fn run(&mut self, mut world: World, events: EventRegistry) -> anyhow::Result<()> {
        env_logger::init();

        let event_loop = EventLoop::new();
        let _window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(self.size.0, self.size.1))
            .with_title(&self.title)
            .build(&event_loop)?;

        event_loop.run(move |event, _, control_flow| {
            control_flow.set_poll();

            match event {
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    _ => (),
                },
                Event::MainEventsCleared => {
                    events.trigger(&MilkTeaUpdate, &mut world);
                }
                _ => (),
            }
        });
    }
}
