use std::any::{Any, TypeId};

use handle_map::RawHandle;
use hashbrown::{hash_map::Entry, HashMap};
use indexmap::IndexMap;

use crate::{
    events::{Event, EventListener, EventRegistrar},
    pearls::{Link, Pearl, PearlCollection, PearlId},
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

/// A window into the resources stored in a world.
///
/// This is used in callbacks for events to provide access to the other pearls and resources.
pub struct WorldView<'a> {
    exclude: RawHandle,
    pearls: &'a mut PearlCollection,

    /// A mutable reference to the resources for the world.
    pub resources: &'a mut BobaResources,
}

impl<'a> WorldView<'a> {
    /// Returns a reference to the pearl associated with `link`.
    ///
    /// Returns `None` if the pearl does not exist, or the pearl is currently being used for a callback.
    #[inline]
    pub fn get_pearl<P: Pearl>(&self, link: &Link<P>) -> Option<&P> {
        match &self.exclude == link.pearl.as_raw() {
            true => None,
            false => self.pearls.get(link),
        }
    }

    /// Returns a mutable reference to the pearl associated with `link`.
    ///
    /// Returns `None` if the pearl does not exist, or the pearl is currently being used for a callback.
    #[inline]
    pub fn get_pearl_mut<P: Pearl>(&mut self, link: &Link<P>) -> Option<&mut P> {
        match &self.exclude == link.pearl.as_raw() {
            true => None,
            false => self.pearls.get_mut(link),
        }
    }
}

type EventCallback<E> = fn(&E, &mut PearlCollection, &mut BobaResources);

#[derive(Default)]
struct EventManager {
    dispatchers: HashMap<TypeId, Box<dyn Any>>,
}

impl EventManager {
    pub fn trigger<E: Event>(
        &self,
        event: &E,
        pearls: &mut PearlCollection,
        resources: &mut BobaResources,
    ) {
        let Some(dispatcher) = self.dispatchers.get(&TypeId::of::<E>()) else { return };
        let dispatcher = dispatcher.downcast_ref::<EventDispatcher<E>>().unwrap();
        dispatcher.call(event, pearls, resources);
    }
}

impl<P: Pearl> EventRegistrar<P> for EventManager {
    fn listen_for<E: Event>(&mut self)
    where
        P: EventListener<E>,
    {
        match self.dispatchers.entry(TypeId::of::<E>()) {
            Entry::Occupied(e) => {
                let dispatcher = e.into_mut().downcast_mut::<EventDispatcher<E>>().unwrap();
                dispatcher.add_callback::<P>();
            }
            Entry::Vacant(e) => {
                let mut dispatcher = EventDispatcher::<E>::new();
                dispatcher.add_callback::<P>();
                e.insert(Box::new(dispatcher));
            }
        }
    }
}

struct EventDispatcher<E: Event> {
    callbacks: IndexMap<PearlId, EventCallback<E>>,
}

impl<E: Event> Default for EventDispatcher<E> {
    #[inline]
    fn default() -> Self {
        Self {
            callbacks: IndexMap::default(),
        }
    }
}

impl<E: Event> EventDispatcher<E> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn add_callback<L: EventListener<E>>(&mut self) {
        self.callbacks
            .insert(PearlId::of::<L>(), Self::callback::<L>);
    }

    #[inline]
    pub fn call(&self, event: &E, pearls: &mut PearlCollection, resources: &mut BobaResources) {
        for callback in self.callbacks.values() {
            callback(event, pearls, resources);
        }
    }

    fn callback<L: EventListener<E>>(
        event: &E,
        pearls: &mut PearlCollection,
        resources: &mut BobaResources,
    ) {
        // SAFETY: We get a pointer to the pearls here so that we can split it.
        // The pearls will essentially be split for each iteration and the WorldView
        // hold the only handle that could incorrectly alias the pearls as an "exclusion".
        // the `WorldView` does not allow modification to the ordering of the collection,
        // and as such, is safe to iterate over while items can still be accessed mutably.
        let pearls_ptr = pearls as *mut PearlCollection;
        let Some(pearl_iter) = pearls.iter_mut::<L>() else { return };
        let pearls = unsafe { &mut *pearls_ptr };

        for (handle, pearl) in pearl_iter {
            let mut view = WorldView {
                exclude: handle.into_raw(),
                pearls,
                resources,
            };

            pearl.callback(event, &mut view);
        }
    }
}
