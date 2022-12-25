use boba_core::BobaApp;
use winit::{
    error::OsError,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

pub struct MilkTeaRunner;

impl MilkTeaRunner {
    pub fn run(mut app: BobaApp) -> Result<(), OsError> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Milk Tea Window")
            .build(&event_loop)?;
        let main_window_id = window.id();

        event_loop.run(move |event, _, control_flow| {
            control_flow.set_poll();

            app.startup_stages
                .run(&mut app.registry, &mut app.resources);

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == main_window_id => match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    _ => (),
                },
                Event::MainEventsCleared => {
                    app.main_stages.run(&mut app.registry, &mut app.resources);
                }
                _ => (),
            }
        })
    }
}
