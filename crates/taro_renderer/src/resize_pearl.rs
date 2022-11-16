use boba_core::*;
use log::warn;
use milk_tea_runner::{events::MilkTeaResize, MilkTeaWindows};

use crate::{TaroRenderer, TaroWindowSurface};

pub struct ResizePearl;

impl BobaUpdate<BobaEvent<MilkTeaResize>> for ResizePearl {
    fn update<'a>(
        &mut self,
        data: &BobaEvent<MilkTeaResize>,
        resources: &mut boba_core::BobaResources,
    ) {
        let Ok(renderer) = resources.borrow::<TaroRenderer>() else {
            return;
        };

        let size = *data.data().size();

        if let Ok(mut windows) = resources.borrow_mut::<MilkTeaWindows>() {
            if let Some(surface) = windows.main_mut().get_surface::<TaroWindowSurface>() {
                surface.resize(data.data().size().clone(), renderer.resources());
            }
        }

        if let Some(camera_controller) = &renderer.cameras.main_camera {
            if let Ok(mut camera) = camera_controller.data().try_borrow_mut() {
                camera.settings.aspect = size.width as f32 / size.height as f32;
            } else {
                warn!("Could not resize camera. Camera is currenly borrowed as mutable.");
            }
        };
    }
}
