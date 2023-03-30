use handle_map::Handle;

use crate::{pearls::Pearl, World};

/// A blanket trait that is automatically implemented for all items that are `Sized + `static`.
pub trait Event: Sized + 'static {}
impl<T: Sized + 'static> Event for T {}

/// The trait that must be implemented to be registered with an [`EventRegistry`][super::EventRegistry]
pub trait EventListener<E: Event>: Pearl {
    fn callback(handle: &Handle<Self>, data: &E, world: &mut World);
}

/// Trait to hide the struct that is passed into an [`EventListener`] `callback()`.
///
/// This is to prevent any modification to the registry outside the desired methods while registering a [`Pearl`].
pub trait EventRegistrar<T: Pearl> {
    fn listen_for<E: Event>(&mut self)
    where
        T: EventListener<E>;
}
