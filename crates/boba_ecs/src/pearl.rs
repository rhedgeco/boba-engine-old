use std::{any::TypeId, hash::Hash, mem::replace};

use imposters::{collections::ImposterVec, Imposter};
use indexmap::{IndexMap, IndexSet};

use crate::EntityId;

#[derive(Eq, Clone)]
pub struct PearlTypes {
    types: IndexSet<TypeId>,
}

impl PartialEq for PearlTypes {
    fn eq(&self, other: &Self) -> bool {
        if self.types.len() != other.types.len() {
            return false;
        }
        // we can do this in O(n) because the types set is always sorted
        for i in 0..self.types.len() {
            let self_type = self.types.get_index(i).unwrap();
            let other_type = other.types.get_index(i).unwrap();
            if self_type != other_type {
                return false;
            }
        }

        true
    }
}

impl Hash for PearlTypes {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // This is consistent because the type set is always sorted
        for t in self.types.iter() {
            t.hash(state);
        }
    }
}

impl PearlTypes {
    pub fn empty() -> Self {
        Self {
            types: IndexSet::new(),
        }
    }

    pub fn new_single<T: 'static>() -> Self {
        let mut types = IndexSet::new();
        types.insert(TypeId::of::<T>());
        Self { types }
    }

    pub fn len(&self) -> usize {
        self.types.len()
    }

    pub fn is_empty(&self) -> bool {
        self.types.is_empty()
    }

    pub fn contains<T: 'static>(&self) -> bool {
        let typeid = TypeId::of::<T>();

        for t in self.types.iter() {
            if t == &typeid {
                return true;
            }
        }

        false
    }

    pub fn remove(&mut self, type_id: &TypeId) -> bool {
        self.types.shift_remove(type_id)
    }

    pub fn is_subset_of(&self, other: &PearlTypes) -> bool {
        other.is_superset_of(self)
    }

    pub fn is_superset_of(&self, other: &PearlTypes) -> bool {
        // if this set has less elements that the other set
        // it could not possibly be a subset
        if self.types.len() < other.types.len() {
            return false;
        }

        // get an iterator for the other pearl types
        let mut other_iter = other.types.iter();
        // if it doesnt have an initial `next`, it is empty. return false.
        let Some(mut find_type) = other_iter.next() else {
            return false;
        };

        // iterate over all of self types and match them
        for t in self.types.iter() {
            // if they dont match, continue to the next item in self types
            if t != find_type {
                continue;
            }

            // if they do match, set find_type to the next item to search for
            // if it is none, that means we found all the items and return true
            let Some(next) = other_iter.next() else {
                return true;
            };

            // put the next value back into find_type for the next iteration
            find_type = next;
        }

        // not all items were found, so it is not a subset
        false
    }
}

#[derive(Default)]
pub struct PearlTypesBuilder {
    types: IndexSet<TypeId>,
}

impl PearlTypesBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_type<T: 'static>(mut self) -> Self {
        self.types.insert(TypeId::of::<T>());
        self
    }

    pub fn build(mut self) -> PearlTypes {
        self.types.sort();
        PearlTypes { types: self.types }
    }
}

pub struct PearlSet {
    types: PearlTypes,
    pearls: IndexMap<TypeId, Imposter>,
}

impl PearlSet {
    pub fn empty() -> Self {
        Self {
            types: PearlTypes::empty(),
            pearls: IndexMap::new(),
        }
    }

    pub fn new_single<T: 'static>(pearl: T) -> Self {
        let types = PearlTypes::new_single::<T>();
        let mut pearls = IndexMap::new();
        pearls.insert(TypeId::of::<T>(), Imposter::new(pearl));
        Self { types, pearls }
    }

    pub fn len(&self) -> usize {
        self.pearls.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pearls.is_empty()
    }

    pub fn types(&self) -> &PearlTypes {
        &self.types
    }

    pub fn drop(&mut self, type_id: &TypeId) -> bool {
        self.types.remove(type_id);
        self.pearls.remove(type_id).is_some()
    }

    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        let type_id = &TypeId::of::<T>();
        self.types.remove(type_id);
        let imposter = self.pearls.shift_remove(type_id)?;
        imposter.downcast::<T>()
    }

    /// Combined two pearl sets together.
    ///
    /// Any overlapping items will be taken from the original set
    pub fn combine(&mut self, other: PearlSet) {
        if other.pearls.is_empty() {
            return;
        }

        if self.pearls.is_empty() {
            self.types = other.types;
            self.pearls = other.pearls;
            return;
        }

        let mut new_types = IndexSet::new();
        let mut other_iter = other.pearls.into_iter().peekable();
        let old_pearls = replace(&mut self.pearls, IndexMap::new());
        for (self_id, self_imp) in old_pearls.into_iter() {
            while let Some((peek_id, _)) = other_iter.peek() {
                if peek_id >= &self_id {
                    break;
                }

                let (other_id, other_imp) = other_iter.next().unwrap();
                new_types.insert(other_id);
                self.pearls.insert(other_id, other_imp);
            }

            new_types.insert(self_id);
            self.pearls.insert(self_id, self_imp);
        }

        self.types = PearlTypes { types: new_types };
    }
}

