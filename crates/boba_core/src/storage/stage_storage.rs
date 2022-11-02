use crate::{controller_storage::ControllerStorage, BobaResources, BobaStage};

#[derive(Default)]
pub struct StageStorage {
    stages: Vec<Box<dyn AnyStage>>,
}

impl StageStorage {
    pub fn add<Stage: 'static + BobaStage>(&mut self, stage: Stage) {
        self.stages.push(Box::new(Box::new(stage)));
    }

    pub fn run_stages(
        &mut self,
        controllers: &mut ControllerStorage,
        resources: &mut BobaResources,
    ) {
        for stage in self.stages.iter_mut() {
            stage.run(controllers, resources);
        }
    }
}

trait AnyStage {
    fn run(&mut self, controllers: &mut ControllerStorage, resources: &mut BobaResources);
}

impl<Stage: 'static + BobaStage> AnyStage for Box<Stage> {
    fn run(&mut self, controllers: &mut ControllerStorage, resources: &mut BobaResources) {
        self.as_mut().run(controllers, resources);
    }
}
