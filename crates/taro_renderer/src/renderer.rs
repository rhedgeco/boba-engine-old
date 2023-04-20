use indexmap::IndexMap;
use milk_tea::{
    boba_core::{pearls::map::BobaPearls, BobaResources},
    winit::window::{Window, WindowId},
    MilkTeaRenderer, RenderBuilder,
};
use wgpu::{Device, Instance, InstanceDescriptor, Queue, Surface, SurfaceConfiguration};

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

impl RenderBuilder for TaroBuilder {
    type Renderer = TaroRenderer;

    fn build(self, window: Window) -> Self::Renderer {
        let size = window.inner_size();
        let instance = Instance::new(InstanceDescriptor::default());
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
        let config_template = config.clone();

        let window_name = "main".to_string();
        let main_window = WindowManager {
            name: window_name.clone(),
            window,
            surface,
            config,
        };

        let mut id_mapper = IndexMap::new();
        id_mapper.insert(window_name, main_window.window.id());

        let mut windows = IndexMap::new();
        windows.insert(main_window.window.id(), main_window);

        TaroRenderer {
            id_mapper,
            windows,
            device,
            queue,
            instance,
            config_template,
        }
    }
}

struct WindowManager {
    name: String,
    window: Window,
    surface: Surface,
    config: SurfaceConfiguration,
}

impl WindowManager {
    pub fn update_size(&mut self, device: &Device) {
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
            self.surface.configure(device, &self.config);
        }
    }
}

pub struct TaroRenderer {
    id_mapper: IndexMap<String, WindowId>,
    windows: IndexMap<WindowId, WindowManager>,

    device: Device,
    queue: Queue,

    instance: Instance,
    config_template: SurfaceConfiguration,
}

impl MilkTeaRenderer for TaroRenderer {
    fn window_count(&self) -> usize {
        self.windows.len()
    }

    fn is_empty(&self) -> bool {
        self.windows.is_empty()
    }

    fn get(&self, name: &str) -> Option<&Window> {
        let id = self.id_mapper.get(name)?;
        Some(&self.windows.get(id)?.window)
    }

    fn get_name(&self, id: WindowId) -> Option<&str> {
        Some(&self.windows.get(&id)?.name)
    }

    fn insert(&mut self, name: String, window: Window) {
        let size = window.inner_size();
        let surface = unsafe { self.instance.create_surface(&window) }.unwrap();
        let mut config = self.config_template.clone();
        config.width = size.width;
        config.height = size.height;
        surface.configure(&self.device, &config);

        let manager = WindowManager {
            name: name.clone(),
            window,
            surface,
            config,
        };

        self.id_mapper.insert(name, manager.window.id());
        self.windows.insert(manager.window.id(), manager);
    }

    fn drop_id(&mut self, id: WindowId) {
        let Some(manager) = self.windows.remove(&id) else { return };
        self.id_mapper.remove(&manager.name);
    }

    fn drop_name(&mut self, name: &str) {
        let Some(id) = self.id_mapper.remove(name) else { return };
        self.windows.remove(&id);
    }

    fn render(&mut self, id: WindowId, pearls: &mut BobaPearls, resources: &mut BobaResources) {
        let Some(window) = self.windows.get_mut(&id) else { return };
        window.update_size(&self.device);

        let Ok(output) = window.surface.get_current_texture() else { return };
        pearls.trigger(&mut TaroRenderStart, resources);
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
        pearls.trigger(&mut TaroRenderFinish, resources);
        window.window.request_redraw();
    }
}
