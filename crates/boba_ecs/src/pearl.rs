use hashbrown::HashMap;
use imposters::Imposter;
use std::{any::TypeId, hash::Hash, mem::replace, vec::IntoIter};

/// An empty trait that covers types that are `Send + Sync + 'static`
///
/// This is automatically implemented for all types that meet the requirements
/// and allows automatic integration with the [`PearlId`] api.
pub trait Pearl: Send + Sync + 'static {}

impl<T: Send + Sync + 'static> Pearl for T {}

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
    pub fn type_id(&self) -> TypeId {
        self.0
    }
}

/// A collection of [`PearlId`] structs stored in sorted order
///
/// Useful for identification of what kinds of "pearls" are stored in an [`Archetype`]
#[derive(Eq, Clone, Default)]
pub struct PearlTypes {
    type_set: HashMap<PearlId, usize>,
    type_vec: Vec<PearlId>,
}

impl Hash for PearlTypes {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for t in self.type_vec.iter() {
            t.hash(state)
        }
    }
}

impl PartialEq for PearlTypes {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.type_vec == other.type_vec
    }
}

impl PearlTypes {
    /// Creates a new `PearlTypes` struct
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `PearlTypes` struct initialized with a single type `T`
    #[inline]
    pub fn new_single<T: Pearl>() -> Self {
        Self::new_single_id(PearlId::of::<T>())
    }

    /// Creates a new `PearlTypes` struct initialized with `id`
    #[inline]
    pub fn new_single_id(id: PearlId) -> Self {
        let mut new = Self::new();
        new.type_set.insert(id, 0);
        new.type_vec.push(id);
        new
    }

    /// Returns the length of the types set
    #[inline]
    pub fn len(&self) -> usize {
        self.type_vec.len()
    }

    /// Returns `true` if this set is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.type_vec.is_empty()
    }

    /// Returns `true` if the set contains the type `T`
    #[inline]
    pub fn contains<T: Pearl>(&self) -> bool {
        self.contains_id(&PearlId::of::<T>())
    }

    /// Returns `true` if the set contains `id`
    #[inline]
    pub fn contains_id(&self, id: &PearlId) -> bool {
        self.type_set.contains_key(id)
    }

    /// Returns `Some(usize)` with the index of type `T`
    ///
    /// Returns `None` if type `T` does not exist
    #[inline]
    pub fn index_of<T: Pearl>(&self) -> Option<usize> {
        self.index_of_id(&PearlId::of::<T>())
    }

    /// Returns `Some(usize)` with the index of `id`
    ///
    /// Returns `None` if `id` does not exist
    #[inline]
    pub fn index_of_id(&self, id: &PearlId) -> Option<usize> {
        Some(*self.type_set.get(id)?)
    }

    /// Removes type `T` from the set and returns `Some(usize)` with the index that it was removed from
    ///
    /// If the item is not in the set, returns `None`
    #[inline]
    pub fn remove<T: Pearl>(&mut self) -> Option<usize> {
        self.remove_id(&PearlId::of::<T>())
    }

    /// Removes `id` from the set and returns `Some(usize)` with the index that it was removed from
    ///
    /// If the item is not in the set, returns `None`
    #[inline]
    pub fn remove_id(&mut self, id: &PearlId) -> Option<usize> {
        let Some(index) = self.type_set.get(id) else { return None };
        self.type_vec.remove(*index);
        Some(*index)
    }

    /// Inserts type `T` into this set and returns `Ok(usize)` with the index it was inserted to
    ///
    /// If the id already existed, returns `Err(usize)` with the index that it already exists at
    #[inline]
    pub fn insert<T: Pearl>(&mut self) -> Result<usize, usize> {
        self.insert_id(PearlId::of::<T>())
    }

    /// Inserts `id` into this set and returns `Ok(usize)` with the index it was inserted to
    ///
    /// If the id already existed, returns `Err(usize)` with the index that it already exists at
    #[inline]
    pub fn insert_id(&mut self, id: PearlId) -> Result<usize, usize> {
        match self.type_vec.binary_search(&id) {
            Ok(index) => Err(index),
            Err(index) => {
                self.type_vec.insert(index, id);
                self.type_set.insert(id, index);
                Ok(index)
            }
        }
    }

    /// Returns true if this set is a superset of `other`
    #[inline]
    pub fn contains_types(&self, other: &PearlTypes) -> bool {
        // if this vec has less elements that the other vec
        // then this vec could not possibly contain the other
        if self.len() < other.len() {
            return false;
        }

        // get an iterator over the other vector
        let mut other_iter = other.type_vec.iter();

        // get the first type we need to find in this vec
        // if there is no types in the other vec, then it is automatically a subset
        let Some(mut find_type) = other_iter.next() else { return true };

        // iterate over all of self types to search for types in other
        // since all types are always sorted, we can do this in O(n) time
        for t in self.type_vec.iter() {
            // if we find a type that is greater than the one we are searching for
            // we can assume that it is not in this vec because the vec is in sorted order
            if t > find_type {
                return false;
            }

            // if they dont match, continue to the next item in self types
            if t != find_type {
                continue;
            }

            // if they do match, retrieve the next item we need to search for
            // if it is None, that means we found all the items and return true
            find_type = match other_iter.next() {
                None => return true,
                Some(t) => t,
            };
        }

        false
    }
}

