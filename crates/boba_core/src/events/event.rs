use crate::{
    pearls::{ExclusivePearlCollection, Pearl, PearlLink},
    BobaResources,
};

use super::commands::EventCommands;

/// A blanket trait that is automatically implemented for all items that are ``Sized + `static``.
/// This is used as a simple tag for what items may be used to trigger an event in boba engine.
pub trait Event: Sized + 'static {}
impl<T: Sized + 'static> Event for T {}

/// The trait that must be implemented to be registered with an [`EventRegistry`][super::EventRegistry]
pub trait EventListener<E: Event>: Pearl {
    fn callback(pearl: PearlLink<Self>, event: EventData<E>);
}

/// Trait to hide the struct that is passed into an [`EventListener`] `callback()`.
///
/// This is to prevent any modification to the registry outside the desired methods while registering a [`Pearl`].
pub trait EventRegistrar<P: Pearl> {
    fn listen_for<E: Event>(&mut self)
    where
        P: EventListener<E>;
}

/// A window into the resources stored in a world.
///
/// This is used internally in [`EventListener`] callbacks
///  for events to provide access to the other pearls and resources in the world.
pub struct EventData<'a, E: Event> {
    pub data: &'a E,
    pub pearls: &'a mut ExclusivePearlCollection<'a>,
    pub resources: &'a mut BobaResources,
    pub commands: &'a mut EventCommands,
}

impl<'a, E: Event> EventData<'a, E> {
    pub fn new(
        event: &'a E,
        pearls: &'a mut ExclusivePearlCollection<'a>,
        resources: &'a mut BobaResources,
        commands: &'a mut EventCommands,
    ) -> Self {
        Self {
            data: event,
            pearls,
            resources,
            commands,
        }
    }
}
