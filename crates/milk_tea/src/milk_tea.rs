use boba_core::{BobaPearls, BobaResources};
use winit::{
    event::{DeviceEvent, Event, WindowEvent},
    event_loop::EventLoop,
};

use crate::{
    events::{
        KeyboardInput, LateUpdate, MouseMotion, Time, Update, WindowCloseRequested, WindowClosed,
        WindowSpawned,
    },
    Command, Commands, RenderBuilder, WindowSettings, Windows,
};

pub struct MilkTeaSettings {
    pub automatic_redraw: bool,
    pub exit_on_close: bool,
    pub close_window_when_requested: bool,
}

impl Default for MilkTeaSettings {
    fn default() -> Self {
        Self {
            automatic_redraw: true,
            exit_on_close: false,
            close_window_when_requested: true,
        }
    }
}

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
        settings: WindowSettings,
        renderer: impl RenderBuilder,
    ) -> anyhow::Result<()> {
        let event_loop = EventLoop::new();
        let renderer = renderer.build("main", settings, &event_loop)?;

        self.resources.insert(Time::new());
        self.resources.insert(Windows::new(renderer));
        self.resources.insert(Commands::new());

        self.pearls.trigger::<WindowSpawned>(
            &WindowSpawned {
                name: "main".into(),
            },
            &mut self.resources,
        );

        event_loop.run(move |event, window_target, control_flow| {
            control_flow.set_wait();

            match event {
                Event::DeviceEvent { device_id, event } => match event {
                    DeviceEvent::MouseMotion { delta } => {
                        let motion_event = MouseMotion {
                            device_id,
                            delta_x: delta.0,
                            delta_y: delta.1,
                        };
                        self.pearls
                            .trigger::<MouseMotion>(&motion_event, &mut self.resources);
                    }
                    _ => (),
                },
                Event::WindowEvent { window_id, event } => {
                    let Some(windows) = self.resources.get_mut::<Windows>() else {
                        log::warn!("Could not find 'Windows' in resources. Exiting application");
                        control_flow.set_exit();
                        return;
                    };

                    match event {
                        WindowEvent::Destroyed => {
                            if windows.manager().is_empty() {
                                log::info!("All windows closed. Exiting app.");
                                control_flow.set_exit();
                                return;
                            }
                        }
                        WindowEvent::CloseRequested => {
                            if self.settings.close_window_when_requested {
                                if let Some(name) = windows.manager().get_name(&window_id) {
                                    let name = name.to_string();
                                    let Some(event) = windows.close_now(&name) else { return };
                                    self.pearls
                                        .trigger::<WindowClosed>(&event, &mut self.resources);
                                }
                            } else {
                                if let Some(name) = windows.manager().get_name(&window_id) {
                                    self.pearls.trigger::<WindowCloseRequested>(
                                        &WindowCloseRequested { name: name.into() },
                                        &mut self.resources,
                                    )
                                }
                            }
                        }
                        WindowEvent::KeyboardInput {
                            device_id,
                            input,
                            is_synthetic,
                        } => {
                            if let Some(name) = windows.manager().get_name(&window_id) {
                                self.pearls.trigger::<KeyboardInput>(
                                    &KeyboardInput::new(
                                        name.into(),
                                        device_id,
                                        input,
                                        is_synthetic,
                                    ),
                                    &mut self.resources,
                                );
                            }
                        }
                        _ => (),
                    }
                }
                Event::MainEventsCleared => {
                    // get and execute the basic update events
                    let Some(time) = self.resources.get_mut::<Time>() else {
                        log::warn!("Could not find 'Time' in resources. Exiting application");
                        control_flow.set_exit();
                        return;
                    };

                    let delta = time.reset_delta();
                    self.pearls
                        .trigger::<Update>(&Update { delta }, &mut self.resources);
                    self.pearls
                        .trigger::<LateUpdate>(&LateUpdate { delta }, &mut self.resources);

                    // get and update all queued windows
                    let Some(windows) = self.resources.get_mut::<Windows>() else {
                        log::warn!("Could not find 'Windows' in resources. Exiting application");
                        control_flow.set_exit();
                        return;
                    };

                    let (closed, spawned) = windows.submit_queues(window_target);
                    for close in closed {
                        self.pearls
                            .trigger::<WindowClosed>(&close, &mut self.resources);
                    }
                    for spawn in spawned {
                        self.pearls
                            .trigger::<WindowSpawned>(&spawn, &mut self.resources);
                    }

                    // get and execute all the collected milk tea commands
                    let Some(commands) = self.resources.get_mut::<Commands>() else {
                        log::warn!("Could not find 'Commands' in resources. Exiting application");
                        control_flow.set_exit();
                        return;
                    };

                    for command in commands.drain() {
                        match command {
                            Command::Exit => {
                                control_flow.set_exit();
                                return;
                            }
                            _ => (),
                        }
                    }
                }
                Event::RedrawRequested(window_id) => {
                    let Some(mut windows) = self.resources.remove::<Windows>() else {
                        log::warn!("Could not find 'Windows' in resources. Exiting application");
                        control_flow.set_exit();
                        return;
                    };

                    windows
                        .manager()
                        .redraw(&window_id, &mut self.pearls, &mut self.resources);

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
        self.resources.insert(Time::new());
        loop {
            let Some(time) = self.resources.get_mut::<Time>() else {
                log::warn!("Could not find 'Time' in resources. Exiting application");
                return;
            };

            let delta = time.reset_delta();
            self.pearls
                .trigger::<Update>(&Update { delta }, &mut self.resources);
            self.pearls
                .trigger::<LateUpdate>(&LateUpdate { delta }, &mut self.resources);
        }
    }
}
