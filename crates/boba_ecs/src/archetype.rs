use std::any::TypeId;

use imposters::collections::ImposterVec;
use indexmap::{IndexMap, IndexSet};

use crate::{EntityId, PearlSet, PearlTypes};

/// A structure containing the data for all [`EntityId`] objects that share the same pearl structure
#[derive(Default)]
pub struct Archetype {
    types: PearlTypes,
    entity_set: IndexSet<EntityId>,
    pearl_vecs: IndexMap<TypeId, ImposterVec>,
}

impl Archetype {
    /// Creates a new archetype with a specified `pearl_set`
    pub fn new(entity: EntityId, pearl_set: PearlSet) -> Self {
        let types = pearl_set.types().clone();
        let mut entity_link = IndexSet::new();
        let mut pearl_vecs = IndexMap::new();

        entity_link.insert(entity);
        for imposter in pearl_set.into_iter() {
            let typeid = imposter.type_id().clone();
            let vec = ImposterVec::from_imposter(imposter);
            pearl_vecs.insert(typeid, vec);
        }

        Self {
            types,
            entity_set: entity_link,
            pearl_vecs,
        }
    }

    /// Returns the number of entities in this archetype
    pub fn len(&self) -> usize {
        self.entity_set.len()
    }

    /// Returns `true` if there are no entities in this archetype
    pub fn is_empty(&self) -> bool {
        self.entity_set.is_empty()
    }

    /// Returns the [`PearlTypes`] for this archetype
    pub fn types(&self) -> &PearlTypes {
        &self.types
    }

    /// Inserts `entity` into this archetype with the provided `pearl_set`
    ///
    /// Returns the old set as `Some(PearlSet)` if the entity already was already present in this archetype
    ///
    /// # Panics
    /// Panics if `pearls` does not match this archetype
    pub fn insert(&mut self, entity: EntityId, pearls: PearlSet) -> Option<PearlSet> {
        if pearls.types() != &self.types {
            panic!("PearlTypes mismatch");
        }

        match self.entity_set.insert_full(entity) {
            (_, true) => {
                for (i, imposter) in pearls.into_iter().enumerate() {
                    let returned = self.pearl_vecs[i].push_imposter(imposter);
                    assert!(returned.is_none())
                }

                None
            }
            (index, false) => {
                let mut pearl_set = PearlSet::new();
                for (i, imposter) in pearls.into_iter().enumerate() {
                    let returned = self.pearl_vecs[i].push_imposter(imposter);
                    assert!(returned.is_none());
                    let old_imposter = self.pearl_vecs[i].swap_remove(index).unwrap();
                    pearl_set.insert_imposter(old_imposter);
                }

                Some(pearl_set)
            }
        }
    }

    /// Removes an [`EntityId`] from this archeytype, returning its pearls as `Some(PearlSet)`
    ///
    /// Returns `None` if the entity does not exist for this archetype
    pub fn remove(&mut self, entity: &EntityId) -> Option<PearlSet> {
        let Some((entity_index, _)) = self.entity_set.swap_remove_full(entity) else { return None };

        let mut pearl_set = PearlSet::new();
        for vec in self.pearl_vecs.values_mut() {
            let imposter = vec.swap_remove(entity_index).unwrap();
            let returned = pearl_set.insert_imposter(imposter);
            assert!(returned.is_none())
        }

        Some(pearl_set)
    }

    /// Destorys an [`EntityId`] from this archeytype and drops all the associated pearls, returning `true`
    ///
    /// Returns `false` if the entity does not exist for this archetype
    pub fn destroy(&mut self, entity: &EntityId) -> bool {
        let Some((entity_index, _)) = self.entity_set.swap_remove_full(entity) else { return false };

        for vec in self.pearl_vecs.values_mut() {
            assert!(vec.swap_drop(entity_index));
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use crate::{Pearl, World};

    use super::*;

    struct Type1(u16);
    struct Type2(u32);
    struct Type3(u64);

    impl Pearl for Type1 {}
    impl Pearl for Type2 {}
    impl Pearl for Type3 {}

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
        assert!(archetype.entity_set.len() == 1);
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
        assert!(archetype.entity_set.len() == 2);
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
        assert!(archetype.entity_set.len() == 1);
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

        assert!(archetype.entity_set.len() == 1);
        assert!(archetype.pearl_vecs.len() == 3);
        assert!(archetype.pearl_vecs[0].len() == 1);
        assert!(archetype.pearl_vecs[1].len() == 1);
        assert!(archetype.pearl_vecs[2].len() == 1);
        assert!(set2.get::<Type1>().unwrap().0 == 4);
        assert!(set2.get::<Type2>().unwrap().0 == 5);
        assert!(set2.get::<Type3>().unwrap().0 == 6);
    }
}
