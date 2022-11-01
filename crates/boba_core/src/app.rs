use crate::{
    storage::{controller_storage::ControllerStorage, stage_storage::StageStorage},
    BobaResources, BobaRunner,
};

#[derive(Default)]
pub struct BobaApp {
    resources: BobaResources,
    controllers: ControllerStorage,
    stages: StageStorage,
    runner: Option<Box<dyn BobaRunner>>,
}

impl BobaApp {
    pub fn new<T: 'static + BobaRunner>(mut runner: T) -> Self {
        let mut app = Self {
            resources: Default::default(),
            controllers: Default::default(),
            stages: Default::default(),
            runner: None,
        };

        runner.add_stages_and_resources(&mut app);
        app.runner = Some(Box::new(runner));
        app
    }

    pub fn resources(&mut self) -> &mut BobaResources {
        &mut self.resources
    }

    pub fn stages(&mut self) -> &mut StageStorage {
        &mut self.stages
    }

    pub fn controllers(&mut self) -> &mut ControllerStorage {
        &mut self.controllers
    }

    pub fn update(&mut self) {
        for stage in self.stages.iter_mut() {
            stage.run(&mut self.controllers, &mut self.resources);
        }

        self.resources.time_mut().reset();
    }

    pub fn run(mut self) {
        if let Some(mut runner) = std::mem::replace(&mut self.runner, None) {
            runner.run(self);
        } else {
            self.update();
        }
    }
}
