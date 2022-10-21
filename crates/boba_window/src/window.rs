use boba_app::{BobaApp, BobaEvent, BobaRunner};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

#[derive(Debug, PartialEq)]
pub enum RendererError {
    AdapterRequestFailed,
    DeviceRequestFailed,
}

pub trait WindowRenderer
where
    Self: Sized,
{
    fn init(window: &Window) -> Result<Self, RendererError>;
    fn resize(&mut self, width: u32, height: u32);
}

pub struct BobaWindow<R: 'static + WindowRenderer> {
    window: Window,
    event_loop: EventLoop<()>,
    renderer: R,
}

impl<R: 'static + WindowRenderer> BobaWindow<R> {
    pub fn new() -> Result<Self, RendererError> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Boba App!")
            .with_inner_size(LogicalSize::new(640, 480))
            .with_resizable(false)
            .build(&event_loop)
            .unwrap();

        let renderer = R::init(&window)?;

        let boba_window = Self {
            window,
            event_loop,
            renderer,
        };

        Ok(boba_window)
    }
}

impl<R: 'static + WindowRenderer> BobaRunner for BobaWindow<R> {
    fn run<A: 'static + BobaApp>(mut self, mut app: A) {
        env_logger::init();

        self.event_loop
            .run(move |event, _, control_flow| match event {
                Event::WindowEvent { event, window_id } if window_id == self.window.id() => {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => self
                            .renderer
                            .resize(physical_size.width, physical_size.height),
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => self
                            .renderer
                            .resize(new_inner_size.width, new_inner_size.height),
                        _ => {}
                    }
                }
                Event::MainEventsCleared => {
                    app.handle_event(BobaEvent::Update);
                }
                _ => {}
            });
    }
}
