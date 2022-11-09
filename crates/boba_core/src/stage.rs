use uuid::Uuid;

use crate::{
    storage::ControllerStorage, BobaController, BobaResources, ControllerData, ControllerStage,
};

pub trait BobaStage {
    type StageData<'a>;
    fn run(&mut self, controllers: &mut ControllerStorage<Self>, resources: &mut BobaResources)
    where
        Self: 'static;
}

pub struct StageRunner<Stage>
where
    Stage: 'static + BobaStage,
{
    stage: Stage,
    pub(crate) controllers: ControllerStorage<Stage>,
}

impl<Stage> StageRunner<Stage>
where
    Stage: 'static + BobaStage,
{
    pub fn build(stage: Stage) -> Self {
        Self {
            stage,
            controllers: Default::default(),
        }
    }

    pub fn add_controller<Controller>(&mut self, controller: BobaController<Controller>)
    where
        Controller: 'static + ControllerData + ControllerStage<Stage>,
    {
        self.controllers.insert(controller);
    }

    pub fn delete_controller(&mut self, uuid: Uuid) -> bool {
        self.controllers.remove(&uuid)
    }

    pub fn run(&mut self, resources: &mut BobaResources) {
        self.stage.run(&mut self.controllers, resources);
    }
}
