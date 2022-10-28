use std::{any::TypeId, cell::RefMut, mem::transmute};

use crate::{BobaController, BobaResources, ControllerStage, RegisteredStages};

pub struct ControllerStorage {
    controllers: Vec<Box<dyn AnyController>>,
}

impl Default for ControllerStorage {
    fn default() -> Self {
        Self {
            controllers: Default::default(),
        }
    }
}

impl ControllerStorage {
    pub fn add<T: 'static + RegisteredStages>(&mut self, controller: BobaController<T>) {
        self.controllers.push(Box::new(controller))
    }

    pub fn update<StageData: 'static>(
        &mut self,
        data: &mut StageData,
        resources: &mut BobaResources,
    ) {
        for controller in self.controllers.iter_mut() {
            let mut updater = controller.data_mut();
            unsafe {
                updater
                    .transmute_trait(TypeId::of::<dyn ControllerStage<StageData>>())
                    .map(|dst| {
                        transmute::<&mut dyn RegisteredStages, &mut dyn ControllerStage<StageData>>(
                            dst,
                        )
                        .update(data, resources)
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
