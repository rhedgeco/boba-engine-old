use crate::{
    events::{Event, EventManager},
    pearls::{Link, Pearl, PearlCollection},
    BobaResources,
};

/// Central storage for [`PearlCollection`] and [`BobaResources`]
#[derive(Default)]
pub struct BobaWorld {
    events: EventManager,
    pearls: PearlCollection,
    resources: BobaResources,
}

impl BobaWorld {
    /// Returns a new world
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts `pearl` into this world and returns a [`Link`] to its location.
    #[inline]
    pub fn insert_pearl<P: Pearl>(&mut self, pearl: P) -> Link<P> {
        // if this pearl type has not been registered, register it
        if !self.pearls.contains_type::<P>() {
            P::register(&mut self.events);
        }

        self.pearls.insert(pearl)
    }

    /// Returns a reference to the pearl associated with `link`.
    ///
    /// Returns `None` if the pearl does not exist.
    #[inline]
    pub fn get_pearl<P: Pearl>(&self, link: &Link<P>) -> Option<&P> {
        self.pearls.get(link)
    }

    /// Returns a mutable reference to the pearl associated with `link`.
    ///
    /// Returns `None` if the pearl does not exist.
    #[inline]
    pub fn get_pearl_mut<P: Pearl>(&mut self, link: &Link<P>) -> Option<&mut P> {
        self.pearls.get_mut(link)
    }

    /// Removes and returns the pearl associated with `link`.
    ///
    /// Returns `None` if the pearl does not exist.
    #[inline]
    pub fn remove_pearl<P: Pearl>(&mut self, link: &Link<P>) -> Option<P> {
        self.pearls.remove(link)
    }

    /// Inserts or replaces a `resource` in this world.
    ///
    /// If a resource of the same type already existed, it is returned as `Some(T)`.
    /// Otherwise `None` is returned.
    #[inline]
    pub fn insert_resource<T: 'static>(&mut self, resource: T) -> Option<T> {
        self.resources.insert(resource)
    }

    /// Returns a reference to the resource of type `T` stored in this world.
    ///
    /// Returns `None` if the resource does not exist.
    #[inline]
    pub fn get_resource<T: 'static>(&self) -> Option<&T> {
        self.resources.get()
    }

    /// Returns a mutable reference to the resource of type `T` stored in this world.
    ///
    /// Returns `None` if the resource does not exist.
    #[inline]
    pub fn get_resource_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.resources.get_mut()
    }

    /// Removes and returns the resource of type `T` stored in this world.
    ///
    /// Returns `None` if the resource does not exist.
    #[inline]
    pub fn remove_resource<T: 'static>(&mut self) -> Option<T> {
        self.resources.remove()
    }

    /// Triggers an `event` that will call every pearl stored in this world that is listening for that event.
    ///
    /// For a pearl to "listen" to an event it must implement [`EventListener`]
    /// and register that event in the `register` method of [`Pearl`].
    #[inline]
    pub fn trigger<E: Event>(&mut self, event: &E) {
        self.events
            .trigger(event, &mut self.pearls, &mut self.resources);
    }
}
