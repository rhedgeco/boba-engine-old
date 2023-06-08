pub mod map;
pub mod set;

pub use map::PearlIdMap;
pub use set::PearlIdSet;

use std::{
    any::TypeId,
    fmt::{Display, Formatter},
};

use crate::Pearl;

/// Light wrapper around [`TypeId`] that can only be created from a valid [`Pearl`] struct.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PearlId {
    pub(crate) type_id: TypeId,
}

impl PearlId {
    /// Returns a new `PearlId` of type `P`.
    pub fn of<P: Pearl>() -> Self {
        Self {
            type_id: TypeId::of::<P>(),
        }
    }

    /// Returns the underlying [`TypeId`].
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }
}

impl Display for PearlId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.type_id)
    }
}
