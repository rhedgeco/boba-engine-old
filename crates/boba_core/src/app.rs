use crate::{
    storage::{controller_storage::ControllerStorage, stage_storage::StageStorage},
    BobaEvent, BobaPlugin, BobaResources, BobaStage,
};

#[derive(Default)]
pub struct BobaApp {
    resources: BobaResources,
    controllers: ControllerStorage,
    startup_stages: StageStorage,
    stages: StageStorage,
}

impl BobaApp {
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

    pub fn add_plugin<Plugin: BobaPlugin>(&mut self, plugin: Plugin) -> &mut Self {
        plugin.setup(self);
        self
    }

    pub fn execute_startup_stages(&mut self) {
        self.startup_stages
            .run_stages(&mut self.controllers, &mut self.resources);
    }

    pub fn execute_stages(&mut self) {
        self.stages
            .run_stages(&mut self.controllers, &mut self.resources);
    }

    pub fn trigger_event<Data: 'static>(&mut self, data: Data) {
        let mut event = BobaEvent::<Data>::new(data);
        event.run(&mut self.controllers, &mut self.resources);
    }
}
