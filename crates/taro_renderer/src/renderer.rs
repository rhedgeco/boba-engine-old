use milk_tea::{boba_core::BobaWorld, winit::window::Window, Renderer, RendererBuilder};
use wgpu::{Device, InstanceDescriptor, Queue, Surface, SurfaceConfiguration};

use crate::events::{TaroRenderFinish, TaroRenderStart};

#[derive(Default)]
pub struct TaroBuilder {
    _private: (),
}

impl TaroBuilder {
    pub fn new() -> Self {
        Self::default()
    }
}

impl RendererBuilder for TaroBuilder {
    type Renderer = TaroRenderer;

    fn build(self, window: Window) -> Self::Renderer {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(InstanceDescriptor::default());
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None, // Trace path
        ))
        .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        TaroRenderer {
            window,
            config,
            surface,
            device,
            queue,
        }
    }
}

pub struct TaroRenderer {
    window: Window,
    config: SurfaceConfiguration,
    surface: Surface,
    device: Device,
    queue: Queue,
}

impl Renderer for TaroRenderer {
    fn update_size(&mut self) {
        let new_size = self
            .window
            .inner_size()
            .to_logical(self.window.scale_factor());

        if new_size.width > 0
            && new_size.height > 0
            && new_size.width != self.config.width
            && new_size.height != self.config.height
        {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn render(&mut self, app: &mut BobaWorld) {
        self.update_size();
        app.trigger(&TaroRenderStart);

        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        app.trigger(&TaroRenderFinish);
        self.window.request_redraw();
    }
}
