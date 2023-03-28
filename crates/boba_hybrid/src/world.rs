use handle_map::Handle;

use crate::{
    events::{Event, EventRegistry},
    pearls::{Pearl, PearlAccessor, PearlCollection},
};

#[derive(Default)]
pub struct World {
    pearls: PearlCollection,
    event_registry: EventRegistry,
}

impl World {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn insert_pearl<T: Pearl>(&mut self, pearl: T) -> Handle<T> {
        if !self.pearls.contains::<T>() {
            T::register(&mut self.event_registry);
        }

        self.pearls.insert(pearl)
    }

    #[inline]
    pub fn get_pearl<T: Pearl>(&self, handle: &Handle<T>) -> Option<&T> {
        self.pearls.get(handle)
    }

    #[inline]
    pub fn get_pearl_mut<T: Pearl>(&mut self, handle: &Handle<T>) -> Option<&mut T> {
        self.pearls.get_mut(handle)
    }

    #[inline]
    pub fn remove_pearl<T: Pearl>(&mut self, handle: &Handle<T>) -> Option<T> {
        self.pearls.remove(handle)
    }

    #[inline]
    pub fn trigger_event<E: Event>(&mut self, data: &E) {
        self.event_registry.trigger::<E>(&mut self.pearls, data);
    }
}
