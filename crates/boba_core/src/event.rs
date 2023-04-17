use crate::pearls::{
    map::{EventData, PearlData},
    Pearl,
};

/// A blanket trait that is automatically implemented for all items that are ``Sized + `static``.
/// This is used as a simple tag for what items may be used to trigger an event in boba engine.
pub trait Event: Sized + 'static {}
impl<T: Sized + 'static> Event for T {}

/// The trait that must be implemented to be registered with an [`EventRegistry`][super::EventRegistry]
pub trait EventListener<E: Event>: Pearl {
    fn callback(pearl: &mut PearlData<Self>, event: EventData<E>);
}

/// Trait to hide the struct that is passed into an [`EventListener`] `callback()`.
///
/// This is to prevent any modification to the registry outside the desired methods while registering a [`Pearl`].
pub trait EventRegistrar<P: Pearl> {
    fn listen_for<E: Event>(&mut self)
    where
        P: EventListener<E>;
}
