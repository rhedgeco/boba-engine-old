use std::time::{Duration, Instant};

use boba_core::{pearls::map::BobaPearls, BobaResources};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::{
    events::{KeyboardInput, LateUpdate, Update, WindowDrop, WindowSpawn},
    MilkTeaCommand, MilkTeaCommands, MilkTeaWindows, RenderBuilder,
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

    pub fn run<R: RenderBuilder>(
        mut self,
        main_window: WindowBuilder,
        render_builder: R,
    ) -> MilkTeaResult {
        let event_loop = EventLoop::new();
        let window = main_window.build(&event_loop)?;
        let main_window_id = window.id();
        let renderer = render_builder.build(window);
        self.resources.insert(MilkTeaCommands::new());
        self.resources.insert(MilkTeaWindows::new(renderer));

        self.pearls.trigger(
            &mut WindowSpawn::new(main_window_id, "main".into()),
            &mut self.resources,
        );

        let mut timer = DeltaTimer::new();
        event_loop.run(move |event, window_target, control_flow| {
            control_flow.set_wait();

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } => {
                    let Some(windows) = self.resources.get_mut::<MilkTeaWindows>() else {
                    control_flow.set_exit_with_code(SOFTWARE_ERROR_CODE);
                    return;
                };

                    match event {
                        WindowEvent::CloseRequested => {
                            let name_option = windows.drop_now(window_id);
                            if windows.is_empty() {
                                control_flow.set_exit();
                            }

                            if let Some(name) = name_option {
                                let mut window_drop = WindowDrop::new(window_id, name);
                                self.pearls.trigger(&mut window_drop, &mut self.resources);
                            }
                        }
                        WindowEvent::KeyboardInput {
                            device_id,
                            input,
                            is_synthetic,
                        } => {
                            let Some(window_name) = windows.get_name(window_id) else { return };
                            let mut input = KeyboardInput::new(
                                window_name.into(),
                                *device_id,
                                *input,
                                *is_synthetic,
                            );
                            self.pearls.trigger(&mut input, &mut self.resources);
                        }
                        _ => (),
                    }
                }
                Event::MainEventsCleared => {
                    let delta_time = timer.measure().as_secs_f64();
                    let mut update = Update::new(delta_time);
                    let mut late_update = LateUpdate::new(delta_time);
                    self.pearls.trigger(&mut update, &mut self.resources);
                    self.pearls.trigger(&mut late_update, &mut self.resources);

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
                Event::RedrawRequested(id) => {
                    // remove the window, so we can still pass the resources into the render function
                    let Some(mut windows) = self.resources.remove::<MilkTeaWindows>() else {
                    control_flow.set_exit_with_code(SOFTWARE_ERROR_CODE);
                    return;
                };

                    // build and drop queued windows
                    windows.submit_drop_queue();
                    while let Some(mut spawn_event) = windows.spawn_next(window_target) {
                        self.pearls.trigger(&mut spawn_event, &mut self.resources);
                    }

                    // render window
                    windows.render(id, &mut self.pearls, &mut self.resources);

                    // re-insert the window afterwards
                    self.resources.insert(windows);
                }
                _ => (),
            }
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
            let mut late_update = LateUpdate::new(delta_time);
            self.pearls.trigger(&mut update, &mut self.resources);
            self.pearls.trigger(&mut late_update, &mut self.resources);

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
