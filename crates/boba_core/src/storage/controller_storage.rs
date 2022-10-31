use std::{any::TypeId, cell::RefMut, mem::transmute};

use crate::{BobaController, BobaResources, BobaStage, ControllerStage, RegisteredStages};

#[derive(Default)]
pub struct ControllerStorage {
    controllers: Vec<Box<dyn AnyController>>,
}

impl ControllerStorage {
    pub fn add<T: 'static + RegisteredStages>(&mut self, controller: BobaController<T>) {
        self.controllers.push(Box::new(controller))
    }

    pub fn update<StageData: 'static + BobaStage>(
        &mut self,
        data: &mut StageData,
        resources: &mut BobaResources,
    ) {
        for controller in self.controllers.iter_mut() {
            let Some(mut registered_stages) = controller.data_mut() else {
                continue;
            };

            unsafe {
                if let Some(updater) = registered_stages
                    .transmute_trait(TypeId::of::<dyn ControllerStage<StageData>>())
                {
                    transmute::<&mut dyn RegisteredStages, &mut dyn ControllerStage<StageData>>(
                        updater,
                    )
                    .update(data, resources)
                }
            }
        }
    }
}

trait AnyController {
    fn data_mut(&mut self) -> Option<RefMut<dyn RegisteredStages>>;
}

impl<T: 'static + RegisteredStages> AnyController for BobaController<T> {
    fn data_mut(&mut self) -> Option<RefMut<dyn RegisteredStages>> {
        let Some(data) = self.data_mut() else {
            return None;
        };

        Some(data)
    }
}
