use boba_core::BobaStage;
use milk_tea_runner::MilkTeaWindows;

use crate::TaroRenderer;

pub struct TaroStartup;

impl BobaStage for TaroStartup {
    type StageData<'a> = TaroStartup;

    fn run(
        &mut self,
        _: &mut boba_core::controller_storage::ControllerStorage,
        resources: &mut boba_core::BobaResources,
    ) {
        let Some(windows) = resources.get::<MilkTeaWindows>() else {
            println!("no window");
            return;
        };

        resources.add(TaroRenderer::new(windows.main()));
    }
}
