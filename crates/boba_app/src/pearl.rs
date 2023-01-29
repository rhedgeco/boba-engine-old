use std::{
    cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut},
    hash::Hash,
    rc::Rc,
};

use crate::BobaId;

struct InnerPearl<T> {
    id: BobaId,
    data: RefCell<T>,
}

/// Core struct for wrapping data to connected to a node
///
/// While this could technically be cloned, it is prevented from doing so as to not connect the same pearl to multiple nodes.
/// Instead, the pearl can be built into a [`PearlLink`] which has essentially the same functionality, but can be cloned and used across anywhere it is needed.
/// However, links cannot be added directly to a node.
pub struct Pearl<T> {
    inner: Rc<InnerPearl<T>>,
}

/// A link to the core [`Pearl`] struct.
///
/// This can be cloned, and should be used when data needs to be linked across pearls.
#[derive(Clone)]
pub struct PearlLink<T> {
    inner: Rc<InnerPearl<T>>,
}

impl<T> Eq for PearlLink<T> {}

impl<T> PartialEq for PearlLink<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.id == other.inner.id
    }
}

impl<T> Hash for PearlLink<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.id.hash(state);
    }
}

impl<T> Pearl<T> {
    /// Creates a new `Pearl` containing `data`
    pub fn new(data: T) -> Self {
        let inner = InnerPearl {
            id: BobaId::new(),
            data: RefCell::new(data),
        };

        Self {
            inner: Rc::new(inner),
        }
    }

    /// Gets the [`BobaId`] for this pearl
    pub fn id(&self) -> &BobaId {
        &self.inner.id
    }

    /// Creates a new [`PearlLink`] for this pearl
    pub fn new_link(&self) -> PearlLink<T> {
        PearlLink {
            inner: self.inner.clone(),
        }
    }

    /// Borrows the underlying data from this pearl
    pub fn borrow(&self) -> Result<Ref<T>, BorrowError> {
        self.inner.data.try_borrow()
    }

    /// Mutably borrows the underlying data from this pearl
    pub fn borrow_mut(&self) -> Result<RefMut<T>, BorrowMutError> {
        self.inner.data.try_borrow_mut()
    }
}

impl<T> PearlLink<T> {
    /// Gets the [`BobaId`] for the [`Pearl`] this link points to
    pub fn id(&self) -> &BobaId {
        &self.inner.id
    }

    /// Borrows the underlying data from the [`Pearl`] this link points to
    pub fn borrow(&self) -> Result<Ref<T>, BorrowError> {
        self.inner.data.try_borrow()
    }

    /// Mutably borrows the underlying data from the [`Pearl`] this link points to
    pub fn borrow_mut(&self) -> Result<RefMut<T>, BorrowMutError> {
        self.inner.data.try_borrow_mut()
    }
}
