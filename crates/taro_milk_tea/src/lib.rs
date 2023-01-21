use boba_core::ResourceError;
use log::warn;
use milk_tea::{events::MilkTeaEvent, MilkTeaRenderAdapter};
use taro_core::{
    rendering::{RenderTexture, TaroRenderPearls},
    wgpu, HardwareBuilder, TaroCamera, TaroHardware,
};

pub struct OnTaroMilkTeaRender;

pub struct TaroGraphicsAdapter {
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
    hardware: TaroHardware,
}

impl MilkTeaRenderAdapter for TaroGraphicsAdapter {
    fn build(window: &milk_tea::winit::window::Window) -> Self
    where
        Self: Sized,
    {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let hardware = HardwareBuilder::new(instance)
            .compatible_surface(&surface)
            .build();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_DST,
            format: *hardware.format(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(hardware.device(), &config);

        Self {
            surface,
            config,
            hardware,
        }
    }

    fn render(
        &mut self,
        window_size: milk_tea::winit::dpi::PhysicalSize<u32>,
        registry: &mut boba_core::PearlRegistry,
        resources: &mut boba_core::BobaResources,
    ) -> boba_core::BobaResult {
        registry.run_stage::<MilkTeaEvent<OnTaroMilkTeaRender>>(&OnTaroMilkTeaRender, resources);

        // get cameras resource
        let mut camera = resources.get_mut::<TaroCamera>()?;

        // configure surface if necessary
        if self.config.width != window_size.width || self.config.height != window_size.height {
            self.config.width = window_size.width;
            self.config.height = window_size.height;
            self.surface.configure(self.hardware.device(), &self.config);
        }

        // create closure to render all cameras
        let size = (self.config.width, self.config.height);
        let render_texture = RenderTexture::new(size, self.surface.get_current_texture()?);

        let mut render_camera = |pearls: &TaroRenderPearls| {
            camera.render(&render_texture, pearls, &self.hardware);
        };

        // get pearls to render and use closure to render them
        match resources.get::<TaroRenderPearls>() {
            Ok(pearls) => {
                render_camera(&pearls);
            }
            Err(ResourceError::NotFound(_)) => {
                warn!("No 'TaroRenderPearls' struct found in resources.");
                render_camera(&TaroRenderPearls::default())
            }
            Err(e) => return Err(e.into()),
        };

        render_texture.present();
        Ok(())
    }
}
