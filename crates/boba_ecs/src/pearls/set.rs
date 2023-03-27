use std::{hash::Hash, mem::replace};

use super::{AnyPearl, Pearl, PearlId};

/// A collection of [`PearlId`] structs stored for quick access
#[derive(Default, Clone, Debug, Eq)]
pub struct PearlIdSet {
    ids: Vec<PearlId>,
}

impl Hash for PearlIdSet {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for t in self.ids.iter() {
            t.hash(state)
        }
    }
}

impl PartialEq for PearlIdSet {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.ids == other.ids
    }
}

impl PearlIdSet {
    /// Returns a new empty set
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new set containing type `T`
    #[inline]
    pub fn new_with<T: Pearl>() -> Self {
        Self::new_with_id(PearlId::of::<T>())
    }

    /// Returns a new set containing `id`
    #[inline]
    pub fn new_with_id(id: PearlId) -> Self {
        let mut ids = Vec::new();
        ids.push(id);
        Self { ids }
    }

    /// Returns the length of the set
    #[inline]
    pub fn len(&self) -> usize {
        self.ids.len()
    }

    /// Returns `true` if the set is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    /// Returns the index of type `T` in this set as `Ok(usize)`.
    ///
    /// Returns `Err(usize)` if the id doesnt exist, where usize is where the index *would* be
    #[inline]
    pub fn index_of<T: Pearl>(&self) -> Result<usize, usize> {
        self.index_of_id(&PearlId::of::<T>())
    }

    /// Returns the index of `id` in this set as `Ok(usize)`.
    ///
    /// Returns `Err(usize)` if the id doesnt exist, where usize is where the index *would* be
    #[inline]
    pub fn index_of_id(&self, id: &PearlId) -> Result<usize, usize> {
        self.ids.binary_search(&id)
    }

    /// Returns `true` if type `T` is in this set
    #[inline]
    pub fn contains<T: Pearl>(&self) -> bool {
        self.contains_id(&PearlId::of::<T>())
    }

    /// Returns `true` if `id` is in this set
    #[inline]
    pub fn contains_id(&self, id: &PearlId) -> bool {
        self.index_of_id(id).is_ok()
    }

    /// Inserts type `T` into this set, returning its index as `Ok(usize)`
    ///
    /// If type `T` already exists in this set, returns `Err(usize)` with its current index.
    #[inline]
    pub fn insert<T: Pearl>(&mut self) -> Result<usize, usize> {
        self.insert_id(PearlId::of::<T>())
    }

    /// Inserts `id` into the set, returning its index as `Ok(usize)`
    ///
    /// If `id` already exists in the set, returns `Err(usize)` with its current index.
    #[inline]
    pub fn insert_id(&mut self, id: PearlId) -> Result<usize, usize> {
        match self.index_of_id(&id) {
            Ok(index) => Err(index),
            Err(index) => {
                self.ids.insert(index, id);
                Ok(index)
            }
        }
    }

    /// Removes type `T` from this set, returning its old index as `Some(usize)`
    ///
    /// Returns `None` if type `T` does not exist in this set
    #[inline]
    pub fn remove<T: Pearl>(&mut self) -> Option<usize> {
        self.remove_id(&PearlId::of::<T>())
    }

    /// Removes `id` from this set, returning its old index as `Some(usize)`
    ///
    /// Returns `None` if `id` does not exist in this set
    #[inline]
    pub fn remove_id(&mut self, id: &PearlId) -> Option<usize> {
        let index = self.index_of_id(id).ok()?;
        self.ids.remove(index);
        Some(index)
    }

    /// Returns a reference to the [`PearlId`] objects in this set as a slice reference
    #[inline]
    pub fn as_slice(&self) -> &[PearlId] {
        &self.ids
    }

