use milk_tea_runner::winit;
use winit::{dpi::PhysicalSize, window::Window};

use crate::{TaroHardware, TaroRenderer};

pub struct TaroWindowSurface {
    pub size: winit::dpi::PhysicalSize<u32>,
    pub surface: wgpu::Surface,
    pub config: wgpu::SurfaceConfiguration,
}

impl TaroWindowSurface {
    pub fn new(window: &Window, renderer: &TaroRenderer) -> Self {
        let resources = renderer.hardware();

        let size = window.inner_size();
        let surface = unsafe { resources.instance.create_surface(window) };

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };

        surface.configure(&resources.device, &config);

        Self {
            size,
            surface,
            config,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>, hardware: &TaroHardware) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&hardware.device, &self.config);
        }
    }
}
