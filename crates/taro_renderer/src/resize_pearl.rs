use boba_core::*;
use log::warn;
use milk_tea_runner::{events::MilkTeaResize, MilkTeaWindows};

use crate::{TaroRenderer, TaroWindowSurface};

pub struct ResizePearl;

impl PearlRegister for ResizePearl {
    fn register(pearl: Pearl<Self>, storage: &mut storage::StageRunners) {
        storage.add(pearl);
    }
}

impl PearlStage<BobaEvent<MilkTeaResize>> for ResizePearl {
    fn update<'a>(
        data: &MilkTeaResize,
        _: &mut Pearl<Self>,
        resources: &mut boba_core::BobaResources,
    ) -> PearlResult {
        let Ok(renderer) = resources.borrow::<TaroRenderer>() else {
            return Ok(());
        };

        let size = data.size();

        if let Ok(mut windows) = resources.borrow_mut::<MilkTeaWindows>() {
            if let Some(surface) = windows.main_mut().get_surface::<TaroWindowSurface>() {
                surface.resize(*size, renderer.resources());
            }
        }

        if let Some(camera_controller) = &renderer.cameras.main_camera {
            if let Ok(mut camera) = camera_controller.data_mut() {
                camera.settings.aspect = size.width as f32 / size.height as f32;
            } else {
                warn!("Could not resize camera. Camera is currenly borrowed as mutable.");
            }
        };

        Ok(())
    }
}
