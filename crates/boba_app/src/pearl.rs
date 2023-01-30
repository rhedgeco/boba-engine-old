use std::{
    cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut},
    hash::Hash,
    ops::Deref,
    rc::Rc,
};

use crate::BobaId;

/// The internal structure that is shared between [`Pearl`] and [`PearlLink`]
pub struct PearlCore<T> {
    id: BobaId,
    data: RefCell<T>,
}

impl<T> PearlCore<T> {
    /// Gets the [`BobaId`] for this pearl
    pub fn id(&self) -> &BobaId {
        &self.id
    }

    /// Borrows the underlying data from this pearl
    pub fn borrow(&self) -> Result<Ref<T>, BorrowError> {
        self.data.try_borrow()
    }

    /// Mutably borrows the underlying data from this pearl
    pub fn borrow_mut(&self) -> Result<RefMut<T>, BorrowMutError> {
        self.data.try_borrow_mut()
    }
}

/// Core struct for wrapping data to connected to a node
///
/// While this could technically be cloned, it is prevented from doing so as to not connect the same pearl to multiple nodes.
/// Instead, the pearl can be built into a [`PearlLink`] which has essentially the same functionality, but can be cloned and used across anywhere it is needed.
/// However, links cannot be added directly to a node.
pub struct Pearl<T> {
    core: Rc<PearlCore<T>>,
}

impl<T> PartialEq<PearlLink<T>> for Pearl<T> {
    fn eq(&self, other: &PearlLink<T>) -> bool {
        self.core.id == other.core.id
    }
}

impl<T> Deref for Pearl<T> {
    type Target = PearlCore<T>;

    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

impl<T> Pearl<T> {
    /// Creates a new `Pearl` containing `data`
    pub fn new(data: T) -> Self {
        let inner = PearlCore {
            id: BobaId::new(),
            data: RefCell::new(data),
        };

        Self {
            core: Rc::new(inner),
        }
    }

    /// Creates a new [`PearlLink`] for this pearl
    pub fn new_link(&self) -> PearlLink<T> {
        PearlLink {
            core: self.core.clone(),
        }
    }
}

/// A link to the core [`Pearl`] struct.
///
/// This can be cloned, and should be used when data needs to be linked across pearls.
#[derive(Clone)]
pub struct PearlLink<T> {
    core: Rc<PearlCore<T>>,
}

impl<T> Eq for PearlLink<T> {}

impl<T> PartialEq for PearlLink<T> {
    fn eq(&self, other: &Self) -> bool {
        self.core.id == other.core.id
    }
}

impl<T> PartialEq<Pearl<T>> for PearlLink<T> {
    fn eq(&self, other: &Pearl<T>) -> bool {
        self.core.id == other.core.id
    }
}

impl<T> Hash for PearlLink<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.core.id.hash(state);
    }
}

impl<T> Deref for PearlLink<T> {
    type Target = PearlCore<T>;

    fn deref(&self) -> &Self::Target {
        &self.core
    }
}
