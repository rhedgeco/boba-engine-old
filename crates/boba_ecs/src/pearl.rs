use std::{any::TypeId, hash::Hash};

use imposters::Imposter;
use reterse::{continue_if, return_if, return_if_err, return_if_none};

/// A lightweight wrapper around [`TypeId`] that is restricted to types that implement [`Pearl`]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct PearlId(TypeId);

impl PearlId {
    /// Returns the pearl id for type `T`
    #[inline]
    pub fn of<T: Pearl>() -> Self {
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

#[derive(Default)]
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
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn new_with<T: Pearl>() -> Self {
        Self::new_with_id(PearlId::of::<T>())
    }

    #[inline]
    pub fn new_with_id(id: PearlId) -> Self {
        let mut ids = Vec::new();
        ids.push(id);
        Self { ids }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.ids.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    #[inline]
    pub fn index_of<T: Pearl>(&self) -> Result<usize, usize> {
        self.index_of_id(&PearlId::of::<T>())
    }

    #[inline]
    pub fn index_of_id(&self, id: &PearlId) -> Result<usize, usize> {
        self.ids.binary_search(&id)
    }

    #[inline]
    pub fn contains<T: Pearl>(&self) -> bool {
        self.contains_id(&PearlId::of::<T>())
    }

    #[inline]
    pub fn contains_id(&self, id: &PearlId) -> bool {
        self.index_of_id(id).is_ok()
    }

    #[inline]
    pub fn insert<T: Pearl>(&mut self) -> Result<usize, usize> {
        self.insert_id(PearlId::of::<T>())
    }

    /// Inserts a [`PearlId`] into the set, returning its index as `Ok(usize)`
    ///
    /// If the id already exists in the set, the index of the existing item will be returned as `Err(usize)`
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

    #[inline]
    pub fn remove<T: Pearl>(&mut self) -> Option<usize> {
        self.remove_id(&PearlId::of::<T>())
    }

    #[inline]
    pub fn remove_id(&mut self, id: &PearlId) -> Option<usize> {
        let index = return_if_err!(self.index_of_id(id), _ => None);
        self.ids.remove(index);
        Some(index)
    }

    #[inline]
    pub fn as_slice(&self) -> &[PearlId] {
        &self.ids
    }

    pub fn contains_set(&self, other: &PearlIdSet) -> bool {
        // if the other set is larger than this one
        // then this set could not possibly contain the other
        return_if!(self.len() < other.len() => false);

        // get an iterator over the other set and get its first id
        // if there are no ids in the other set, then it is automatically a subset
        let mut other_iter = other.as_slice().iter();
        let mut find_type = return_if_none!(other_iter.next() => true);

        // iterate over all of self ids to search for ids in other
        // since all ids are always sorted, we can do this in O(n) time
        for t in self.as_slice().iter() {
            // if we find a type that is greater than the one we are searching for
            // we can assume that it is not in this vec because the vec is in sorted order
            return_if!(t > find_type => false);

            // if the ids dont match, continue to the next item in self types
            continue_if!(t != find_type);

            // if the ids do match, retrieve the next id we need to search for
            // if it is None, that means we found all the items and return true
            find_type = return_if_none!(other_iter.next() => true);
        }

        // false is only returned here when we have exhausted all self ids
        // and there are still ids in other waiting to be matched
        false
    }
}

#[derive(Default)]
pub struct PearlSet {
    ids: PearlIdSet,
    pearls: Vec<Imposter>,
}

impl PearlSet {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn new_with<T: Pearl>(pearl: T) -> Self {
        let ids = PearlIdSet::new_with::<T>();
        let mut pearls = Vec::new();
        pearls.push(Imposter::new(pearl));
        Self { ids, pearls }
    }

    #[inline]
    pub fn id_set(&self) -> &PearlIdSet {
        &self.ids
    }

    #[inline]
    pub fn insert<T: Pearl>(&mut self, pearl: T) -> Result<(), T> {
        let index = return_if_err!(self.ids.insert::<T>(), _ => Err(pearl));
        self.pearls.insert(index, Imposter::new(pearl));
        Ok(())
    }

    #[inline]
    pub fn get<T: Pearl>(&self) -> Option<&T> {
        let index = self.ids.index_of::<T>().ok()?;
        Some(self.pearls[index].downcast_ref::<T>().unwrap())
    }

    #[inline]
    pub fn get_mut<T: Pearl>(&mut self) -> Option<&mut T> {
        let index = self.ids.index_of::<T>().ok()?;
        Some(self.pearls[index].downcast_mut::<T>().unwrap())
    }

    #[inline]
    pub fn remove<T: Pearl>(&mut self) -> Option<T> {
        let index = self.ids.remove::<T>()?;
        Some(self.pearls.remove(index).downcast::<T>().unwrap())
    }

    #[inline]
    pub fn into_vec(self) -> Vec<Imposter> {
        self.pearls
    }
}
