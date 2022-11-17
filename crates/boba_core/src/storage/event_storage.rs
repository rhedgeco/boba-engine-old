use std::any::{Any, TypeId};

use indexmap::IndexMap;
use log::info;

use crate::{BobaEvent, BobaResources, Pearl, PearlStage};

use super::PearlStorage;

#[derive(Default)]
pub struct EventStorage {
    stages: IndexMap<TypeId, Box<dyn Any>>,
}

impl EventStorage {
    pub fn add_listener<Data, Update>(&mut self, pearl: Pearl<Update>)
    where
        Data: 'static,
        Update: 'static + PearlStage<BobaEvent<Data>>,
    {
        match self.stages.get_mut(&TypeId::of::<BobaEvent<Data>>()) {
            Some(stage) => stage
                .downcast_mut::<PearlStorage<BobaEvent<Data>>>()
                .unwrap()
                .add(pearl),
            None => {
                let mut storage = PearlStorage::<BobaEvent<Data>>::default();
                storage.add(pearl);
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
            .downcast_mut::<PearlStorage<BobaEvent<Data>>>()
            .expect("pearl storage should be valid at this point")
            .update(&event.data, resources);
    }
}
