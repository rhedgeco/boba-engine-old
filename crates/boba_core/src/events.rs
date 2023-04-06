use crate::{pearls::Pearl, WorldView};

/// A blanket trait that is automatically implemented for all items that are `Sized + `static`.
pub trait Event: Sized + 'static {}
impl<T: Sized + 'static> Event for T {}

/// The trait that must be implemented to be registered with an [`EventRegistry`][super::EventRegistry]
pub trait EventListener<E: Event>: Pearl {
    fn callback(&mut self, event: &E, world: &mut WorldView);
}

/// Trait to hide the struct that is passed into an [`EventListener`] `callback()`.
///
/// This is to prevent any modification to the registry outside the desired methods while registering a [`Pearl`].
pub trait EventRegistrar<P: Pearl> {
    fn listen_for<E: Event>(&mut self)
    where
        P: EventListener<E>;
}
