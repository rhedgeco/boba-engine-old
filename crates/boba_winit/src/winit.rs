use boba_app::*;
use boba_input::*;

use winit::{
    dpi::LogicalSize,
    event::{Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::WgpuRenderer;

#[derive(Default)]
pub struct WinitHeadless;

pub struct WinitRunner {
    window: Window,
    event_loop: EventLoop<()>,
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
        let event_loop = EventLoop::new();
        event_loop.run(move |event, _, _| {
            if event == Event::MainEventsCleared {
                app.update();
            }
        });
    }
}

impl BobaRunner for WinitRunner {
    fn run(self, mut app: BobaApp) {
        env_logger::init();

        let mut renderer = WgpuRenderer::new(&self.window, wgpu::PresentMode::AutoNoVsync).unwrap();

        self.event_loop.run(move |event, _, control_flow| {
            // *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => renderer.resize(*physical_size),
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        renderer.resize(**new_inner_size)
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                // state,
                                // virtual_keycode,
                                scancode,
                                ..
                            },
                        ..
                    } => {
                        app.keyboard_input(KeyState::Pressed, Some(KeyCode::A), *scancode);
                    }
                    _ => {}
                },
                Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                    app.update();
                    match renderer.render_app(&mut app) {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => renderer.resize(renderer.get_size()),
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                Event::MainEventsCleared => {
                    self.window.request_redraw();
                }
                _ => {}
            }
        });
    }
}
