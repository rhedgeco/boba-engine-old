use indexmap::{map::Entry, IndexMap};

use crate::{Archetype, Entity, EntityManager, PearlIdSet, PearlSet};

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

#[derive(Default)]
pub struct World {
    entities: EntityManager<PearlLink>,
    archetypes: IndexMap<PearlIdSet, Archetype>,
}

impl World {
    /// Creates a new world
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates and returns a new unique [`Entity`] for this world.
    #[inline]
    pub fn create_entity(&mut self) -> Entity {
        self.entities.create(Default::default())
    }

    /// Returns `true` if `entity` is valid and alive in this world
    #[inline]
    pub fn has_entity(&self, entity: &Entity) -> bool {
        self.entities.contains(entity)
    }

    /// Creates and returns a new unique [`Entity`] in this world with an associated [`PearlSet`]
    pub fn create_entity_with_pearls(&mut self, set: PearlSet) -> Entity {
        let entity = self.entities.create(Default::default());
        let link = match self.archetypes.entry(set.id_set().clone()) {
            Entry::Occupied(e) => {
                let archetype_index = e.index();
                let archetype = e.into_mut();
                let pearl_index = archetype.insert(entity, set);
                PearlLink::new(archetype_index, pearl_index)
            }
            Entry::Vacant(e) => {
                let archetype = Archetype::new(entity, set);
                let archetype_index = e.index();
                e.insert(archetype);
                PearlLink::new(archetype_index, 0)
            }
        };

        self.entities.replace_data(&entity, link);
        entity
    }

    /// Destroys the `entity` in this world, returning `true`.
    ///
    /// Returns `false` if `entity` was already invalid for this world.
    pub fn destroy_entity(&mut self, entity: &Entity) -> bool {
        match self.entities.destroy(entity) {
            None => false,
            Some(link) if link.active => {
                let archetype = &mut self.archetypes[link.archetype];
                let Some(swapped_entity) = archetype.swap_destroy(link.pearl) else { return true };
                let swapped_data = self.entities.get_data_mut(&swapped_entity).unwrap();
                swapped_data.pearl = link.pearl;
                true
            }
            _ => true,
        }
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
        assert!(world.archetypes[pearl_link.archetype].len() == 1);
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
        assert!(archetype.len() == 0);
    }
}
