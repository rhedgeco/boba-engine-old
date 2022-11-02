use crate::{storage::controller_storage::ControllerStorage, BobaResources};

pub trait BobaStage {
    type StageData<'a>;
    fn run(&mut self, controllers: &mut ControllerStorage, resources: &mut BobaResources);
}
