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
        let Some(map) = self.callbacks.get(&TypeId::of::<E>()) else { return };
        for any_callback in map.values() {
            let callback_runner = any_callback.downcast_ref::<CallbackRunner<E>>().unwrap();
            callback_runner.call(data, world);
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
            entry.insert(Box::new(CallbackRunner::<E>::new::<T>()));
        }
    }
}

struct CallbackRunner<E: Event> {
    function: fn(&E, &mut World),
}

impl<E: Event> CallbackRunner<E> {
    pub fn new<P: EventListener<E>>() -> Self {
        let function = Self::callback::<P>;
        Self { function }
    }

    #[inline]
    pub fn call(&self, data: &E, world: &mut World) {
        (self.function)(data, world);
    }

    fn callback<P: EventListener<E>>(data: &E, world: &mut World) {
        let Some(handles) = world.pearls.get_handles::<P>() else { return };

        // handles must call `to_vec` first so that it is copied
        // and the reference to world is dropped. This is needed so that
        // world may be passed mutably to the pearl callback.
        for handle in handles.to_vec().iter() {
            if world.pearls.contains(handle) {
                P::callback(&handle, data, world);
            }
        }
    }
}
