use boba_core::{BobaResources, BobaResult, PearlRegistry, ResourceError};
use log::warn;
use milk_tea::{
    events::MilkTeaEvent,
    winit::{dpi::PhysicalSize, window::Window},
    MilkTeaRenderAdapter,
};
use taro_renderer::{TaroCameras, TaroHardware, TaroRenderPearls};

/// Stage that gets called before the `TaroGraphicsAdapter` renders all the cameras
pub struct OnTaroMilkTeaRender;

pub struct TaroGraphicsAdapter {
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
    hardware: TaroHardware,
}

impl MilkTeaRenderAdapter for TaroGraphicsAdapter {
    fn build(window: &Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let hardware = TaroHardware::build(instance, &surface);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&hardware.adapter())[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&hardware.device(), &config);

        Self {
            surface,
            config,
            hardware,
        }
    }

    fn render(
        &mut self,
        window_size: PhysicalSize<u32>,
        registry: &mut PearlRegistry,
        resources: &mut BobaResources,
    ) -> BobaResult {
        registry.run_stage::<MilkTeaEvent<OnTaroMilkTeaRender>>(&OnTaroMilkTeaRender, resources);

        // get cameras resource
        let mut cameras = resources.get_mut::<TaroCameras>()?;

        // configure surface if necessary
        if self.config.width != window_size.width || self.config.height != window_size.height {
            self.config.width = window_size.width;
            self.config.height = window_size.height;
            self.surface.configure(self.hardware.device(), &self.config);
        }

        // create texture view from surface
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&Default::default());

        // create command encoder
        let mut encoder =
            self.hardware
                .device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("OnTaroRender Command Encoder"),
                });

        // create closure to render all cameras
        let size = (self.config.width, self.config.height);
        let mut render_all_cameras = |pearls: &TaroRenderPearls| {
            for camera in cameras.cameras.iter_mut() {
                camera.render(&pearls, &view, size, &mut encoder, &self.hardware);
            }
        };

        // get pearls to render and use closure to render them
        match resources.get::<TaroRenderPearls>() {
            Ok(pearls) => {
                render_all_cameras(&pearls);
            }
            Err(ResourceError::NotFound(_)) => {
                warn!("No 'TaroRenderPearls' struct found in resources.");
                render_all_cameras(&TaroRenderPearls::default())
            }
            Err(e) => return Err(e.into()),
        };

        // submit and present the rendered frame
        self.hardware
            .queue()
            .submit(std::iter::once(encoder.finish()));
        Ok(output.present())
    }
}
