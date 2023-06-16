pub mod map;
pub mod set;

pub use map::ComponentIdMap;
pub use set::ComponentIdSet;

use std::{
    any::TypeId,
    fmt::{Display, Formatter},
};

use crate::Component;

/// Light wrapper around [`TypeId`] that can only be created from a valid [`Component`] struct.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ComponentId {
    pub(crate) type_id: TypeId,
}

impl ComponentId {
    /// Returns a new `ComponentId` of type `P`.
    pub fn of<T: Component>() -> Self {
        Self {
            type_id: TypeId::of::<T>(),
        }
    }

    /// Returns the underlying [`TypeId`].
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }
}

impl Display for ComponentId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.type_id)
    }
}
