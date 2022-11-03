use crate::{
    storage::{controller_storage::ControllerStorage, stage_storage::StageStorage},
    BobaPlugin, BobaResources, BobaRunner,
};

pub struct BobaApp {
    resources: BobaResources,
    controllers: ControllerStorage,
    startup_stages: StageStorage,
    stages: StageStorage,
    runner: Option<Box<dyn BobaRunner>>,
}

impl BobaApp {
    pub fn new<Runner: 'static + BobaRunner>(runner: Runner) -> Self {
        let app = Self {
            resources: Default::default(),
            controllers: Default::default(),
            startup_stages: Default::default(),
            stages: Default::default(),
            runner: Some(Box::new(runner)),
        };

        app
    }

    pub fn resources(&mut self) -> &mut BobaResources {
        &mut self.resources
    }

    pub fn startup_stages(&mut self) -> &mut StageStorage {
        &mut self.startup_stages
    }

    pub fn stages(&mut self) -> &mut StageStorage {
        &mut self.stages
    }

    pub fn controllers(&mut self) -> &mut ControllerStorage {
        &mut self.controllers
    }

    pub fn add_plugin<Plugin: BobaPlugin>(&mut self, plugin: &Plugin) {
        plugin.setup(self)
    }

    pub fn execute_startup_stages(&mut self) {
        self.startup_stages
            .run_stages(&mut self.controllers, &mut self.resources);
    }

    pub fn execute_stages(&mut self) {
        self.stages
            .run_stages(&mut self.controllers, &mut self.resources);
    }

    pub fn run(mut self) {
        let mut runner = std::mem::replace(&mut self.runner, None)
            .expect("Runner should not be None at this point");
        runner.run(self);
    }
}
