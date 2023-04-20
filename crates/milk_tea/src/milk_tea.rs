use std::time::{Duration, Instant};

use boba_core::{pearls::map::BobaPearls, BobaResources};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window,
};

use crate::{
    events::{KeyboardInput, Update},
    MilkTeaBuilder, MilkTeaCommand, MilkTeaCommands, MilkTeaWindow,
};

const SOFTWARE_ERROR_CODE: i32 = 70;

type MilkTeaResult = anyhow::Result<()>;

pub struct WindowSettings {
    pub title: String,
    pub size: (u32, u32),
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            title: "Milk Tea Window".into(),
            size: (1280, 800),
        }
    }
}

#[derive(Default)]
pub struct MilkTea {
    pub settings: WindowSettings,
    pub pearls: BobaPearls,
    pub resources: BobaResources,
}

impl MilkTea {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run<W: MilkTeaBuilder>(mut self, window_builder: W) -> MilkTeaResult {
        let event_loop = EventLoop::new();
        let window = window::WindowBuilder::new()
            .with_inner_size(LogicalSize::new(self.settings.size.0, self.settings.size.1))
            .with_title(&self.settings.title)
            .build(&event_loop)?;

        self.resources.insert(MilkTeaCommands::new());
        self.resources
            .insert(MilkTeaWindow::new(window_builder.build(window)));

        let mut timer = DeltaTimer::new();
        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::CloseRequested => control_flow.set_exit(),
                WindowEvent::KeyboardInput {
                    device_id,
                    input,
                    is_synthetic,
                } => {
                    let mut input = KeyboardInput::new(*device_id, *input, *is_synthetic);
                    self.pearls.trigger(&mut input, &mut self.resources);
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                let delta_time = timer.measure().as_secs_f64();
                let mut update = Update::new(delta_time);
                self.pearls.trigger(&mut update, &mut self.resources);

                let commands = match self.resources.get_mut::<MilkTeaCommands>() {
                    Some(commands) => commands,
                    None => {
                        control_flow.set_exit_with_code(SOFTWARE_ERROR_CODE);
                        return;
                    }
                };

                for command in commands.drain() {
                    match command {
                        MilkTeaCommand::Exit => control_flow.set_exit(),
                        _ => (),
                    }
                }
            }
            Event::RedrawRequested(_) => {
                // remove the window, so we can still pass the resources into the render function
                let Some(mut window) = self.resources.remove::<MilkTeaWindow>() else {
                    control_flow.set_exit_with_code(SOFTWARE_ERROR_CODE);
                    return;
                };

                // render the window
                window
                    .manager()
                    .render(&mut self.pearls, &mut self.resources);

                // re-insert the window afterwards
                self.resources.insert(window);
            }
            _ => (),
        });
    }
}

#[derive(Default)]
pub struct MilkTeaHeadless {
    pub pearls: BobaPearls,
    pub resources: BobaResources,
}

impl MilkTeaHeadless {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(mut self) {
        let mut timer = DeltaTimer::new();
        self.resources.insert(MilkTeaCommands::new());
        loop {
            let delta_time = timer.measure().as_secs_f64();
            let mut update = Update::new(delta_time);
            self.pearls.trigger(&mut update, &mut self.resources);

            match self.resources.get_mut::<MilkTeaCommands>() {
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
