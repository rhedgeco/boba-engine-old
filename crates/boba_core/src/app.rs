use std::{any::TypeId, cell::RefMut, mem::transmute};

use crate::{BobaController, BobaResources, ControllerStage, RegisteredStages};

pub struct BobaApp {
    resources: BobaResources,
    controllers: Vec<Box<dyn AnyController>>,
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
        self.controllers.push(Box::new(updater));
    }

    pub fn update<StageData: 'static>(&mut self, data: &mut StageData) {
        for controller in self.controllers.iter_mut() {
            let mut updater = controller.data_mut();
            unsafe {
                updater
                    .transmute_trait(TypeId::of::<dyn ControllerStage<StageData>>())
                    .map(|dst| {
                        transmute::<&mut dyn RegisteredStages, &mut dyn ControllerStage<StageData>>(
                            dst,
                        )
                        .update(data, &mut self.resources)
                    });
            }
        }
    }
}

trait AnyController {
    fn data_mut(&mut self) -> RefMut<dyn RegisteredStages>;
}

impl<T: 'static + RegisteredStages> AnyController for BobaController<T> {
    fn data_mut(&mut self) -> RefMut<dyn RegisteredStages> {
        self.data_mut()
    }
}
