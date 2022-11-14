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
        let mut renderer = match resources.borrow_mut::<TaroRenderer>() {
            Ok(item) => item,
            Err(e) => {
                warn!(
                    "Skipping TaroRenderer initialization. TaroRenderer Resource Error: {:?}",
                    e
                );
                return;
            }
        };

        let windows = match resources.borrow::<MilkTeaWindows>() {
            Ok(item) => item,
            Err(e) => {
                warn!(
                    "Skipping TaroRenderer initialization. MilkTeaWindows Resource Error: {:?}",
                    e
                );
                return;
            }
        };

        renderer.initialize(windows.main());
    }
}
