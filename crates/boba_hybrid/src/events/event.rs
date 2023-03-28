use handle_map::Handle;

use crate::pearls::{Pearl, PearlAccessor};

pub trait Event: Sized + 'static {}
impl<T: Sized + 'static> Event for T {}

pub trait EventListener<E: Event>: Pearl {
    fn callback(handle: &Handle<Self>, pearls: &mut impl PearlAccessor, data: &E);
}
