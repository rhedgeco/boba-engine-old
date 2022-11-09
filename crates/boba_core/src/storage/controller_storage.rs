use hashbrown::HashMap;
use uuid::Uuid;

use crate::{BobaController, BobaResources, BobaStage, ControllerData, ControllerStage};

pub struct ControllerStorage<Stage: 'static + ?Sized + BobaStage> {
    controllers: HashMap<Uuid, Box<dyn GenericControllerStage<Stage>>>,
}

impl<Stage: 'static + BobaStage> Default for ControllerStorage<Stage> {
    fn default() -> Self {
        Self {
            controllers: Default::default(),
        }
    }
}

impl<Stage: 'static + BobaStage> ControllerStorage<Stage> {
    pub fn insert<Controller>(&mut self, controller: BobaController<Controller>)
    where
        Controller: 'static + ControllerData + ControllerStage<Stage>,
    {
        self.controllers
            .insert(controller.uuid(), Box::new(controller));
    }

    pub fn remove(&mut self, uuid: &Uuid) -> bool {
        self.controllers.remove(uuid).is_some()
    }

    pub fn update<'a>(
        &'a mut self,
        data: &mut Stage::StageData<'a>,
        resources: &mut BobaResources,
    ) {
        for controller in self.controllers.values_mut() {
            controller.update(data, resources);
        }
    }
}

trait GenericControllerStage<Stage: 'static + BobaStage> {
    fn update<'a>(&'a mut self, data: &mut Stage::StageData<'a>, resources: &mut BobaResources);
}

impl<Stage, Controller> GenericControllerStage<Stage> for BobaController<Controller>
where
    Stage: 'static + BobaStage,
    Controller: ControllerStage<Stage>,
{
    fn update<'a>(&'a mut self, data: &mut Stage::StageData<'a>, resources: &mut BobaResources) {
        self.data_mut().update(data, resources);
    }
}
