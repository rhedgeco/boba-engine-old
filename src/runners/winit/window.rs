use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::app::BobaRunner;

use super::WgpuState;

pub struct WinitRunner {}

impl Default for WinitRunner {
    fn default() -> Self {
        Self {}
    }
}

impl BobaRunner for WinitRunner {
    fn run(&mut self, app: &mut crate::app::BobaApp) {
        env_logger::init();
        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_title("Boba App!")
            .with_inner_size(LogicalSize::new(640., 360.))
            .with_resizable(false)
            .build(&event_loop)
            .unwrap();

        let mut state = pollster::block_on(WgpuState::new(&window));

        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => state.resize(*physical_size),
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    state.resize(**new_inner_size)
                }
                _ => {}
            },

            Event::RedrawRequested(window_id) if window_id == window.id() => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        });
    }
}
