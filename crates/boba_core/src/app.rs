use crate::{
    storage::controller_storage::ControllerStorage, BobaController, BobaResources, RegisteredStages,
};

pub struct BobaApp {
    resources: BobaResources,
    controllers: ControllerStorage,
}

impl Default for BobaApp {
    fn default() -> Self {
        Self {
            resources: Default::default(),
            controllers: Default::default(),
        }
    }
}

impl BobaApp {
    pub fn add_controller<T: 'static + RegisteredStages>(&mut self, updater: BobaController<T>) {
        self.controllers.add(updater);
    }

    pub fn update<T: 'static>(&mut self, data: &mut T) {
        self.controllers.update(data, &mut self.resources);
    }
}
