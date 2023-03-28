use std::any::{Any, TypeId};

use handle_map::Handle;
use hashbrown::{hash_map, HashMap};
use indexmap::{map, IndexMap};

use crate::pearl::{Pearl, PearlCollection, PearlId};

pub trait Event: Sized + 'static {}
impl<T: Sized + 'static> Event for T {}

pub trait EventListener<E: Event>: Pearl {
    fn callback(handle: &Handle<Self>, data: &E, pearls: &mut PearlCollection);
}

pub trait EventRegistrar<T: Pearl> {
    fn push_listener<E: Event>(&mut self)
    where
        T: EventListener<E>;
}

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
    fn push_listener<E: Event>(&mut self)
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
                let Some(handles) = collection.as_slice_handles::<T>() else { return };
                let handles = handles.iter().copied().collect::<Vec<_>>();
                for handle in handles.iter() {
                    T::callback(&handle, data, collection);
                }
            }));
        }
    }
}
