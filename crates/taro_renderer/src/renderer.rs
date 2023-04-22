use indexmap::IndexMap;
use milk_tea::{
    anyhow::{Context, Result},
    boba_core::{pearls::map::BobaPearls, BobaResources},
    winit::window::{Window, WindowId},
    RenderBuilder, WindowRenderer,
};
use wgpu::{Device, Instance, InstanceDescriptor, Queue, Surface, SurfaceConfiguration};

use crate::events::TaroRender;

#[derive(Default)]
pub struct TaroBuilder {
    desc: InstanceDescriptor,
}

impl TaroBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_descriptor(desc: InstanceDescriptor) -> Self {
        Self { desc }
    }
}

impl RenderBuilder for TaroBuilder {
    type Renderer = TaroRenderer;

    fn build(self) -> Self::Renderer {
        TaroRenderer::new(Instance::new(self.desc))
    }
}

pub struct TaroHardware {
    device: Device,
    queue: Queue,
}

impl TaroHardware {
    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }
}

pub struct WindowManager {
    window: Window,
    surface: Surface,
    config: SurfaceConfiguration,
    hardware: Option<TaroHardware>,
}

impl WindowManager {
    pub fn update_surface_size(&mut self) {
        let Some(hardware) = &mut self.hardware else { return };
        let physical_size = self.window.inner_size();
        let logical_size = physical_size.to_logical(self.window.scale_factor());

        if physical_size.width > 0
            && physical_size.height > 0
            && logical_size.width != self.config.width
            && logical_size.height != self.config.height
        {
            self.config.width = logical_size.width;
            self.config.height = logical_size.height;
            self.surface.configure(hardware.device(), &self.config);
        }
    }
}

pub struct TaroRenderer {
    instance: Instance,
    windows: IndexMap<WindowId, WindowManager>,
}

impl TaroRenderer {
    fn new(instance: Instance) -> Self {
        Self {
            instance,
            windows: IndexMap::new(),
        }
    }
}

impl WindowRenderer for TaroRenderer {
    fn is_empty(&self) -> bool {
        self.windows.is_empty()
    }

    fn len(&self) -> usize {
        self.windows.len()
    }

    fn contains(&self, id: WindowId) -> bool {
        self.windows.contains_key(&id)
    }

    fn init(&mut self, window: Window) -> Result<()> {
        let size = window.inner_size();
        let surface = unsafe { self.instance.create_surface(&window) }?;

        let adapter =
            pollster::block_on(self.instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }))
            .context("Failed to find sufficient hardware adapter.")?;

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
        ))?;

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

        let manager = WindowManager {
            window,
            surface,
            config,
            hardware: Some(TaroHardware { device, queue }),
        };

        self.windows.insert(manager.window.id(), manager);

        Ok(())
    }

    fn destroy(&mut self, id: WindowId) -> bool {
        self.windows.remove(&id).is_some()
    }

    fn render(
        &mut self,
        id: WindowId,
        name: String,
        pearls: &mut BobaPearls,
        resources: &mut BobaResources,
    ) {
        // get the window to render
        let Some(manager) = self.windows.get_mut(&id) else { return };

        // update the surface and get the output texture for this render
        manager.update_surface_size();
        let Ok(output) = manager.surface.get_current_texture() else { return };

        // take hardware to send ownership into render event
        // this is required since events must be 'static
        let hardware = manager.hardware.take().unwrap();
        let mut render_event = TaroRender::new(name, id, output, hardware);
        pearls.trigger(&mut render_event, resources);
        if render_event.immediate_redraw_requested() {
            manager.window.request_redraw();
        }

        // submit render event, and give the hardware back to the manager
        manager.hardware = Some(render_event.submit())
    }
}
