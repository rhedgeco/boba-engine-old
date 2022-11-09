use crate::{
    storage::{EventStorage, StageStorage},
    BobaEvent, BobaPlugin, BobaResources,
};

#[derive(Default)]
pub struct BobaApp {
    resources: BobaResources,
    startup_stages: StageStorage,
    stages: StageStorage,
    events: EventStorage,
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

    pub fn events(&mut self) -> &mut EventStorage {
        &mut self.events
    }

    pub fn add_plugin<Plugin: BobaPlugin>(&mut self, plugin: Plugin) -> &mut Self {
        plugin.setup(self);
        self
    }

    pub fn run_startup_stages(&mut self) {
        self.startup_stages.run(&mut self.resources);
    }

    pub fn run_stages(&mut self) {
        self.stages.run(&mut self.resources);
    }

    pub fn trigger_event<Data>(&mut self, data: Data)
    where
        Data: 'static,
    {
        let mut event = BobaEvent::new(data);
        self.events
            .trigger_event::<Data>(&mut event, &mut self.resources);
    }
}