/// A collection of pearls held in sorted order based on their [`PearlId`]
///
/// Useful for moving pearls out of and into [`Archetype`] objects
#[derive(Default)]
pub struct PearlSet {
    type_link: PearlTypes,
    pearl_vec: Vec<Imposter>,
}

impl PearlSet {
    /// Creates a new pearl set
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new pearl set initilized with `item`
    #[inline]
    pub fn new_single<T: Pearl>(item: T) -> Self {
        let mut pearl_vec = Vec::new();
        pearl_vec.push(Imposter::new(item));
        Self {
            type_link: PearlTypes::new_single::<T>(),
            pearl_vec,
        }
    }

    /// Returns the length of this set
    #[inline]
    pub fn len(&self) -> usize {
        self.pearl_vec.len()
    }

    /// Returns `true` if this set is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.pearl_vec.is_empty()
    }

    /// Returns a reference to the [`PearlTypes`] for this set
    #[inline]
    pub fn types(&self) -> &PearlTypes {
        &self.type_link
    }

    /// Returns a reference to the pearl of type `T`
    ///
    /// Returns `None` if the pearl does not exist
    #[inline]
    pub fn get<T: Pearl>(&self) -> Option<&T> {
        self.get_imposter(&PearlId::of::<T>())?.downcast_ref::<T>()
    }

    /// Returns a mutable reference to the pearl of type `T`
    ///
    /// Returns `None` if the pearl does not exist
    #[inline]
    pub fn get_mut<T: Pearl>(&mut self) -> Option<&mut T> {
        self.get_imposter_mut(&PearlId::of::<T>())?
            .downcast_mut::<T>()
    }

    /// Returns a reference to the pearl with `id`
    ///
    /// Returns `None` if the pearl does not exist
    #[inline]
    pub fn get_imposter(&self, id: &PearlId) -> Option<&Imposter> {
        let Some(index) = self.type_link.index_of_id(id) else { return None };
        Some(&self.pearl_vec[index])
    }

    /// Returns a mutable reference to the pearl with `id`
    ///
    /// Returns `None` if the pearl does not exist
    #[inline]
    pub fn get_imposter_mut(&mut self, id: &PearlId) -> Option<&mut Imposter> {
        let Some(index) = self.type_link.index_of_id(id) else { return None };
        Some(&mut self.pearl_vec[index])
    }

    /// Removes the pearl of type `T` from this set
    ///
    /// Returns `None` if the pearl does not exist
    #[inline]
    pub fn remove<T: Pearl>(&mut self) -> Option<T> {
        self.remove_imposter(&PearlId::of::<T>())?.downcast::<T>()
    }

    /// Removes and drops the pearl with `id` and returns `true`
    ///
    /// Returns `false` if the pearl does not exist
    #[inline]
    pub fn drop_type(&mut self, id: &PearlId) -> bool {
        // dropped implicitly
        self.remove_imposter(id).is_some()
    }

    /// Removes the pearl with `id` and returns an untyped [`Imposter`] that contains the pearl data
    ///
    /// Returns `None` if the pearl does not exist
    #[inline]
    pub fn remove_imposter(&mut self, id: &PearlId) -> Option<Imposter> {
        let index = self.type_link.remove_id(id)?;
        Some(self.pearl_vec.remove(index))
    }

    /// Inserts `item` into the set, replacing it if necessary
    ///
    /// If there was an existing pearl of type `T`,
    /// then it will be replaced and this method will return it as `Some(T)`
    #[inline]
    pub fn insert<T: Pearl>(&mut self, item: T) -> Option<T> {
        self.insert_imposter(Imposter::new(item))?.downcast::<T>()
    }

    /// Inserts `imposter` into the set, replacing it if necessary
    ///
    /// If there was an existing [`Imposter`] with the same [`PearlId`],
    /// then it will be replaced and this method will return it as `Some(Imposter)`
    #[inline]
    pub fn insert_imposter(&mut self, imposter: Imposter) -> Option<Imposter> {
        match self.type_link.insert_id(PearlId(imposter.type_id())) {
            Ok(index) => {
                self.pearl_vec.insert(index, imposter);
                return None;
            }
            Err(index) => {
                let old = replace(&mut self.pearl_vec[index], imposter);
                return Some(old);
            }
        }
    }

    /// Inserts an entire set `other` into this set.
    ///
    /// All duplicate pearls in this set will be replaced by the pearls in `other`
    pub fn insert_set(&mut self, other: PearlSet) {
        if other.pearl_vec.is_empty() {
            return;
        }

        if self.pearl_vec.is_empty() {
            self.type_link = other.type_link;
            self.pearl_vec = other.pearl_vec;
            return;
        }

        // TODO: this can be optimized.
        // each insert does a new hash lookup on the array
        // since they are already in order, they may be able to be interleaved in a faster way
        for imposter in other.pearl_vec.into_iter() {
            self.insert_imposter(imposter);
        }
    }

    /// Creates a consuming iterator over the [`Imposter`] objects
    #[inline]
    pub fn into_iter(self) -> IntoIter<Imposter> {
        self.pearl_vec.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Type1(u16, u32);
    struct Type2(u32, u64);
    struct Type3(u64, u128);
    struct Type4(u128, u64);

    #[test]
    fn pearl_types_sorted() {
        let mut types1 = PearlTypes::new();
        types1.insert::<Type1>().unwrap();
        types1.insert::<Type2>().unwrap();
        types1.insert::<Type3>().unwrap();

        let mut types2 = PearlTypes::new();
        types2.insert::<Type1>().unwrap();
        types2.insert::<Type2>().unwrap();
        types2.insert::<Type3>().unwrap();

        for i in 0..3 {
            let type1 = types1.type_vec[i];
            let type2 = types2.type_vec[i];
            assert!(type1 == type2);
        }
    }

    #[test]
    fn pearl_set_sorted() {
        let mut set1 = PearlSet::new();
        set1.insert(Type1(1, 42));
        set1.insert(Type2(2, 42));
        set1.insert(Type3(3, 42));

        let mut set2 = PearlSet::new();
        set2.insert(Type1(1, 42));
        set2.insert(Type2(2, 42));
        set2.insert(Type3(3, 42));

        assert!(set2.types() == set1.types());

        for i in 0..3 {
            let type1 = set1.type_link.type_vec[i];
            let imposter1 = &set1.pearl_vec[i];
            let type2 = set2.type_link.type_vec[i];
            let imposter2 = &set2.pearl_vec[i];
            assert!(imposter1.type_id() == type1.type_id());
            assert!(imposter2.type_id() == type2.type_id());
            assert!(type1 == type2);
            assert!(imposter1.type_id() == imposter2.type_id())
        }
    }

    #[test]
    fn type_subsets() {
        let mut types1 = PearlTypes::new();
        types1.insert::<Type1>().unwrap();
        types1.insert::<Type2>().unwrap();
        types1.insert::<Type3>().unwrap();

        let mut types2 = PearlTypes::new();
        types2.insert::<Type2>().unwrap();
        types2.insert::<Type1>().unwrap();

        let mut types3 = PearlTypes::new();
        types3.insert::<Type2>().unwrap();
        types3.insert::<Type1>().unwrap();
        types3.insert::<Type4>().unwrap();

        assert!(types1.contains_types(&types2));
        assert!(!types2.contains_types(&types1));
        assert!(!types1.contains_types(&types3));
        assert!(types3.contains_types(&types2));
    }
}
