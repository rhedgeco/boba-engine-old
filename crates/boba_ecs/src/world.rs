use crate::{
    pearls::{Pearl, PearlSet},
    ArchetypeIndexer, ArchetypeManager, Entity, EntityManager,
};

#[derive(Default, Clone, Copy)]
struct ArchLink {
    active: bool,
    indexer: ArchetypeIndexer,
}

impl ArchLink {
    pub fn new(indexer: ArchetypeIndexer) -> Self {
        Self {
            active: true,
            indexer,
        }
    }
}

/// The central storage point for [`Entity`] and [`Pearl`] structs.
/// This is the point where all ECS operations will be performed.
#[derive(Default)]
pub struct World {
    entities: EntityManager<ArchLink>,
    archetypes: ArchetypeManager,
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
        let indexer = self.archetypes.insert(entity, set);
        self.entities.replace_data(&entity, ArchLink::new(indexer));
        entity
    }

    /// Modifies the pearls in `entity` using the provided `f`.
    ///
    /// If `entity` is not valid for this world, nothing will happen and `f` will not execute.
    pub fn modify_entity(&mut self, entity: &Entity, f: impl FnOnce(&mut PearlSet)) {
        // check if the entity is valid while getting the indexer
        let Some(link) = self.entities.get_data(entity) else { return };
        if link.active {
            return;
        }

        // get the archetype and swap remove the entities data
        let indexer = link.indexer;
        let (mut set, swapped) = self.archetypes.swap_remove(&indexer);

        // execute the modify method
        f(&mut set);

        // insert the entity into its new archetype
        self.archetypes.insert(entity.clone(), set);
        self.entities.replace_data(&entity, ArchLink::new(indexer));

        // fix the swapped entity if necessary
        let Some(swapped_entity) = swapped else { return };
        let swapped_data = self.entities.get_data_mut(&swapped_entity).unwrap();
        swapped_data.indexer = indexer;
    }

    /// Inserts or replaces a pearl in a given entity.
    ///
    /// This is a simple wrapper for `modify_entity` under the hood.
    #[inline]
    pub fn insert_or_replace_pearl<T: Pearl>(&mut self, entity: &Entity, pearl: T) -> Option<T> {
        let mut replaced = None;
        self.modify_entity(entity, |set| {
            replaced = set.insert_or_replace(pearl);
        });
        replaced
    }

    /// Removes a pearl in a given entity.
    ///
    /// This is a simple wrapper for `modify_entity` under the hood.
    #[inline]
    pub fn remove_pearl<T: Pearl>(&mut self, entity: &Entity) -> Option<T> {
        let mut removed = None;
        self.modify_entity(entity, |set| {
            removed = set.remove::<T>();
        });
        removed
    }

    /// Destroys the `entity` in this world, returning `true`.
    ///
    /// Returns `false` if `entity` was already invalid for this world.
    pub fn destroy_entity(&mut self, entity: &Entity) {
        // check if the entity is valid while getting the indexer
        let Some(link) = self.entities.destroy(entity) else { return };
        if !link.active {
            return;
        }

        // get the archetype and swap destroy the entity, fixing the swapped index after
        let Some(swapped_entity) = self.archetypes.swap_destroy(&link.indexer) else { return };
        let swapped_data = self.entities.get_data_mut(&swapped_entity).unwrap();
        swapped_data.indexer = link.indexer;
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
        pearl_set.insert_or_replace(Struct1(43));
        pearl_set.insert_or_replace(Struct2(44));
        pearl_set.insert_or_replace(Struct3(45));
        let id_set = pearl_set.id_set().clone();

        let mut world = World::new();
        let entity = world.create_entity_with_pearls(pearl_set);
        assert!(world.has_entity(&entity));

        assert!(world.archetypes.contains_archetype(&id_set));
        let pearl_link = world.entities.get_data(&entity).unwrap();
        assert!(pearl_link.active);

        let arch_index = pearl_link.indexer.archetype;
        let archetype = world.archetypes.get_index(arch_index).unwrap();
        assert!(archetype.len() == 1);
    }

    #[test]
    fn destroy_entities() {
        let mut world = World::new();
        let entity0 = world.create_entity();
        let entity1 = world.create_entity();
        let entity2 = world.create_entity();

        world.destroy_entity(&entity1);
        let entity3 = world.create_entity();

        assert!(world.has_entity(&entity0));
        assert!(!world.has_entity(&entity1));
        assert!(world.has_entity(&entity2));
        assert!(world.has_entity(&entity3));
    }

    #[test]
    fn destroy_entity_with_pearls() {
        let mut pearl_set = PearlSet::new_with(Struct0(42));
        pearl_set.insert_or_replace(Struct1(43));
        pearl_set.insert_or_replace(Struct2(44));
        pearl_set.insert_or_replace(Struct3(45));
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