    /// Returns `true` if `other` shares all its items with this set
    pub fn contains_set(&self, other: &PearlIdSet) -> bool {
        // if the other set is larger than this one
        // then this set could not possibly contain the other
        if self.len() < other.len() {
            return false;
        }

        // get an iterator over the other set and get its first id
        // if there are no ids in the other set, then it is automatically a subset
        let mut other_iter = other.as_slice().iter();
        let Some(mut find_type) = other_iter.next() else { return true };

        // iterate over all of self ids to search for ids in other
        // since all ids are always sorted, we can do this in O(n) time
        for t in self.as_slice().iter() {
            // if we find a type that is greater than the one we are searching for
            // we can assume that it is not in this vec because the vec is in sorted order
            if t > find_type {
                return false;
            }

            // if the ids dont match, continue to the next item in self types
            if t != find_type {
                continue;
            }

            // if the ids do match, retrieve the next id we need to search for
            // if it is None, that means we found all the items and return true
            let Some(new_find_type) = other_iter.next() else { return true };
            find_type = new_find_type;
        }

        // false is only returned here when we have exhausted all self ids
        // and there are still ids in other waiting to be matched
        false
    }
}

/// A collection of [`Pearl`] objects stored for quick access
#[derive(Default)]
pub struct PearlSet {
    ids: PearlIdSet,
    pearls: Vec<AnyPearl>,
}

impl PearlSet {
    /// Returns a new set
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new set containing `pearl`
    #[inline]
    pub fn new_with<T: Pearl>(pearl: T) -> Self {
        let ids = PearlIdSet::new_with::<T>();
        let mut pearls = Vec::new();
        pearls.push(AnyPearl::new(pearl));
        Self { ids, pearls }
    }

    /// Returns the underlying [`PearlIdSet`] for this collection
    #[inline]
    pub fn id_set(&self) -> &PearlIdSet {
        &self.ids
    }

    /// Inserts or replaces `pearl` of type `T` in this set.
    ///
    /// If an item is replaced, it is returned as `Some(T)`
    #[inline]
    pub fn insert_or_replace<T: Pearl>(&mut self, pearl: T) -> Option<T> {
        match self.ids.insert::<T>() {
            Ok(index) => {
                self.pearls.insert(index, AnyPearl::new(pearl));
                None
            }
            Err(index) => {
                let replaced = replace(self.pearls.get_mut(index).unwrap(), AnyPearl::new(pearl));
                Some(replaced.downcast::<T>().unwrap())
            }
        }
    }

    /// Inserts or replaces an [`AnyPearl`] in this set.
    #[inline]
    pub fn insert_or_replace_any_pearl(&mut self, pearl: AnyPearl) -> Option<AnyPearl> {
        match self.ids.insert_id(pearl.id()) {
            Ok(index) => {
                self.pearls.insert(index, pearl);
                None
            }
            Err(index) => {
                let replaced = replace(self.pearls.get_mut(index).unwrap(), pearl);
                Some(replaced)
            }
        }
    }

    /// Returns a reference to the item of type `T` in this set.
    ///
    /// Returns `None` if type `T` does not exist
    #[inline]
    pub fn get<T: Pearl>(&self) -> Option<&T> {
        let index = self.ids.index_of::<T>().ok()?;
        Some(self.pearls[index].downcast_ref::<T>().unwrap())
    }

    /// Returns a mutable reference to the item of type `T` in this set.
    ///
    /// Returns `None` if type `T` does not exist
    #[inline]
    pub fn get_mut<T: Pearl>(&mut self) -> Option<&mut T> {
        let index = self.ids.index_of::<T>().ok()?;
        Some(self.pearls[index].downcast_mut::<T>().unwrap())
    }

    /// Removes an item of type `T` from this set, returning it as `Some(T)`
    ///
    /// Returns `None` if type `T` does not exist.
    #[inline]
    pub fn remove<T: Pearl>(&mut self) -> Option<T> {
        let index = self.ids.remove::<T>()?;
        Some(self.pearls.remove(index).downcast::<T>().unwrap())
    }

    /// Consumes this set and returns an owned [`Vec`] of [`AnyPearl`] objects
    #[inline]
    pub fn into_vec(self) -> Vec<AnyPearl> {
        self.pearls
    }
}
