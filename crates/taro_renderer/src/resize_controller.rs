use boba_core::*;
use milk_tea_runner::events::MilkTeaResize;

use crate::{TaroCamera, TaroRenderer};

pub struct ResizeController;

impl BobaController for ResizeController {}

impl BobaUpdate<BobaEvent<MilkTeaResize>> for ResizeController {
    fn update<'a>(
        &mut self,
        data: &BobaEvent<MilkTeaResize>,
        resources: &mut boba_core::BobaResources,
    ) {
        let size = *data.data().size();

        if let Ok(mut renderer) = resources.borrow_mut::<TaroRenderer>() {
            renderer.resize(size);
            return;
        };

        if let Ok(mut camera) = resources.borrow_mut::<TaroCamera>() {
            camera.settings.aspect = size.width as f32 / size.height as f32;
            return;
        };
    }
}
