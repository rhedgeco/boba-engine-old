use std::any::{Any, TypeId};

use hashbrown::{hash_map, HashMap};
use indexmap::{map, IndexMap};

use crate::{
    pearls::{Pearl, PearlId},
    World,
};

use super::{Event, EventListener, EventRegistrar};

/// A collection of event runners that can be used to operate on a [`World`].
#[derive(Default)]
pub struct EventRegistry {
    callbacks: HashMap<TypeId, IndexMap<PearlId, Box<dyn Any>>>,
}

impl EventRegistry {
    /// Returns a new empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Triggers an event that runs the callback for all registered [`Pearl`] objects in `world`.
    pub fn trigger<E: Event>(&self, data: &E, world: &mut World) {
        type Callback<Data> = Box<dyn Fn(&Data, &mut World)>;

        let Some(map) = self.callbacks.get(&TypeId::of::<E>()) else { return };
        for any_callback in map.values() {
            any_callback.downcast_ref::<Callback<E>>().unwrap()(data, world);
        }
    }
}

impl<T: Pearl> EventRegistrar<T> for EventRegistry {
    /// Registers a pearl of type `T` to listen for events of type `E`.
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
            entry.insert(Box::new(|data: &E, world: &mut World| {
                let Some(handles) = world.pearls.get_handles::<T>() else { return };
                let handles = handles.iter().copied().collect::<Vec<_>>();
                for handle in handles.iter() {
                    T::callback(&handle, data, world);
                }
            }));
        }
    }
}
