use std::{
    cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut},
    hash::Hash,
    rc::Rc,
};

use crate::{BobaId, Node};

struct PearlCore<T> {
    id: BobaId,
    node: Node,
    data: RefCell<T>,
}

/// Core struct for wrapping data to connected to a node
///
/// While this could technically be cloned, it is prevented from doing so as to not connect the same pearl to multiple [`Node`]s.
/// Instead, the pearl can be built into a [`PearlLink`] which has essentially the same functionality, but can be cloned and used across anywhere it is needed.
/// However, links cannot be added directly to a [`Node`].
pub struct Pearl<T> {
    core: Rc<PearlCore<T>>,
}

impl<T> Eq for Pearl<T> {}

impl<T> PartialEq for Pearl<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl<T> Hash for Pearl<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

impl<T> Clone for Pearl<T> {
    fn clone(&self) -> Self {
        Self {
            core: self.core.clone(),
        }
    }
}

impl<T> Pearl<T> {
    /// Creates a new `Pearl` containing `data`
    pub(crate) fn new(data: T, node: Node) -> Self {
        let inner = PearlCore {
            id: BobaId::new(),
            node,
            data: RefCell::new(data),
        };

        Self {
            core: Rc::new(inner),
        }
    }

    /// Gets the [`BobaId`] for this pearl
    pub fn id(&self) -> &BobaId {
        &self.core.id
    }

    /// Gets the [`Node`] that this pearl is attached to
    pub fn node(&self) -> &Node {
        &self.core.node
    }

    /// Borrows the underlying data from this pearl
    pub fn borrow(&self) -> Result<Ref<T>, BorrowError> {
        self.core.data.try_borrow()
    }

    /// Mutably borrows the underlying data from this pearl
    pub fn borrow_mut(&self) -> Result<RefMut<T>, BorrowMutError> {
        self.core.data.try_borrow_mut()
    }
}