#[derive(Default)]
pub struct PearlSetBuilder {
    pearls: IndexMap<TypeId, Imposter>,
}

impl PearlSetBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn add<T: 'static>(mut self, pearl: T) -> Self {
        self.pearls.insert(TypeId::of::<T>(), Imposter::new(pearl));
        self
    }

    pub fn build(mut self) -> PearlSet {
        self.pearls.sort_keys();

        let mut types = IndexSet::new();
        for typeid in self.pearls.keys() {
            types.insert(*typeid);
        }

        PearlSet {
            types: PearlTypes { types },
            pearls: self.pearls,
        }
    }
}

pub struct Archetype {
    types: PearlTypes,
    entities: IndexSet<EntityId>,
    pearls: IndexMap<TypeId, ImposterVec>,
}

impl Archetype {
    pub fn with_pearl<T: 'static>(entity: EntityId, pearl: T) -> Self {
        let mut entities = IndexSet::new();
        entities.insert(entity);

        let types = PearlTypes::new_single::<T>();
        let mut pearls = IndexMap::new();
        pearls.insert(
            TypeId::of::<T>(),
            ImposterVec::from_imposter(Imposter::new(pearl)),
        );

        Self {
            entities,
            types,
            pearls,
        }
    }

    pub fn with_pearl_set(entity: EntityId, set: PearlSet) -> Self {
        let mut entities = IndexSet::new();
        entities.insert(entity);

        let types = set.types;
        let mut pearls = IndexMap::new();
        for (typeid, imposter) in set.pearls.into_iter() {
            let vec = ImposterVec::from_imposter(imposter);
            pearls.insert(typeid, vec);
        }

        Self {
            entities,
            types,
            pearls,
        }
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    pub fn insert(&mut self, entity: EntityId, set: PearlSet) -> Option<(EntityId, PearlSet)> {
        if set.types() != &self.types || !self.entities.insert(entity) {
            return Some((entity, set));
        }

        for (index, (_, imposter)) in set.pearls.into_iter().enumerate() {
            self.pearls[index].push_imposter(imposter);
        }

        None
    }

    pub fn remove(&mut self, entity: &EntityId) -> Option<PearlSet> {
        let Some((index, _)) = self.entities.swap_remove_full(entity) else {
            return None;
        };

        let mut new_types = IndexSet::new();
        let mut new_pearls = IndexMap::new();
        for vec in self.pearls.values_mut() {
            let imposter = unsafe { vec.swap_remove_unchecked(index) };
            new_types.insert(imposter.type_id());
            new_pearls.insert(imposter.type_id(), imposter);
        }

        Some(PearlSet {
            types: PearlTypes { types: new_types },
            pearls: new_pearls,
        })
    }

    pub fn destroy(&mut self, entity: &EntityId) -> bool {
        let Some((index, _)) = self.entities.swap_remove_full(entity) else {
            return false;
        };

        for vec in self.pearls.values_mut() {
            vec.swap_drop(index);
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
        let types1 = PearlTypesBuilder::new()
            .add_type::<Type1>()
            .add_type::<Type2>()
            .add_type::<Type3>()
            .build();

        let types2 = PearlTypesBuilder::new()
            .add_type::<Type2>()
            .add_type::<Type3>()
            .add_type::<Type1>()
            .build();

        for i in 0..3 {
            let type1 = types1.types.get_index(i).unwrap();
            let type2 = types2.types.get_index(i).unwrap();
            assert!(type1 == type2);
        }
    }

    #[test]
    fn pearl_set_sorted() {
        let set1 = PearlSetBuilder::new()
            .add(Type1(1))
            .add(Type2(2))
            .add(Type3(3))
            .build();

        let set2 = PearlSetBuilder::new()
            .add(Type2(1))
            .add(Type3(2))
            .add(Type1(3))
            .build();

        assert!(set2.types() == set1.types());

        for i in 0..3 {
            let type1 = set1.types.types.get_index(i).unwrap();
            let (pearltype1, data1) = set1.pearls.get_index(i).unwrap();
            let type2 = set2.types.types.get_index(i).unwrap();
            let (pearltype2, data2) = set2.pearls.get_index(i).unwrap();
            assert!(&data1.type_id() == type1);
            assert!(&data2.type_id() == type2);
            assert!(type1 == type2);
            assert!(pearltype1 == pearltype2);
        }
    }

    #[test]
    fn types_subset() {
        let types1 = PearlTypesBuilder::new()
            .add_type::<Type1>()
            .add_type::<Type2>()
            .add_type::<Type3>()
            .build();

        let types2 = PearlTypesBuilder::new()
            .add_type::<Type2>()
            .add_type::<Type1>()
            .build();

        let types3 = PearlTypesBuilder::new()
            .add_type::<Type2>()
            .add_type::<Type1>()
            .add_type::<Type4>()
            .build();

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

        let set = PearlSetBuilder::new()
            .add(Type1(1))
            .add(Type2(2))
            .add(Type3(3))
            .build();

        let types = set.types().clone();
        let archetype = Archetype::with_pearl_set(entity, set);
        assert!(archetype.entities.len() == 1);
        assert!(archetype.pearls.len() == 3);
        assert!(types == archetype.types);
    }

    #[test]
    fn insert_archetype() {
        let mut entities = World::new();
        let entity1 = entities.new_entity();
        let entity2 = entities.new_entity();
        let set1 = PearlSetBuilder::new()
            .add(Type1(1))
            .add(Type2(2))
            .add(Type3(3))
            .build();
        let set2 = PearlSetBuilder::new()
            .add(Type1(1))
            .add(Type2(2))
            .add(Type3(3))
            .build();

        let mut archetype = Archetype::with_pearl_set(entity1, set1);
        archetype.insert(entity2, set2);
        assert!(archetype.entities.len() == 2);
        assert!(archetype.pearls.len() == 3);
        assert!(archetype.pearls.get_index(0).unwrap().1.len() == 2);
        assert!(archetype.pearls.get_index(1).unwrap().1.len() == 2);
        assert!(archetype.pearls.get_index(2).unwrap().1.len() == 2);
    }

    #[test]
    fn delete_from_archetype() {
        let mut entities = World::new();
        let entity1 = entities.new_entity();
        let entity2 = entities.new_entity();
        let set1 = PearlSetBuilder::new()
            .add(Type1(1))
            .add(Type2(2))
            .add(Type3(3))
            .build();
        let set2 = PearlSetBuilder::new()
            .add(Type1(4))
            .add(Type2(5))
            .add(Type3(6))
            .build();

        let mut archetype = Archetype::with_pearl_set(entity1, set1);
        archetype.insert(entity2, set2);
        archetype.destroy(&entity1);
        assert!(archetype.entities.len() == 1);
        assert!(archetype.pearls.len() == 3);
        assert!(archetype.pearls.get_index(0).unwrap().1.len() == 1);
        assert!(archetype.pearls.get_index(1).unwrap().1.len() == 1);
        assert!(archetype.pearls.get_index(2).unwrap().1.len() == 1);

        let type1_vec = archetype.pearls.get(&TypeId::of::<Type1>()).unwrap();
        assert!(type1_vec.get::<Type1>(0).unwrap().0 == 4);

        let type2_vec = archetype.pearls.get(&TypeId::of::<Type2>()).unwrap();
        assert!(type2_vec.get::<Type2>(0).unwrap().0 == 5);

        let type3_vec = archetype.pearls.get(&TypeId::of::<Type3>()).unwrap();
        assert!(type3_vec.get::<Type3>(0).unwrap().0 == 6);
    }

    #[test]
    fn remove_from_archetype() {
        let mut entities = World::new();
        let entity1 = entities.new_entity();
        let entity2 = entities.new_entity();
        let set1 = PearlSetBuilder::new()
            .add(Type1(1))
            .add(Type2(2))
            .add(Type3(3))
            .build();
        let set2 = PearlSetBuilder::new()
            .add(Type1(4))
            .add(Type2(5))
            .add(Type3(6))
            .build();

        let set2_types = set2.types().clone();
        let mut archetype = Archetype::with_pearl_set(entity1, set1);
        archetype.insert(entity2, set2);

        let set2 = archetype.remove(&entity2).unwrap();
        assert!(set2.types() == &set2_types);

        assert!(archetype.entities.len() == 1);
        assert!(archetype.pearls.len() == 3);
        assert!(archetype.pearls.get_index(0).unwrap().1.len() == 1);
        assert!(archetype.pearls.get_index(1).unwrap().1.len() == 1);
        assert!(archetype.pearls.get_index(2).unwrap().1.len() == 1);

        let type1_imp = set2.pearls.get(&TypeId::of::<Type1>()).unwrap();
        assert!(type1_imp.downcast_ref::<Type1>().unwrap().0 == 4);

        let type2_imp = set2.pearls.get(&TypeId::of::<Type2>()).unwrap();
        assert!(type2_imp.downcast_ref::<Type2>().unwrap().0 == 5);

        let type3_imp = set2.pearls.get(&TypeId::of::<Type3>()).unwrap();
        assert!(type3_imp.downcast_ref::<Type3>().unwrap().0 == 6);
    }
}
