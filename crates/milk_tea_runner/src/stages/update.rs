use boba_core::{storage::PearlStorage, BobaResources, BobaStage};

pub struct MilkTeaUpdate;

impl BobaStage for MilkTeaUpdate {
    type StageData = ();

    fn run(&mut self, pearls: &mut PearlStorage<Self>, resources: &mut BobaResources) {
        pearls.update(&(), resources);
    }
}
