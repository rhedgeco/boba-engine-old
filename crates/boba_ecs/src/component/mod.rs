pub mod id;
pub mod set;

pub use id::ComponentId;
pub use set::ComponentSet;

/// Blanket marker trait for all valid component types that can be attached to an entity.
pub trait Component: 'static + Sized + Send + Sync {
    fn id() -> ComponentId;
    fn component_id(&self) -> ComponentId;
}

impl<T: 'static + Sized + Send + Sync> Component for T {
    /// Returns the [`ComponentId`] for this type.
    fn id() -> ComponentId {
        ComponentId::of::<T>()
    }

    /// Returns the [`ComponentId`] for this type.
    fn component_id(&self) -> ComponentId {
        T::id()
    }
}
