use handle_map::Handle;

use crate::{events::EventRegistry, pearls::Pearl, World};

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

    /// Inserts a resource into the application.
    #[inline]
    pub fn insert_resource<T: 'static>(&mut self, resource: T) {
        self.world.resources.insert(resource);
    }

    /// Consumes the builder, and returns a [`World`] and [`EventRegistry`].
    pub fn consume(self) -> (World, EventRegistry) {
        (self.world, self.events)
    }
}
