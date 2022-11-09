use boba_core::{storage::ControllerStorage, BobaStage};
use log::warn;
use milk_tea_runner::MilkTeaWindows;

use crate::TaroRenderer;

pub struct TaroInitStage;

impl BobaStage for TaroInitStage {
    type StageData<'a> = Self;

    fn run(&mut self, _: &mut ControllerStorage<Self>, resources: &mut boba_core::BobaResources) {
        let Some(window) = resources.get::<MilkTeaWindows>() else {
            warn!("Could not create TaroRenderer. No MilkTeaWindows found.");
            return;
        };

        resources.add(TaroRenderer::new(window.main()));
    }
}
