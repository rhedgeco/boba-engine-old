use crate::{storage::ControllerStorage, BobaResources};

pub trait BobaStage {
    type StageData;
    fn run(&mut self, controllers: &mut ControllerStorage<Self>, resources: &mut BobaResources)
    where
        Self: 'static;
}

pub struct MainBobaUpdate;

impl BobaStage for MainBobaUpdate {
    type StageData = ();

    fn run(&mut self, controllers: &mut ControllerStorage<Self>, resources: &mut BobaResources)
    where
        Self: 'static,
    {
        controllers.update(&(), resources);
    }
}

pub struct StageRunner<Stage>
where
    Stage: 'static + BobaStage,
{
    stage: Stage,
    controllers: ControllerStorage<Stage>,
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

    pub fn controllers(&mut self) -> &mut ControllerStorage<Stage> {
        &mut self.controllers
    }

    pub fn run(&mut self, resources: &mut BobaResources) {
        self.stage.run(&mut self.controllers, resources);
    }
}
