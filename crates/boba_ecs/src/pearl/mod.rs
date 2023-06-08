pub mod id;
pub mod set;

pub use id::PearlId;
pub use set::PearlSet;

/// Blanket marker trait for all valid pearl types that can be attached to an entity.
pub trait Pearl: 'static + Sized + Send + Sync {
    fn id() -> PearlId;
    fn pearl_id(&self) -> PearlId;
}

impl<P: 'static + Sized + Send + Sync> Pearl for P {
    /// Returns the [`PearlId`] for this type.
    fn id() -> PearlId {
        PearlId::of::<P>()
    }

    /// Returns the [`PearlId`] for this type.
    fn pearl_id(&self) -> PearlId {
        P::id()
    }
}
