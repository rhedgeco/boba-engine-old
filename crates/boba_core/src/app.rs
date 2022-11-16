use crate::{
    storage::{EventStorage, StageStorage},
    BobaEvent, BobaPlugin, BobaResources,
};

#[derive(Default)]
pub struct BobaApp {
    pub resources: BobaResources,
    pub startup_stages: StageStorage,
    pub stages: StageStorage,
    pub events: EventStorage,
}

impl BobaApp {
    pub fn add_plugin<Plugin: BobaPlugin>(&mut self, plugin: Plugin) -> &mut Self {
        plugin.setup(self);
        self
    }

    pub fn run_startup_stages(&mut self) {
        self.startup_stages.run(&mut self.resources);
    }

    pub fn run_stages(&mut self) {
        self.resources.time.reset();
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
