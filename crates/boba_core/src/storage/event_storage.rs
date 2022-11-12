use std::any::{Any, TypeId};

use indexmap::IndexMap;
use log::info;

use crate::{BobaController, BobaEvent, BobaResources, ControllerStage};

use super::ControllerStorage;

#[derive(Default)]
pub struct EventStorage {
    stages: IndexMap<TypeId, Box<dyn Any>>,
}

impl EventStorage {
    pub fn add_listener<Data, Controller>(&mut self, controller: BobaController<Controller>)
    where
        Data: 'static,
        Controller: 'static + ControllerStage<BobaEvent<Data>>,
    {
        match self.stages.get_mut(&TypeId::of::<BobaEvent<Data>>()) {
            Some(stage) => stage
                .downcast_mut::<ControllerStorage<BobaEvent<Data>>>()
                .unwrap()
                .add(controller),
            None => {
                let mut storage = ControllerStorage::<BobaEvent<Data>>::default();
                storage.add(controller);
                self.stages
                    .insert(TypeId::of::<BobaEvent<Data>>(), Box::new(storage));
            }
        }
    }

    pub fn trigger_event<Data>(
        &mut self,
        event: &mut BobaEvent<Data>,
        resources: &mut BobaResources,
    ) where
        Data: 'static,
    {
        let Some(any_storage) = self.stages.get_mut(&TypeId::of::<BobaEvent<Data>>()) else {
            info!("Event {:?} triggered, but there were no listeners", std::any::type_name::<BobaEvent<Data>>());
            return;
        };

        any_storage
            .downcast_mut::<ControllerStorage<BobaEvent<Data>>>()
            .expect("controller storage should be valid at this point")
            .update(event, resources);
    }
}