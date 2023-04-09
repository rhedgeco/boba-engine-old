use crate::{
    pearls::{ExclusivePearlProvider, Pearl},
    BobaResources,
};

use super::commands::EventCommands;

/// A blanket trait that is automatically implemented for all items that are ``Sized + `static``.
/// This is used as a simple tag for what items may be used to trigger an event in boba engine.
pub trait Event: Sized + 'static {}
impl<T: Sized + 'static> Event for T {}

/// The trait that must be implemented to be registered with an [`EventRegistry`][super::EventRegistry]
pub trait EventListener<E: Event>: Pearl {
    fn callback(&mut self, event: &E, world: EventView<Self>);
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
pub struct EventView<'a, P: Pearl> {
    pub pearls: &'a mut ExclusivePearlProvider<'a, P>,
    pub resources: &'a mut BobaResources,
    pub commands: &'a mut EventCommands,
}

impl<'a, P: Pearl> EventView<'a, P> {
    pub fn new(
        pearls: &'a mut ExclusivePearlProvider<'a, P>,
        resources: &'a mut BobaResources,
        commands: &'a mut EventCommands,
    ) -> Self {
        Self {
            pearls,
            resources,
            commands,
        }
    }
}
