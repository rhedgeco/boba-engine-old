use std::{any::TypeId, hash::Hash};

use imposters::{collections::ImposterVec, Imposter};
use indexmap::{IndexMap, IndexSet};

use crate::EntityId;

#[derive(Eq)]
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
    pub fn len(&self) -> usize {
        self.types.len()
    }

    pub fn is_empty(&self) -> bool {
        self.types.is_empty()
    }

    pub fn is_superset(&self, other: &PearlTypes) -> bool {
        other.is_subset(self)
    }

    pub fn is_subset(&self, other: &PearlTypes) -> bool {
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

    pub fn add_type<T: 'static>(&mut self) {
        self.types.insert(TypeId::of::<T>());
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
    pub fn types(&self) -> &PearlTypes {
        &self.types
    }

    pub fn combine(self, other: PearlSet) -> PearlSet {
        if self.pearls.is_empty() {
            return other;
        }

        if other.pearls.is_empty() {
            return self;
        }

        let mut new_types = IndexSet::new();
        let mut new_pearls = IndexMap::new();
        let mut other_iter = other.pearls.into_iter().peekable();
        for (self_id, self_imp) in self.pearls.into_iter() {
            while let Some((peek_id, _)) = other_iter.peek() {
                if peek_id > &self_id {
                    break;
                }

                let (other_id, other_imp) = other_iter.next().unwrap();
                new_types.insert(other_id);
                new_pearls.insert(other_id, other_imp);
            }

            new_types.insert(self_id);
            new_pearls.insert(self_id, self_imp);
        }

        PearlSet {
            types: PearlTypes { types: new_types },
            pearls: new_pearls,
        }
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
    pub fn add<T: 'static>(&mut self, pearl: T) {
        self.pearls.insert(TypeId::of::<T>(), Imposter::new(pearl));
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
    pub fn new(entity: EntityId, set: PearlSet) -> Self {
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
