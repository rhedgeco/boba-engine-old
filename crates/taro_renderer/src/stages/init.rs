use boba_core::{storage::ControllerStorage, BobaResources, BobaStage};
use log::warn;
use milk_tea_runner::MilkTeaWindows;

use crate::TaroRenderer;

pub struct TaroRendererInitStage;

impl BobaStage for TaroRendererInitStage {
    type StageData = ();

    fn run(&mut self, _: &mut ControllerStorage<Self>, resources: &mut BobaResources)
    where
        Self: 'static,
    {
        let Some(renderer) = resources.get::<TaroRenderer>() else {
            warn!("Skipping TaroRenderer initialization. No TaroRenderer found.");
            return;
        };

        let Some(windows) = resources.get::<MilkTeaWindows>() else {
            warn!("Skipping TaroRenderer initialization. No MilkTeaWindows found.");
            return;
        };

        // SAFTEY
        //
        // Converts renderer to mutable so that we can get both renderer and window out of resources.
        // They are used once and dropped immediately after.
        unsafe {
            let const_ptr = renderer as *const TaroRenderer;
            let mut_ptr = const_ptr as *mut TaroRenderer;
            (*mut_ptr).initialize(windows.main());
        }
    }
}
