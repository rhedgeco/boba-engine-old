use std::any::TypeId;

use crate::EventRegistrar;

use super::map::{Handle, PearlData};

pub trait PearlProvider {
    fn get<P: Pearl>(&self, handle: Handle<P>) -> Option<&P>;
    fn get_mut<P: Pearl>(&mut self, handle: Handle<P>) -> Option<&mut P>;
}

#[allow(unused_variables)]
pub trait Pearl: Sized + 'static {
    fn register(registrar: &mut impl EventRegistrar<Self>) {}
    fn on_insert(handle: Handle<Self>, pearls: &mut impl PearlProvider) {}
    fn on_remove(pearl: &mut PearlData<Self>, pearls: &mut impl PearlProvider) {}
}

/// A light wrapper over [`TypeId`] that is limited to types that derive [`Pearl`]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PearlId(TypeId);

impl PearlId {
    /// Returns the id for pearl of type `T`
    #[inline]
    pub fn of<P: Pearl>() -> Self {
        Self(TypeId::of::<P>())
    }

    /// Returns the underlying [`TypeId`]
    #[inline]
    pub fn into_raw(self) -> TypeId {
        self.0
    }
}

pub trait PearlExt: Pearl {
    fn id() -> PearlId;
    fn pearl_id(&self) -> PearlId;
    fn is<P: Pearl>(&self) -> bool;
    fn downcast<P: Pearl>(self) -> Result<P, Self>;
}

impl<T: Pearl> PearlExt for T {
    fn id() -> PearlId {
        PearlId::of::<T>()
    }

    fn pearl_id(&self) -> PearlId {
        T::id()
    }

    fn is<P: Pearl>(&self) -> bool {
        P::id() == T::id()
    }

    fn downcast<P: Pearl>(self) -> Result<P, Self> {
        if self.is::<P>() {
            let ptr = std::ptr::addr_of!(self);
            return Ok(unsafe { (ptr as *const P).read() });
        }

        Err(self)
    }
}
