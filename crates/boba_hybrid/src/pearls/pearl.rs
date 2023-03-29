use std::any::TypeId;

use handle_map::Handle;

use crate::events::EventRegistrar;

pub trait Pearl: Sized + 'static {
    fn register(registrar: &mut impl EventRegistrar<Self>);
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PearlId(TypeId);

impl PearlId {
    /// Returns the id for pearl of type `T`
    #[inline]
    pub fn of<T: Pearl>() -> Self {
        Self(TypeId::of::<T>())
    }

    /// Returns the underlying [`TypeId`]
    #[inline]
    pub fn into_raw(self) -> TypeId {
        self.0
    }
}

pub trait PearlManager: 'static {
    fn insert<T: Pearl>(&mut self, pearl: T) -> Handle<T>;
    fn contains<T: Pearl>(&self, handle: &Handle<T>) -> bool;
    fn get<T: Pearl>(&self, handle: &Handle<T>) -> Option<&T>;
    fn get_mut<T: Pearl>(&mut self, handle: &Handle<T>) -> Option<&mut T>;
    fn remove<T: Pearl>(&mut self, handle: &Handle<T>) -> Option<T>;
    fn get_slice<T: Pearl>(&self) -> Option<&[T]>;
    fn get_slice_mut<T: Pearl>(&mut self) -> Option<&mut [T]>;
    fn get_handles<T: Pearl>(&self) -> Option<&[Handle<T>]>;
}
