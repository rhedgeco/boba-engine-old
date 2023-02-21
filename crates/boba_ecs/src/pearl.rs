use imposters::{collections::ImposterVec, Imposter};
use indexmap::{IndexMap, IndexSet};
use std::{any::TypeId, hash::Hash, mem::replace};

use crate::EntityId;

/// A collection of pearl types
///
/// Used for identification of what kinds of pearls are stored in an [`Archetype`]
#[derive(Eq, Clone, Default)]
pub struct PearlTypes {
    type_vec: Vec<TypeId>,
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
    pub fn new_single<T: 'static>() -> Self {
        Self::new_single_id(TypeId::of::<T>())
    }

    /// Creates a new `PearlTypes` struct initialized with `id`
    #[inline]
    pub fn new_single_id(id: TypeId) -> Self {
        let mut new = Self::new();
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

    /// Returns the index of a specified `id`
    ///
    /// If the id exists, returns `Ok(usize)` with the index.
    /// If it does not exist, returns Err(usize) with the index where it would be inserted.
    #[inline]
    fn index_of_id(&self, id: &TypeId) -> Result<usize, usize> {
        self.type_vec.binary_search(id)
    }

    /// Returns `true` if the set contains the type `T`
    #[inline]
    pub fn contains<T: 'static>(&self) -> bool {
        self.contains_id(&TypeId::of::<T>())
    }

    /// Returns `true` if the set contains `id`
    #[inline]
    pub fn contains_id(&self, id: &TypeId) -> bool {
        self.index_of_id(id).is_ok()
    }

    /// Removes type `T` from the set and returns `true`
    ///
    /// If the item is not in the set, returns `false`
    #[inline]
    pub fn remove<T: 'static>(&mut self) -> bool {
        self.remove_id(&TypeId::of::<T>())
    }

    /// Removes `id` from the set and returns `true`
    ///
    /// If the item is not in the set, returns `false`
    #[inline]
    pub fn remove_id(&mut self, id: &TypeId) -> bool {
        self.remove_id_get_index(id).is_some()
    }

    /// Removes `id` from the set and returns `Ok(usize)` with the index that it was removed from
    ///
    /// If the item is not in the set, returns `None`
    #[inline]
    fn remove_id_get_index(&mut self, id: &TypeId) -> Option<usize> {
        let Ok(index) = self.index_of_id(id) else { return None };
        self.type_vec.remove(index);
        Some(index)
    }

    /// Inserts type `T` into this set and returns `true`
    ///
    /// If the type already existed, returns `false`
    #[inline]
    pub fn insert<T: 'static>(&mut self) -> bool {
        self.insert_id(TypeId::of::<T>())
    }

    /// Inserts `id` into this set and returns `true`
    ///
    /// If the id already existed, returns `false`
    #[inline]
    pub fn insert_id(&mut self, id: TypeId) -> bool {
        if let Err(index) = self.index_of_id(&id) {
            self.type_vec.insert(index, id);
            return true;
        }

        false
    }

    /// Returns true if this set is a superset of `other`
    #[inline]
    pub fn is_superset_of(&self, other: &PearlTypes) -> bool {
        other.is_subset_of(self)
    }

    /// Returns true if this set is a subset of `other`
    pub fn is_subset_of(&self, other: &PearlTypes) -> bool {
        // if they both have no items, then other is technically a superset
        if self.len() == 0 && other.len() == 0 {
            return true;
        }

        // if this vec has more elements that the other vec
        // then other could not possibly be superset
        if self.len() > other.len() {
            return false;
        }

        // get an iterator for self pearl types
        let mut self_iter = self.type_vec.iter();
        // get the first item to search for
        let mut find_type = self_iter.next().unwrap();

        // iterate over all of self types and match them
        for t in other.type_vec.iter() {
            // if they dont match, continue to the next item in self types
            if t != find_type {
                continue;
            }

            // if they do match, get the next item we need to search for
            // if it is none, that means we found all the items and return true
            let Some(next) = self_iter.next() else {
                return true;
            };

            // put the next value back into find_type for the next iteration
            find_type = next;
        }

        // not all items were found, so this is not a subset
        false
    }
}

