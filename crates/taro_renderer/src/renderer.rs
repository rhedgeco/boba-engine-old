use hashbrown::HashMap;
use milk_tea::{
    anyhow::{self, Context},
    boba_core::{BobaPearls, BobaResources},
    winit::{
        dpi::{PhysicalPosition, PhysicalSize},
        event_loop::EventLoopWindowTarget,
        window::{Window, WindowBuilder, WindowId},
    },
    RenderBuilder, RenderManager, WindowEditor, WindowSettings,
};
use wgpu::{Device, Instance, Queue, Surface, SurfaceConfiguration};

use crate::events::TaroRender;

fn build_window(
    settings: WindowSettings,
    target: &EventLoopWindowTarget<()>,
) -> anyhow::Result<Window> {
    Ok(WindowBuilder::new()
        .with_title(settings.title)
        .with_inner_size(PhysicalSize::new(settings.size.0, settings.size.1))
        .build(target)?)
}

pub struct TaroBuilder {
    _private: (),
}

impl TaroBuilder {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl RenderBuilder for TaroBuilder {
    type Renderer = TaroRenderer;

    fn build(
        self,
        name: &str,
        settings: WindowSettings,
        target: &EventLoopWindowTarget<()>,
    ) -> anyhow::Result<Self::Renderer> {
        let window = build_window(settings, target)?;
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }?;
        let adapter =
            pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptionsBase {
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
            None,
        ))?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
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

        let mut name_map = HashMap::new();
        name_map.insert(window.id(), name.to_string());

        let mut windows = HashMap::new();
        windows.insert(
            name.to_string(),
            WindowManager {
                window,
                surface,
                config: config.clone(),
            },
        );

        Ok(TaroRenderer {
            instance,
            hardware: TaroHardware { device, queue },
            template: config,
            name_map,
            windows,
        })
    }
}

struct WindowManager {
    window: Window,
    surface: Surface,
    config: SurfaceConfiguration,
}

impl WindowManager {
    pub fn update_surface(&mut self, hardware: &TaroHardware) {
        let physical_size = self.window.inner_size();

        if physical_size.width > 0
            && physical_size.height > 0
            && physical_size.width != self.config.width
            && physical_size.height != self.config.height
        {
            self.config.width = physical_size.width;
            self.config.height = physical_size.height;
            self.surface.configure(hardware.device(), &self.config);
        }
    }
}

impl WindowEditor for WindowManager {
    fn title(&self) -> String {
        self.window.title()
    }

    fn set_title(&mut self, title: &str) {
        self.window.set_title(title)
    }

    fn size(&self) -> (u32, u32) {
        let size = self.window.inner_size();
        (size.width, size.height)
    }

    fn set_size(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            log::warn!("Cannot set window width or height to 0. Skipping.");
            return;
        }

        self.window.set_inner_size(PhysicalSize::new(width, height));
    }

    fn position(&self) -> (u32, u32) {
        match self.window.outer_position() {
            Ok(pos) => (
                pos.x.clamp(0, i32::MAX) as u32,
                pos.y.clamp(0, i32::MAX) as u32,
            ),
            Err(_) => (0, 0),
        }
    }

    fn set_position(&self, x: u32, y: u32) {
        self.window.set_outer_position(PhysicalPosition::new(x, y));
    }

    fn fullscreen(&self) -> bool {
        self.window.fullscreen().is_some()
    }

    fn set_fullscreen(&mut self, full: bool) {
        self.window.set_fullscreen(match full {
            false => None,
            true => Some(milk_tea::winit::window::Fullscreen::Borderless(None)),
        })
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

pub struct TaroRenderer {
    instance: Instance,
    hardware: TaroHardware,
    template: SurfaceConfiguration,

    name_map: HashMap<WindowId, String>,
    windows: HashMap<String, WindowManager>,
}

impl RenderManager for TaroRenderer {
    fn spawn_window(
        &mut self,
        name: &str,
        settings: WindowSettings,
        target: &EventLoopWindowTarget<()>,
    ) -> anyhow::Result<()> {
        let window = build_window(settings, target)?;
        let size = window.inner_size();
        let surface = unsafe { self.instance.create_surface(&window) }?;
        let config = wgpu::SurfaceConfiguration {
            width: size.width,
            height: size.height,
            ..self.template.clone()
        };
        surface.configure(&self.hardware.device(), &config);

        self.name_map.insert(window.id(), name.into());
        self.windows.insert(
            name.into(),
            WindowManager {
                window,
                surface,
                config,
            },
        );

        Ok(())
    }

    fn close_window(&mut self, name: &str) -> bool {
        self.windows.remove(name).is_some()
    }

    fn get_window(&mut self, name: &str) -> Option<&mut dyn WindowEditor> {
        let window_manager = self.windows.get_mut(name)?;
        Some(window_manager as &mut dyn WindowEditor)
    }

    fn len(&self) -> usize {
        self.windows.len()
    }

    fn is_empty(&self) -> bool {
        self.windows.is_empty()
    }

    fn get_name(&self, id: &WindowId) -> Option<&str> {
        Some(self.name_map.get(id)?)
    }

    fn redraw(&mut self, id: &WindowId, pearls: &mut BobaPearls, resources: &mut BobaResources) {
        // get the target window
        let Some(name) = self.name_map.get(id) else { return };
        let Some(manager) = self.windows.get_mut(name) else { return };

        // update the surface and get the current output texture
        manager.update_surface(&self.hardware);
        let Ok(output) = manager.surface.get_current_texture() else { return };

        // trigger the render event
        let mut render_event = TaroRender::new(name.clone(), output);
        pearls.trigger::<TaroRender>(render_event.event_data(&self.hardware), resources);

        // submit all collected render data
        render_event.submit(&self.hardware);
    }
}
