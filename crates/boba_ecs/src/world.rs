use imposters::collections::vec::ImposterVec;
use indexmap::{map::Entry, IndexMap};

use crate::{Entity, EntityManager, PearlIdSet, PearlSet};

#[derive(Default, Clone, Copy)]
struct PearlLink {
    active: bool,
    archetype: usize,
    pearl: usize,
}

impl PearlLink {
    pub fn new(archetype: usize, pearl: usize) -> Self {
        Self {
            active: true,
            archetype,
            pearl,
        }
    }
}

struct Archetype {
    len: usize,
    entities: Vec<Entity>,
    pearls: Vec<ImposterVec>,
}

impl Archetype {
    pub fn new(entity: Entity, set: PearlSet) -> Self {
        let mut pearls = Vec::new();
        for imposter in set.into_vec().into_iter() {
            pearls.push(ImposterVec::from_imposter(imposter));
        }

        Self {
            len: 1,
            entities: vec![entity],
            pearls,
        }
    }

    /// Inserts a pearl set into the archetype and associates it with an `entity`
    ///
    /// # Panics
    /// Will panic if the pearl set does not match this archetypes pearl set, leaving the archetype in an undefined state.
    pub fn insert(&mut self, entity: Entity, set: PearlSet) -> usize {
        assert!(self.pearls.len() == set.id_set().len());
        let index = self.len;
        self.entities.push(entity);
        for (i, imposter) in set.into_vec().into_iter().enumerate() {
            self.pearls[i].push_imposter(imposter).ok().unwrap();
        }
        self.len += 1;
        index
    }

    /// Destroys the entity at a given index and swaps it with the last entity.
    ///
    /// A copy of the swapped entity is returned.
    ///
    /// # Panics
    /// Panics if `index` is out of bounds
    pub fn swap_destroy(&mut self, index: usize) -> Option<Entity> {
        assert!(index < self.len);
        for vec in self.pearls.iter_mut() {
            vec.swap_drop(index);
        }
        self.len -= 1;
        self.entities.swap_remove(index);

        if self.entities.len() == 0 {
            return None;
        }

        Some(self.entities[index])
    }
}

#[derive(Default)]
pub struct World {
    entities: EntityManager<PearlLink>,
    archetypes: IndexMap<PearlIdSet, Archetype>,
}

impl World {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn create_entity(&mut self) -> Entity {
        self.entities.create(Default::default())
    }

    #[inline]
    pub fn has_entity(&self, entity: &Entity) -> bool {
        self.entities.contains(entity)
    }

    pub fn create_entity_with_pearls(&mut self, set: PearlSet) -> Entity {
        let entity = self.entities.create(Default::default());
        let (archetype, pearl) = match self.archetypes.entry(set.id_set().clone()) {
            Entry::Occupied(e) => {
                let archetype_index = e.index();
                let archetype = e.into_mut();
                let pearl_index = archetype.insert(entity, set);
                (archetype_index, pearl_index)
            }
            Entry::Vacant(e) => {
                let archetype = Archetype::new(entity, set);
                let archetype_index = e.index();
                e.insert(archetype);
                (archetype_index, 0)
            }
        };

        self.entities
            .swap_data(&entity, PearlLink::new(archetype, pearl));
        entity
    }

    pub fn destroy_entity(&mut self, entity: &Entity) -> bool {
        let Some(link_data) = self.entities.destroy(entity) else { return false };

        if link_data.active {
            let Some(swapped_entity) =
                self.archetypes[link_data.archetype].swap_destroy(link_data.pearl) else { return true };
            let swapped_data = self.entities.get_data_mut(&swapped_entity).unwrap();
            swapped_data.pearl = link_data.pearl;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Struct0(u32);

    #[derive(Debug)]
    struct Struct1(u32);

    #[derive(Debug)]
    struct Struct2(u32);

    #[derive(Debug)]
    struct Struct3(u32);

    #[test]
    fn create_entity() {
        let mut world = World::new();
        let entity = world.create_entity();
        assert!(world.has_entity(&entity));
    }

    #[test]
    fn create_entity_with_pearls() {
        let mut pearl_set = PearlSet::new_with(Struct0(42));
        pearl_set.insert(Struct1(43)).unwrap();
        pearl_set.insert(Struct2(44)).unwrap();
        pearl_set.insert(Struct3(45)).unwrap();
        let id_set = pearl_set.id_set().clone();

        let mut world = World::new();
        let entity = world.create_entity_with_pearls(pearl_set);
        assert!(world.has_entity(&entity));

        assert!(world.archetypes.contains_key(&id_set));
        let pearl_link = world.entities.get_data(&entity).unwrap();
        assert!(pearl_link.active);
        assert!(world.archetypes[pearl_link.archetype].len == 1);
    }

    #[test]
    fn destroy_entities() {
        let mut world = World::new();
        let entity0 = world.create_entity();
        let entity1 = world.create_entity();
        let entity2 = world.create_entity();

        assert!(world.destroy_entity(&entity1));
        let entity3 = world.create_entity();

        assert!(world.has_entity(&entity0));
        assert!(!world.has_entity(&entity1));
        assert!(world.has_entity(&entity2));
        assert!(world.has_entity(&entity3));
    }

    #[test]
    fn destroy_entity_with_pearls() {
        let mut pearl_set = PearlSet::new_with(Struct0(42));
        pearl_set.insert(Struct1(43)).unwrap();
        pearl_set.insert(Struct2(44)).unwrap();
        pearl_set.insert(Struct3(45)).unwrap();
        let id_set = pearl_set.id_set().clone();

        let mut world = World::new();
        let entity = world.create_entity_with_pearls(pearl_set);
        assert!(world.has_entity(&entity));

        world.destroy_entity(&entity);
        assert!(!world.has_entity(&entity));

        let archetype = world.archetypes.get(&id_set).unwrap();
        assert!(archetype.len == 0);
    }
}
