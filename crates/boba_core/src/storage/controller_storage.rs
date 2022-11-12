use hashbrown::HashMap;
use log::error;
use uuid::Uuid;

use crate::{BobaController, BobaResources, BobaStage, ControllerData, ControllerStage};

/// A storage solution for `BobaController` objects.
///
/// You may be `insert` and `remove` controllers in any order.
/// The storage system has an `update` function that will iterate over
/// every component and call its corresponding `update` function.
/// This struct will typically be used inside a `BobaStage` as the
/// owner of all the controllers to be run for that stage.
pub struct ControllerStorage<Stage: 'static + ?Sized + BobaStage> {
    controllers: HashMap<Uuid, Box<dyn GenericControllerStage<Stage>>>,
}

/// The default implementation for `ControllerStorage<BobaStage>`
impl<Stage: 'static + BobaStage> Default for ControllerStorage<Stage> {
    fn default() -> Self {
        Self {
            controllers: Default::default(),
        }
    }
}

impl<Stage: 'static + BobaStage> ControllerStorage<Stage> {
    /// Adds a controller to the storage system.
    pub fn add<Controller>(&mut self, controller: BobaController<Controller>)
    where
        Controller: 'static + ControllerData + ControllerStage<Stage>,
    {
        self.controllers
            .insert(*controller.uuid(), Box::new(controller));
    }

    /// Removed a controller from the storage system
    pub fn remove(&mut self, uuid: &Uuid) {
        self.controllers.remove(uuid);
    }

    // updates all controllers that are currently in storage
    pub fn update(&mut self, data: &Stage::StageData, resources: &mut BobaResources) {
        for controller in self.controllers.values_mut() {
            controller.update(data, resources);
        }
    }
}

trait GenericControllerStage<Stage: 'static + BobaStage> {
    fn update(&mut self, data: &Stage::StageData, resources: &mut BobaResources);
}

impl<Stage, Controller> GenericControllerStage<Stage> for BobaController<Controller>
where
    Stage: 'static + BobaStage,
    Controller: ControllerStage<Stage>,
{
    fn update<'a>(&'a mut self, data: &Stage::StageData, resources: &mut BobaResources) {
        let Ok(mut controller) = self.data().try_borrow_mut() else {
            error!("Skipping update on BobaController<{:?}>: BorrowMutError. Already Borrowed", std::any::type_name::<Controller>());
            return;
        };

        controller.update(data, resources);
    }
}
