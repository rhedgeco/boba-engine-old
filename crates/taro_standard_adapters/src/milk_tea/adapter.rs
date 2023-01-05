use boba_core::{BobaResources, Pearl, PearlRegistry, StageCollection};

use log::error;
use milk_tea::{event_types::MilkTeaSize, winit::window::Window, MilkTeaAdapter, MilkTeaPlugin};
use taro_renderer::{
    stages::{OnTaroRender, TaroSurfaceManager},
    TaroHardware, TaroSurface,
};

use super::TaroMilkTeaResizeListener;

pub struct TaroMilkTea {
    _window: Window,
    taro_surface: TaroSurface,
    hardware: TaroHardware,
}

impl TaroMilkTea {
    pub fn resize(&mut self, size: &MilkTeaSize) {
        let width = size.width;
        let height = size.height;
        if width == 0 || height == 0 {
            error!(
                "Tried to resize surface to ({width}, {height}). Dimensions must be greater than 0."
            );
            return;
        }

        self.taro_surface.config.width = width;
        self.taro_surface.config.height = height;
        self.taro_surface
            .surface
            .configure(&self.hardware.device(), &self.taro_surface.config);
    }
}

impl TaroSurfaceManager for TaroMilkTea {
    fn get_hardware(&self) -> &TaroHardware {
        &self.hardware
    }

    fn get_current_texture(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError> {
        self.taro_surface.surface.get_current_texture()
    }

    fn get_surface_size(&self) -> (u32, u32) {
        (
            self.taro_surface.config.width,
            self.taro_surface.config.height,
        )
    }
}

impl MilkTeaAdapter for TaroMilkTea {
    fn build(window: Window) -> Self {
        let size = window.inner_size();

        // Safety: Surface must be alive for as long as the window.
        // The surface and window are not exposed externally,
        // and thus any problems between the two are handled internally.
        let (hardware, taro_surface) =
            unsafe { TaroHardware::build(&window, (size.width, size.height)) };

        Self {
            _window: window,
            taro_surface,
            hardware,
        }
    }
}

impl MilkTeaPlugin for TaroMilkTea {
    fn setup(
        registry: &mut PearlRegistry,
        _: &mut StageCollection,
        main_stages: &mut StageCollection,
        _resources: &mut BobaResources,
    ) {
        main_stages.append(OnTaroRender::<TaroMilkTea>::default());
        registry.add(&Pearl::wrap(TaroMilkTeaResizeListener));
    }
}
