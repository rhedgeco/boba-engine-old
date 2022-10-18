use boba_app::*;

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub struct WinitHeadless {
    event_loop: EventLoop<()>,
}

pub struct WinitRunner {
    window: Window,
    event_loop: EventLoop<()>,
}

impl Default for WinitHeadless {
    fn default() -> Self {
        Self {
            event_loop: EventLoop::new(),
        }
    }
}

impl Default for WinitRunner {
    fn default() -> Self {
        Self::new(640, 480, false)
    }
}

impl WinitRunner {
    pub fn new(width: u32, height: u32, resizable: bool) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Boba App!")
            .with_inner_size(LogicalSize::new(width, height))
            .with_resizable(resizable)
            .build(&event_loop)
            .unwrap();

        Self { window, event_loop }
    }
}

impl BobaRunner for WinitHeadless {
    fn run(self, mut app: BobaApp) {
        env_logger::init();

        self.event_loop.run(move |event, _, _| match event {
            Event::MainEventsCleared => {
                app.update();
            }
            _ => {}
        });
    }
}

impl BobaRunner for WinitRunner {
    fn run(self, mut app: BobaApp) {
        env_logger::init();

        self.event_loop
            .run(move |event, _, control_flow| match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => {}
                },

                Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                    app.update();
                }
                Event::MainEventsCleared => {
                    self.window.request_redraw();
                }
                _ => {}
            });
    }
}
