use std::collections::VecDeque;

use indexmap::{map::Entry, IndexMap};

use crate::{Archetype, Pearl, PearlId, PearlSet, PearlTypes, QueryItem};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct EntityId(u64);

impl EntityId {
    const ID_BITS: u64 = 16;
    const INDEX_BITS: u64 = 64 - Self::ID_BITS;
    const INDEX_MASK: u64 = !(!0 << Self::ID_BITS);
    const MAX_INDEX: usize = Self::INDEX_MASK as usize;
    const ID_INCREMENT: u64 = 1 << Self::INDEX_BITS;

    #[inline]
    fn new(index: usize) -> Self {
        assert!(index < Self::MAX_INDEX);
        Self(index as u64)
    }

    #[inline]
    fn increment_id(&mut self) {
        self.0 += Self::ID_INCREMENT;
    }

    #[inline]
    pub fn index(&self) -> u64 {
        self.0 & Self::INDEX_MASK
    }

    #[inline]
    pub fn uindex(&self) -> usize {
        self.index() as usize
    }

    #[inline]
    pub fn is_alive(&self, world: &World) -> bool {
        match world.entities.get(self.uindex()) {
            Some(entity) if self == entity => true,
            _ => false,
        }
    }
}

pub struct World {
    entities: Vec<EntityId>,
    dead: VecDeque<usize>,
    archetypes: IndexMap<PearlTypes, Archetype>,
    entity_arch: IndexMap<EntityId, usize>,
}

impl World {
    /// Returns a new empty world
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            dead: VecDeque::new(),
            archetypes: IndexMap::new(),
            entity_arch: IndexMap::new(),
        }
    }

    /// Creates a new [`EntityId`] in this world
    pub fn new_entity(&mut self) -> EntityId {
        match self.dead.pop_front() {
            // if there is a dead entity in the queue, then we reuse it
            Some(index) => self.entities.get(index).unwrap().clone(),
            // if there is no dead entities in the queue, create a new one
            None => {
                let entity = EntityId::new(self.entities.len());
                self.entities.push(entity.clone());
                entity
            }
        }
    }

    /// Modifies the pearl set of `entity` using the provided `f` if the entity exists
    pub fn modify_pearls(&mut self, entity: &EntityId, f: impl FnOnce(PearlSet) -> PearlSet) {
        // if entity doesnt exist, early return
        if !entity.is_alive(self) {
            return;
        }

        let new_pearl_set = match self.entity_arch.get(entity) {
            None => f(PearlSet::new()),
            Some(arch_index) => {
                let (_, archetype) = self.archetypes.get_index_mut(*arch_index).unwrap();
                let old_pearl_set = archetype.remove(entity).unwrap();
                f(old_pearl_set)
            }
        };

        if new_pearl_set.is_empty() {
            return;
        }

        match self.archetypes.entry(new_pearl_set.types().clone()) {
            Entry::Occupied(e) => {
                assert!(e.into_mut().insert(*entity, new_pearl_set).is_none());
            }
            Entry::Vacant(e) => {
                e.insert(Archetype::new(*entity, new_pearl_set));
            }
        }
    }

    /// Adds a pearl of type `T` to `entity` if it exists
    pub fn add_pearl<T: Pearl>(&mut self, entity: &EntityId, pearl: T) {
        self.modify_pearls(entity, |mut set| {
            set.insert(pearl);
            set
        })
    }

    /// Adds `pearl_set` to `entity` if it exists
    pub fn add_pearl_set(&mut self, entity: &EntityId, mut pearl_set: PearlSet) {
        self.modify_pearls(entity, |set| {
            pearl_set.insert_set(set);
            pearl_set
        })
    }

    /// Destroys a pearl of type `T` from `entity` if it exists
    pub fn destroy_pearl<T: Pearl>(&mut self, entity: &EntityId) {
        self.modify_pearls(entity, |mut set| {
            set.drop_type(&PearlId::of::<T>());
            set
        })
    }

    /// Destroys the given `entity` if it exists in this world
    pub fn destroy(&mut self, entity: &EntityId) {
        let index = entity.uindex();
        // check if the entity is in the list and matches id
        let Some(self_entity) = self.entities.get_mut(index) else { return };
        if self_entity != entity {
            return;
        }

        // increment the id to stop it from matching in the future
        self_entity.increment_id();

        // add its index to the dropped queue
        self.dead.push_back(index);

        // if the entity has an archetype, destory the pearls in it too
        if let Some(arch_index) = self.entity_arch.remove(entity) {
            let (_, arch) = self.archetypes.get_index_mut(arch_index).unwrap();
            assert!(arch.destroy(entity));
        }
    }

    /// Executes a raw query on this world, modifying the provided `query` items
    ///
    /// Only archetypes that contain all the pearls in the requested query will be returned.
    /// This should generally be abstracted over to perform more idiomatic query operations.
    ///
    /// # Panics
    /// This function will panic if the `query` contains duplicate [`QueryItem`] requests
    pub fn raw_query<'a, const SIZE: usize>(&'a self, query: &'a mut [QueryItem<'a>; SIZE]) {
        if SIZE == 0 {
            return;
        }

        let mut pearl_types = PearlTypes::new_single_id(query[0].id());
        for i in 1..SIZE {
            pearl_types
                .insert_id(query[i].id())
                .expect("Duplicate pearl types in raw query");
        }

        for arch in self.archetypes.values() {
            if !arch.types().contains_set(&pearl_types) {
                continue;
            }

            for i in 0..SIZE {
                let query_item = &mut query[i];
                let vec = arch.get_pearls(&query_item.id()).unwrap();
                query_item.add_vec(vec);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_entity() {
        let mut world = World::new();
        let entity0 = world.new_entity();
        let entity1 = world.new_entity();
        assert!(world.entities.len() == 2);
        assert!(entity0.index() == 0);
        assert!(entity1.index() == 1);
    }

    #[test]
    fn destroy_entity() {
        let mut world = World::new();
        let entity0 = world.new_entity();
        world.destroy(&entity0);
        assert!(world.entities.len() == 1);
        assert!(world.dead.len() == 1);
        let entity1 = world.new_entity();
        assert!(world.entities.len() == 1);
        assert!(world.dead.len() == 0);
        assert!(!entity0.is_alive(&world));
        assert!(entity1.is_alive(&world));
    }
}
