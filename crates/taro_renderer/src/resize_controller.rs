use boba_core::*;
use milk_tea_runner::events::MilkTeaResize;

use crate::TaroRenderer;

pub struct ResizeController;

impl BobaController for ResizeController {}

impl BobaUpdate<BobaEvent<MilkTeaResize>> for ResizeController {
    fn update<'a>(
        &mut self,
        data: &BobaEvent<MilkTeaResize>,
        resources: &mut boba_core::BobaResources,
    ) {
        let Ok(mut renderer) = resources.borrow_mut::<TaroRenderer>() else {
                return;
            };

        renderer.resize(*data.data().size());
    }
}
