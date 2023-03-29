use handle_map::Handle;

use crate::pearls::{Pearl, PearlManager};

pub trait Event: Sized + 'static {}
impl<T: Sized + 'static> Event for T {}

pub trait EventListener<E: Event>: Pearl {
    fn callback(handle: &Handle<Self>, pearls: &mut impl PearlManager, data: &E);
}

pub trait EventRegistrar<T: Pearl> {
    fn listen_for<E: Event>(&mut self)
    where
        T: EventListener<E>;
}
