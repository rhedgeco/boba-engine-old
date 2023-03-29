use std::any::{Any, TypeId};

use hashbrown::{hash_map, HashMap};
use indexmap::{map, IndexMap};

use crate::pearls::{Pearl, PearlCollection, PearlId, PearlManager};

use super::{Event, EventListener, EventRegistrar};

#[derive(Default)]
pub struct EventRegistry {
    callbacks: HashMap<TypeId, IndexMap<PearlId, Box<dyn Any>>>,
}

impl EventRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn trigger<E: Event>(&self, pearls: &mut PearlCollection, data: &E) {
        type Callback<Data> = Box<dyn Fn(&mut PearlCollection, &Data)>;

        let Some(map) = self.callbacks.get(&TypeId::of::<E>()) else { return };
        for any_callback in map.values() {
            any_callback.downcast_ref::<Callback<E>>().unwrap()(pearls, data);
        }
    }
}

impl<T: Pearl> EventRegistrar<T> for EventRegistry {
    fn listen_for<E: Event>(&mut self)
    where
        T: EventListener<E>,
    {
        // get or create callback map for event E
        let callback_map = match self.callbacks.entry(TypeId::of::<E>()) {
            hash_map::Entry::Occupied(e) => e.into_mut(),
            hash_map::Entry::Vacant(e) => e.insert(IndexMap::new()),
        };

        // add event iterator callback to map if it is not already there
        if let map::Entry::Vacant(entry) = callback_map.entry(PearlId::of::<T>()) {
            entry.insert(Box::new(|collection: &mut PearlCollection, data: &E| {
                let Some(handles) = collection.get_handles::<T>() else { return };
                let handles = handles.iter().copied().collect::<Vec<_>>();
                for handle in handles.iter() {
                    T::callback(&handle, collection, data);
                }
            }));
        }
    }
}
