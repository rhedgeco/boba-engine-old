use std::any::{Any, TypeId};

use hashbrown::{hash_map, HashMap};

use crate::{Pearl, PearlCollection};

pub trait Event: Sized + 'static {
    type EventData;
}

pub trait EventListener<T: Event> {
    fn callback(&mut self, data: &T::EventData);
}

pub trait EventRegistrar<T: Pearl> {
    fn push_listener<E: Event>(&mut self)
    where
        T: EventListener<E>;
}

#[derive(Default)]
pub struct EventRegistry {
    callbacks: HashMap<TypeId, Vec<Box<dyn Any>>>,
}

impl EventRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn trigger<E: Event>(&self, pearls: &mut PearlCollection, data: &E::EventData) {
        type Callback<Data> = Box<dyn Fn(&mut PearlCollection, &Data)>;

        let Some(map) = self.callbacks.get(&TypeId::of::<E>()) else { return };
        for any_callback in map.iter() {
            let callback = any_callback
                .downcast_ref::<Callback<E::EventData>>()
                .unwrap();

            callback(pearls, data);
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
            hash_map::Entry::Vacant(e) => e.insert(Vec::new()),
        };

        // add callback function runner for pearl T to the callback map
        callback_map.push(Box::new(
            |collection: &mut PearlCollection, data: &E::EventData| {
                let Some(slice) = collection.as_slice_mut::<T>() else { return };
                for pearl in slice.iter_mut() {
                    pearl.callback(data);
                }
            },
        ));
    }
}