/// A collection of pearls
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
    pub fn new_single<T: 'static>(item: T) -> Self {
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

    /// Removes the pearl of type `T` from this set
    ///
    /// Returns `None` if the pearl does not exist
    #[inline]
    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        self.remove_imposter(&TypeId::of::<T>())?.downcast::<T>()
    }

    /// Removes and drops the pearl with `id` and returns `true`
    ///
    /// Returns `false` if the pearl does not exist
    #[inline]
    pub fn drop_type(&mut self, id: &TypeId) -> bool {
        // dropped implicitly
        self.remove_imposter(id).is_some()
    }

    /// Removes the pearl with `id` and returns an untyped [`Imposter`] that contains the pearl data
    ///
    /// Returns `None` if the pearl does not exist
    #[inline]
    pub fn remove_imposter(&mut self, id: &TypeId) -> Option<Imposter> {
        let index = self.type_link.remove_id_get_index(id)?;
        Some(self.pearl_vec.remove(index))
    }

    /// Inserts `item` into the set, replacing it if necessary
    ///
    /// If there was an existing pearl of type `T`,
    /// then it will be replaced and this method will return it as `Some(T)`
    #[inline]
    pub fn insert<T: 'static>(&mut self, item: T) -> Option<T> {
        self.insert_imposter(Imposter::new(item))?.downcast::<T>()
    }

    /// Inserts `imposter` into the set, replacing it if necessary
    ///
    /// If there was an existing [`Imposter`] with the same [`TypeId`],
    /// then it will be replaced and this method will return it as `Some(Imposter)`
    pub fn insert_imposter(&mut self, imposter: Imposter) -> Option<Imposter> {
        let typeid = imposter.type_id();
        match self.type_link.index_of_id(&typeid) {
            Ok(index) => {
                let old = replace(&mut self.pearl_vec[index], imposter);
                return Some(old);
            }
            Err(index) => {
                self.type_link.type_vec.insert(index, typeid);
                self.pearl_vec.insert(index, imposter);
                return None;
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
        // each insert does a new binary search on the array
        // since they are already in order, they can be interleaved in a faster way
        for imposter in other.pearl_vec.into_iter() {
            self.insert_imposter(imposter);
        }
    }
}

#[derive(Default)]
pub struct Archetype {
    types: PearlTypes,
    entity_link: IndexSet<EntityId>,
    pearl_vecs: IndexMap<TypeId, ImposterVec>,
}

impl Archetype {
    pub fn new(entity: EntityId, pearl_set: PearlSet) -> Self {
        let types = pearl_set.types().clone();
        let mut entity_link = IndexSet::new();
        let mut pearl_vecs = IndexMap::new();

        entity_link.insert(entity);
        for imposter in pearl_set.pearl_vec.into_iter() {
            let typeid = imposter.type_id().clone();
            let vec = ImposterVec::from_imposter(imposter);
            pearl_vecs.insert(typeid, vec);
        }

        Self {
            types,
            entity_link,
            pearl_vecs,
        }
    }

    pub fn len(&self) -> usize {
        self.entity_link.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entity_link.is_empty()
    }

    pub fn types(&self) -> &PearlTypes {
        &self.types
    }

    pub fn insert(&mut self, entity: EntityId, pearls: PearlSet) -> Option<PearlSet> {
        if pearls.types() != &self.types {
            panic!("PearlTypes mismatch");
        }

        match self.entity_link.insert_full(entity) {
            (_, true) => {
                for (i, imposter) in pearls.pearl_vec.into_iter().enumerate() {
                    let returned = self.pearl_vecs[i].push_imposter(imposter);
                    assert!(returned.is_none())
                }

                None
            }
            (index, false) => {
                let mut pearl_set = PearlSet::new();
                for (i, imposter) in pearls.pearl_vec.into_iter().enumerate() {
                    let returned = self.pearl_vecs[i].push_imposter(imposter);
                    assert!(returned.is_none());
                    let old_imposter = self.pearl_vecs[i].swap_remove(index).unwrap();
                    pearl_set.insert_imposter(old_imposter);
                }

                Some(pearl_set)
            }
        }
    }

    pub fn remove(&mut self, entity: &EntityId) -> Option<PearlSet> {
        let Some((entity_index, _)) = self.entity_link.swap_remove_full(entity) else { return None };

        let mut pearl_set = PearlSet::new();
        for vec in self.pearl_vecs.values_mut() {
            let imposter = vec.swap_remove(entity_index).unwrap();
            let returned = pearl_set.insert_imposter(imposter);
            assert!(returned.is_none())
        }

        Some(pearl_set)
    }

    pub fn destroy(&mut self, entity: &EntityId) -> bool {
        let Some((entity_index, _)) = self.entity_link.swap_remove_full(entity) else { return false };

        for vec in self.pearl_vecs.values_mut() {
            assert!(vec.swap_drop(entity_index));
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use crate::World;

    use super::*;

    struct Type1(u16);
    struct Type2(u32);
    struct Type3(u64);
    struct Type4(u128);

    #[test]
    fn pearl_types_sorted() {
        let mut types1 = PearlTypes::new();
        types1.insert::<Type1>();
        types1.insert::<Type2>();
        types1.insert::<Type3>();

        let mut types2 = PearlTypes::new();
        types2.insert::<Type1>();
        types2.insert::<Type2>();
        types2.insert::<Type3>();

        for i in 0..3 {
            let type1 = types1.type_vec[i];
            let type2 = types2.type_vec[i];
            assert!(type1 == type2);
        }
    }

    #[test]
    fn pearl_set_sorted() {
        let mut set1 = PearlSet::new();
        set1.insert(Type1(1));
        set1.insert(Type2(2));
        set1.insert(Type3(3));

        let mut set2 = PearlSet::new();
        set2.insert(Type1(1));
        set2.insert(Type2(2));
        set2.insert(Type3(3));

        assert!(set2.types() == set1.types());

        for i in 0..3 {
            let type1 = set1.type_link.type_vec[i];
            let imposter1 = &set1.pearl_vec[i];
            let type2 = set2.type_link.type_vec[i];
            let imposter2 = &set2.pearl_vec[i];
            assert!(imposter1.type_id() == type1);
            assert!(imposter2.type_id() == type2);
            assert!(type1 == type2);
            assert!(imposter1.type_id() == imposter2.type_id())
        }
    }

    #[test]
    fn types_subset() {
        let mut types1 = PearlTypes::new();
        types1.insert::<Type1>();
        types1.insert::<Type2>();
        types1.insert::<Type3>();

        let mut types2 = PearlTypes::new();
        types2.insert::<Type2>();
        types2.insert::<Type1>();

        let mut types3 = PearlTypes::new();
        types3.insert::<Type2>();
        types3.insert::<Type1>();
        types3.insert::<Type4>();

        assert!(types2.is_subset_of(&types1));
        assert!(types1.is_superset_of(&types2));
        assert!(!types2.is_superset_of(&types1));
        assert!(!types3.is_subset_of(&types1));
        assert!(types2.is_subset_of(&types3));
    }

    #[test]
    fn new_archetype() {
        let mut entities = World::new();
        let entity = entities.new_entity();

        let mut set = PearlSet::new();
        set.insert(Type1(1));
        set.insert(Type2(2));
        set.insert(Type3(3));

        let types = set.types().clone();
        let archetype = Archetype::new(entity, set);
        assert!(archetype.entity_link.len() == 1);
        assert!(archetype.pearl_vecs.len() == 3);
        assert!(types == archetype.types);
    }

    #[test]
    fn insert_archetype() {
        let mut entities = World::new();
        let entity1 = entities.new_entity();
        let entity2 = entities.new_entity();
        let mut set1 = PearlSet::new();
        set1.insert(Type1(1));
        set1.insert(Type2(2));
        set1.insert(Type3(3));

        let mut set2 = PearlSet::new();
        set2.insert(Type1(1));
        set2.insert(Type2(2));
        set2.insert(Type3(3));

        let mut archetype = Archetype::new(entity1, set1);
        archetype.insert(entity2, set2);
        assert!(archetype.entity_link.len() == 2);
        assert!(archetype.pearl_vecs.len() == 3);
        assert!(archetype.pearl_vecs[0].len() == 2);
        assert!(archetype.pearl_vecs[1].len() == 2);
        assert!(archetype.pearl_vecs[2].len() == 2);
    }

    #[test]
    fn delete_from_archetype() {
        let mut entities = World::new();
        let entity1 = entities.new_entity();
        let entity2 = entities.new_entity();
        let mut set1 = PearlSet::new();
        set1.insert(Type1(1));
        set1.insert(Type2(2));
        set1.insert(Type3(3));

        let mut set2 = PearlSet::new();
        set2.insert(Type1(4));
        set2.insert(Type2(5));
        set2.insert(Type3(6));

        let mut archetype = Archetype::new(entity1, set1);
        archetype.insert(entity2, set2);
        archetype.destroy(&entity1);
        assert!(archetype.entity_link.len() == 1);
        assert!(archetype.pearl_vecs.len() == 3);
        assert!(archetype.pearl_vecs[0].len() == 1);
        assert!(archetype.pearl_vecs[1].len() == 1);
        assert!(archetype.pearl_vecs[2].len() == 1);

        let type1_vec = archetype.pearl_vecs.get(&TypeId::of::<Type1>()).unwrap();
        assert!(type1_vec.get::<Type1>(0).unwrap().0 == 4);

        let type2_vec = archetype.pearl_vecs.get(&TypeId::of::<Type2>()).unwrap();
        assert!(type2_vec.get::<Type2>(0).unwrap().0 == 5);

        let type3_vec = archetype.pearl_vecs.get(&TypeId::of::<Type3>()).unwrap();
        assert!(type3_vec.get::<Type3>(0).unwrap().0 == 6);
    }

    #[test]
    fn remove_from_archetype() {
        let mut entities = World::new();
        let entity1 = entities.new_entity();
        let entity2 = entities.new_entity();
        let mut set1 = PearlSet::new();
        set1.insert(Type1(1));
        set1.insert(Type2(2));
        set1.insert(Type3(3));

        let mut set2 = PearlSet::new();
        set2.insert(Type1(4));
        set2.insert(Type2(5));
        set2.insert(Type3(6));

        let set2_types = set2.types().clone();
        let mut archetype = Archetype::new(entity1, set1);
        archetype.insert(entity2, set2);

        let set2 = archetype.remove(&entity2).unwrap();
        assert!(set2.types() == &set2_types);

        assert!(archetype.entity_link.len() == 1);
        assert!(archetype.pearl_vecs.len() == 3);
        assert!(archetype.pearl_vecs[0].len() == 1);
        assert!(archetype.pearl_vecs[1].len() == 1);
        assert!(archetype.pearl_vecs[2].len() == 1);

        let type1_imp =
            &set2.pearl_vec[set2.type_link.index_of_id(&TypeId::of::<Type1>()).unwrap()];
        assert!(type1_imp.downcast_ref::<Type1>().unwrap().0 == 4);

        let type2_imp =
            &set2.pearl_vec[set2.type_link.index_of_id(&TypeId::of::<Type2>()).unwrap()];
        assert!(type2_imp.downcast_ref::<Type2>().unwrap().0 == 5);

        let type3_imp =
            &set2.pearl_vec[set2.type_link.index_of_id(&TypeId::of::<Type3>()).unwrap()];
        assert!(type3_imp.downcast_ref::<Type3>().unwrap().0 == 6);
    }
}
