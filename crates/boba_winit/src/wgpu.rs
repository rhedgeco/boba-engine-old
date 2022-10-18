use boba_app::{BobaApp, BobaRenderer};
use raster::Color;
use wgpu::{CommandEncoder, SurfaceTexture, TextureView};
use winit::{dpi::PhysicalSize, window::Window};

#[derive(Debug)]
pub enum RendererError {
    AdapterRequestFailed,
    DeviceRequestFailed,
}

#[derive(Debug)]
pub struct WgpuRenderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
}

pub struct WgpuRenderExecutor {
    output: SurfaceTexture,
    view: TextureView,
    encoder: CommandEncoder,
}

impl WgpuRenderer {
    pub async fn new(window: &Window) -> Result<Self, RendererError> {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let Some(adapter) = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await else {
                return Err(RendererError::AdapterRequestFailed);
            };

        let Ok((device, queue)) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                },
                None,
            )
            .await else {
                return Err(RendererError::DeviceRequestFailed);
            };

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };

        surface.configure(&device, &config);

        let renderer = Self {
            surface,
            device,
            queue,
            config,
            size,
        };

        Ok(renderer)
    }

    pub fn get_size(&self) -> PhysicalSize<u32> {
        self.size
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render_app(&mut self, app: &mut BobaApp) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Wgpu Render Encoder"),
            });

        let mut executor = WgpuRenderExecutor {
            output,
            view,
            encoder,
        };

        app.render(&mut executor);
        self.queue
            .submit(std::iter::once(executor.encoder.finish()));
        executor.output.present();

        Ok(())
    }
}

impl BobaRenderer for WgpuRenderExecutor {
    fn render_color(&mut self, color: Color) {
        self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Encoder"),
            depth_stencil_attachment: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: color.r as f64 / 255.,
                        g: color.g as f64 / 255.,
                        b: color.b as f64 / 255.,
                        a: color.a as f64 / 255.,
                    }),
                    store: true,
                },
            })],
        });
    }
}
