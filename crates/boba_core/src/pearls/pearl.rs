use std::any::TypeId;

use crate::EventRegistrar;

/// Central trait to register structs in boba engine.
pub trait Pearl: Sized + 'static {
    #[allow(unused_variables)]
    fn register(registrar: &mut impl EventRegistrar<Self>) {}
}

/// A light wrapper over [`TypeId`] that is limited to types that derive [`Pearl`]
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

pub trait PearlExt: Pearl {
    fn id() -> PearlId;
    fn pearl_id(&self) -> PearlId;
    fn into_concrete<P: Pearl>(self) -> Result<P, Self>;
}

impl<T: Pearl> PearlExt for T {
    #[inline]
    fn id() -> PearlId {
        PearlId::of::<T>()
    }

    #[inline]
    fn pearl_id(&self) -> PearlId {
        T::id()
    }

    fn into_concrete<P: Pearl>(self) -> Result<P, Self> {
        if T::id() == P::id() {
            let ptr = Box::into_raw(Box::new(self)) as *mut P;
            return Ok(unsafe { *Box::from_raw(ptr) });
        }

        return Err(self);
    }
}
