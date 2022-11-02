use boba_core::BobaStage;

pub struct MilkTeaUpdate;

impl BobaStage for MilkTeaUpdate {
    type StageData<'a> = MilkTeaUpdate;

    fn run(
        &mut self,
        controllers: &mut boba_core::controller_storage::ControllerStorage,
        resources: &mut boba_core::BobaResources,
    ) {
        controllers.update::<MilkTeaUpdate>(self, resources);
    }
}
