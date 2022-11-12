use boba_core::{storage::ControllerStorage, BobaResources, BobaStage};

pub struct MilkTeaUpdate;

impl BobaStage for MilkTeaUpdate {
    type StageData = ();

    fn run(&mut self, controllers: &mut ControllerStorage<Self>, resources: &mut BobaResources) {
        controllers.update(&(), resources);
    }
}
