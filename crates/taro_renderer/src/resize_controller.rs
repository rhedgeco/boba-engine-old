use boba_core::*;
use milk_tea_runner::{events::MilkTeaResize, MilkTeaWindows};

use crate::{TaroCamera, TaroRenderer, TaroWindowSurface};

pub struct ResizeController;

impl BobaController for ResizeController {}

impl BobaUpdate<BobaEvent<MilkTeaResize>> for ResizeController {
    fn update<'a>(
        &mut self,
        data: &BobaEvent<MilkTeaResize>,
        resources: &mut boba_core::BobaResources,
    ) {
        let size = *data.data().size();

        if let Ok(mut windows) = resources.borrow_mut::<MilkTeaWindows>() {
            if let Some(surface) = windows.main_mut().get_surface::<TaroWindowSurface>() {
                if let Ok(renderer) = resources.borrow::<TaroRenderer>() {
                    surface.resize(data.data().size().clone(), renderer.resources());
                }
            }
        }

        if let Ok(mut camera) = resources.borrow_mut::<TaroCamera>() {
            camera.settings.aspect = size.width as f32 / size.height as f32;
            return;
        };
    }
}
