use boba_core::*;
use milk_tea_runner::{events::MilkTeaResize, MilkTeaWindows};

use crate::{TaroRenderer, TaroWindowSurface};

pub struct ResizePearl;

impl PearlRegister for ResizePearl {
    fn register(pearl: Pearl<Self>, storage: &mut storage::StageRunners) {
        storage.add(pearl);
    }
}

impl PearlStage<BobaEvent<MilkTeaResize>> for ResizePearl {
    fn update(
        &mut self,
        data: &MilkTeaResize,
        resources: &mut boba_core::BobaResources,
    ) -> PearlResult {
        let Ok(renderer) = resources.borrow::<TaroRenderer>() else {
            return Ok(());
        };

        let size = data.size();

        if let Ok(mut windows) = resources.borrow_mut::<MilkTeaWindows>() {
            if let Some(surface) = windows.main_mut().get_surface::<TaroWindowSurface>() {
                surface.resize(*size, renderer.hardware());
            }
        }

        Ok(())
    }
}
