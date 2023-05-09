use std::{
    any::TypeId,
    cmp::Ordering,
    fmt::{Display, Formatter},
};

use super::Pearl;

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

/// A set of unique [`PearlId`] structs.
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PearlIdSet {
    pub(crate) ids: Vec<PearlId>,
}

impl PearlIdSet {
    /// Returns a new empty id set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new set containing the `PearlId` of `P`.
    pub fn new_with<P: Pearl>() -> Self {
        Self { ids: vec![P::id()] }
    }

    /// Returns a new set containing `id`.
    pub fn new_with_id(id: PearlId) -> Self {
        Self { ids: vec![id] }
    }

    /// Returns the length of the set.
    pub fn len(&self) -> usize {
        self.ids.len()
    }

    /// Returns `true` if the set contains no elements.
    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    /// Returns all underlying [`PearlId`] structs as a immutable slice.
    pub fn as_slice(&self) -> &[PearlId] {
        &self.ids
    }

    /// Returns `Some` if the set contains [`PearlId`] of type `P`.
    /// The returned `usize` is the index of the corresponding id.
    pub fn contains<P: Pearl>(&self) -> Option<usize> {
        self.contains_id(&P::id())
    }

    /// Returns `Some` if the set contains `id`.
    /// The returned `usize` is the index of the corresponding id.
    pub fn contains_id(&self, id: &PearlId) -> Option<usize> {
        match self.ids.binary_search(id) {
            Ok(index) => Some(index),
            _ => None,
        }
    }

    /// Inserts a [`PearlId`] of type `P` into this set and returns its index.
    /// If the id already exists in the set, the index is returned in an `Err`.
    pub fn insert<P: Pearl>(&mut self) -> Result<usize, usize> {
        self.insert_id(P::id())
    }

    /// Inserts `id` into this set and returns its index.
    /// If the id already exists in the set, the index is returned in an `Err`.
    pub fn insert_id(&mut self, id: PearlId) -> Result<usize, usize> {
        match self.ids.binary_search(&id) {
            Err(index) => {
                self.ids.insert(index, id);
                Ok(index)
            }
            Ok(index) => Err(index),
        }
    }

    /// Removes the [`PearlId`] of type `P` from this set and returns its old index.
    /// Returns `None` if the pearl does not exist in this set.
    pub fn remove<P: Pearl>(&mut self) -> Option<usize> {
        self.remove_id(&P::id())
    }

    /// Removes `id` from this set and returns its old index.
    /// Returns `None` if the pearl does not exist in this set.
    pub fn remove_id(&mut self, id: &PearlId) -> Option<usize> {
        match self.ids.binary_search(id) {
            Ok(index) => {
                self.ids.remove(index);
                Some(index)
            }
            _ => None,
        }
    }

    /// Returns `true` if this set contains every [`PearlId`] present in `other`.
    pub fn contains_set(&self, other: &PearlIdSet) -> bool {
        // other could not possibly be contained if it is bigger
        if other.len() > self.len() {
            return false;
        }

        // get iterators over both sets
        let self_iter = self.ids.iter();
        let mut other_iter = other.as_slice().iter();
        let Some(mut other_id) = other_iter.next() else { return true };

        // iterate over both sets
        for self_id in self_iter {
            match self_id.cmp(&other_id) {
                // if self is less, do nothing and continue
                // self may contain other_id down the line
                Ordering::Less => (),

                // if they are equal, get next from other iter
                // if other iter doesnt have any more, we matched them all
                Ordering::Equal => match other_iter.next() {
                    Some(next) => other_id = next,
                    None => return true,
                },

                // if self is greater, early return false since ids are sorted.
                // once self is greater it will never contain any less than items
                Ordering::Greater => return false,
            }
        }

        // if we make it out of the loop,
        // it is because we never matched all the other items
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPearl1;
    struct TestPearl2;
    struct TestPearl3;

    #[test]
    fn id() {
        let type_id = TypeId::of::<TestPearl1>();
        let id = TestPearl1.pearl_id();
        assert!(type_id == id.type_id)
    }

    #[test]
    fn set_manipulation() {
        let mut ids = PearlIdSet::new();
        assert!(ids.len() == 0);
        assert!(ids.is_empty());

        ids.insert::<TestPearl1>().ok().unwrap();
        ids.insert::<TestPearl1>().err().unwrap();
        ids.insert::<TestPearl2>().ok().unwrap();
        ids.insert_id(PearlId::of::<TestPearl3>()).ok().unwrap();
        assert!(ids.len() == 3);
        assert!(!ids.is_empty());

        ids.remove::<TestPearl1>().unwrap();
        ids.remove_id(&TestPearl3.pearl_id()).unwrap();
        assert!(ids.len() == 1);
        assert!(!ids.is_empty());

        ids.remove::<TestPearl2>().unwrap();
        assert!(ids.len() == 0);
        assert!(ids.is_empty());
    }

    #[test]
    fn contains() {
        let mut set1 = PearlIdSet::new();
        set1.insert::<TestPearl1>().ok().unwrap();
        set1.insert::<TestPearl3>().ok().unwrap();

        assert!(set1.contains::<TestPearl1>().is_some());
        assert!(set1.contains::<TestPearl2>().is_none());
        assert!(set1.contains_id(&TestPearl3.pearl_id()).is_some());
    }

    #[test]
    fn contains_set() {
        let mut set1 = PearlIdSet::new();
        set1.insert::<TestPearl1>().ok().unwrap();
        set1.insert::<TestPearl2>().ok().unwrap();
        set1.insert::<TestPearl3>().ok().unwrap();

        let mut set2 = PearlIdSet::new();
        assert!(set1.contains_set(&set2));
        assert!(!set2.contains_set(&set1));

        set2.insert::<TestPearl2>().ok().unwrap();
        set2.insert::<TestPearl3>().ok().unwrap();

        assert!(set1.contains_set(&set2));
        assert!(!set2.contains_set(&set1));
    }
}
