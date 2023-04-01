use handle_map::Handle;

use crate::{
    events::{Event, EventRegistry},
    pearls::Pearl,
    World,
};

/// A simple system for directing the flow of a boba application.
#[derive(Default)]
pub struct BobaApp {
    world: World,
    events: EventRegistry,
}

impl BobaApp {
    /// Returns a new app.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a [`Pearl`] to be managed by this app, and returns a [`Handle`] to its location.
    #[inline]
    pub fn insert_pearl<T: Pearl>(&mut self, pearl: T) -> Handle<T> {
        // if the pearl hasnt been seen before, register it
        if !self.world.pearls.contains_type::<T>() {
            T::register(&mut self.events);
        }

        self.world.pearls.insert(pearl)
    }

    /// Removes and returns the [`Pearl`] associated with `handle`.
    ///
    /// Returns `None` if the handle is invalid.
    #[inline]
    pub fn remove_pearl<T: Pearl>(&mut self, handle: &Handle<T>) -> Option<T> {
        self.world.pearls.remove(handle)
    }

    /// Inserts a resource into the application.
    #[inline]
    pub fn insert_resource<T: 'static>(&mut self, resource: T) {
        self.world.resources.insert(resource);
    }

    /// Removes and returns a resource from the application.
    #[inline]
    pub fn remove_resource<T: 'static>(&mut self) -> Option<T> {
        self.world.resources.remove::<T>()
    }

    /// Triggers an event on all the relevant pearls in this app.
    #[inline]
    pub fn trigger<E: Event>(&mut self, event: &E) {
        self.events.trigger(event, &mut self.world);
    }
}
