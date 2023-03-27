use std::{any::TypeId, hash::Hash};

/// A lightweight wrapper around [`TypeId`] that is restricted to types that implement [`Pearl`]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct PearlId(pub(super) TypeId);

impl PearlId {
    /// Returns the pearl id for type `T`
    #[inline]
    pub const fn of<T: Pearl>() -> Self {
        PearlId(TypeId::of::<T>())
    }

    /// Returns the underlying [`TypeId`]
    #[inline]
    pub fn raw_type_id(&self) -> TypeId {
        self.0
    }
}

/// An trait that covers types that are `Send + Sync + 'static`
///
/// This is automatically implemented for all types that meet the requirements
/// and allows automatic integration with the [`PearlId`] api.
pub trait Pearl: Send + Sync + 'static {}
impl<T: Send + Sync + 'static> Pearl for T {}
