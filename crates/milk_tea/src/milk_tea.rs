use std::time::{Duration, Instant};

use boba_core::{pearls::map::BobaPearls, BobaResources};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::{
    events::{KeyboardInput, LateUpdate, Update, WindowCloseRequested},
    MilkTeaCommand, MilkTeaCommands, MilkTeaSettings, MilkTeaWindows, RenderBuilder,
};

#[derive(Default)]
pub struct MilkTea {
    pub pearls: BobaPearls,
    pub resources: BobaResources,
    pub settings: MilkTeaSettings,
}

impl MilkTea {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(
        mut self,
        window_builder: WindowBuilder,
        render_builder: impl RenderBuilder,
    ) -> anyhow::Result<()> {
        // create main event loop and window
        let event_loop = EventLoop::new();
        let window = window_builder.build(&event_loop)?;

        // create and spawn the window with the window rendering system
        let mut windows = MilkTeaWindows::new(render_builder.build());
        let mut spawn_event = windows.spawn_now("main", window);

        // add commands and windows to the resources
        self.resources.insert(MilkTeaCommands::new());
        self.resources.insert(windows);

        // trigger the spawn event for the main window
        self.pearls.trigger(&mut spawn_event, &mut self.resources);

        // run the main event loop
        let mut timer = DeltaTimer::new();
        event_loop.run(move |event, window_target, control_flow| {
            control_flow.set_wait();

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } => {
                    let Some(windows) = self.resources.get_mut::<MilkTeaWindows>() else {
                        control_flow.set_exit();
                        return;
                    };

                    match event {
                        WindowEvent::CloseRequested => {
                            let Some(name) = windows.get_name(window_id) else { return };
                            let mut close_event = WindowCloseRequested::new(&name);
                            self.pearls.trigger(&mut close_event, &mut self.resources);

                            if self.settings.close_window_when_requested {
                                let Some(windows) = self.resources.get_mut::<MilkTeaWindows>() else {
                                    control_flow.set_exit();
                                    return;
                                };
                                
                                let Some(mut destroy_event) = windows.destroy_now(&name) else { return };
                                self.pearls.trigger(&mut destroy_event, &mut self.resources);
                            }

                            if self.settings.exit_when_close_requested {
                                control_flow.set_exit();
                            }
                        }
                        WindowEvent::KeyboardInput {
                            device_id,
                            input,
                            is_synthetic,
                        } => {
                            let Some(name) = windows.get_name(window_id) else { return };
                            self.pearls.trigger(
                                &mut KeyboardInput::new(name, *device_id, *input, *is_synthetic),
                                &mut self.resources,
                            );
                        }
                        _ => (),
                    }
                }
                Event::MainEventsCleared => {
                    // trigger main updates with delta time
                    let delta_time = timer.measure().as_secs_f64();
                    let mut update = Update::new(delta_time);
                    let mut late_update = LateUpdate::new(delta_time);
                    self.pearls.trigger(&mut update, &mut self.resources);
                    self.pearls.trigger(&mut late_update, &mut self.resources);

                    // get and execute all the collected milk tea commands
                    if let Some(commands) = self.resources.get_mut::<MilkTeaCommands>() {
                        for command in commands.drain() {
                            match command {
                                MilkTeaCommand::Exit => {
                                    control_flow.set_exit();
                                    return;
                                }
                                _ => (),
                            }
                        }
                    } else {
                        control_flow.set_exit();
                        return;
                    };

                    // submit all pending window spawn and destroy queues and trigger pearl events
                    if let Some(windows) = self.resources.get_mut::<MilkTeaWindows>() {
                        let destroy_events = windows.submit_destroy_queue();
                        let spawn_events = windows.submit_spawn_queue(window_target);
                        for (mut destroy, mut spawn) in
                            destroy_events.into_iter().zip(spawn_events.into_iter())
                        {
                            self.pearls.trigger(&mut destroy, &mut self.resources);
                            self.pearls.trigger(&mut spawn, &mut self.resources);
                        }
                    } else {
                        control_flow.set_exit();
                        return;
                    }
                }
                Event::RedrawRequested(id) => {
                    // remove windows from resources temporarily to run a render update
                    let Some(mut windows) = self.resources.remove::<MilkTeaWindows>() else {
                        control_flow.set_exit();
                        return;
                    };

                    // render the requested window
                    windows.render(id, &mut self.pearls, &mut self.resources);

                    // put windows back into resources for next iteration
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
