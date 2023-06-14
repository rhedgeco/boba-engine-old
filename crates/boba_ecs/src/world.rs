use indexmap::IndexMap;

use crate::{
    entity::EntityMap,
    pearl::{id::PearlIdSet, PearlSet},
    Archetype, Entity,
};

#[derive(Debug, Clone, Copy)]
struct EntityData {
    arch_index: usize,
    pearl_index: usize,
}

/// The central storage structure for [`Entity`] structs and its associated [`Pearl`](crate::Pearl)
/// types. Each [`Entity`] will have an [`Archetype`] and they may be queried to obtain the data.
#[derive(Debug, Default)]
pub struct World {
    entities: EntityMap<Option<EntityData>>,
    archetypes: IndexMap<PearlIdSet, Archetype>,
}

impl World {
    /// Returns a new empty world.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns `true` if `entity` exists and is alive in this world.
    pub fn is_alive(&self, entity: Entity) -> bool {
        self.entities.is_alive(entity)
    }

    /// Spawns and returns a new empty [`Entity`].
    pub fn spawn(&mut self) -> Entity {
        self.entities.spawn(None)
    }

    /// Spawns and returns a new [`Entity`] with the provided [`PearlSet`].
    pub fn spawn_with(&mut self, pearls: PearlSet) -> Entity {
        let entity = self.spawn();
        self.set_pearls(entity, pearls);
        entity
    }

    /// Inserts the provided [`PearlSet`] into `entity`.
    /// If `entity` already had existing pearls, they will be dropped.
    /// Returns `false` if `entity` does not exist in this world.
    pub fn set_pearls(&mut self, entity: Entity, pearls: PearlSet) -> bool {
        let Some(data_option) = self.entities.get(entity) else { return false };

        if let Some(entity_data) = data_option {
            let archetype = &mut self.archetypes[entity_data.arch_index];
            archetype.swap_drop(entity_data.pearl_index);
            if let Some(swapped_entity) = archetype.entities().get(entity_data.pearl_index) {
                self.entities.set(*swapped_entity, Some(*entity_data));
            }
        }

        use indexmap::map::Entry::*;
        match self.archetypes.entry(pearls.id_set().clone()) {
            Occupied(e) => {
                let arch_index = e.index();
                let archetype = e.into_mut();
                let pearl_index = archetype.len();
                self.entities.set(
                    entity,
                    Some(EntityData {
                        arch_index,
                        pearl_index,
                    }),
                );
                archetype.push(entity, pearls);
            }
            Vacant(e) => {
                let arch_index = e.index();
                self.entities.set(
                    entity,
                    Some(EntityData {
                        arch_index,
                        pearl_index: 0,
                    }),
                );
                e.insert(Archetype::from_set(entity, pearls));
            }
        }

        true
    }

    /// Modifies the [`PearlSet`] of `entity` using the provided callback `f`.
    /// Returns `false` if `entity` does not exist in this world.
    pub fn modify<F>(&mut self, entity: Entity, f: F) -> bool
    where
        F: FnOnce(Option<PearlSet>) -> Option<PearlSet>,
    {
        let Some(data_option) = self.entities.get(entity) else { return false };

        let pearls = match *data_option {
            Some(entity_data) => {
                let archetype = &mut self.archetypes[entity_data.arch_index];
                let (_, pearls) = archetype.swap_remove(entity_data.pearl_index);
                if let Some(swapped_entity) = archetype.entities().get(entity_data.pearl_index) {
                    self.entities.set(*swapped_entity, Some(entity_data));
                }
                Some(pearls)
            }
            None => None,
        };

        let Some(new_pearls) = f(pearls) else {
            self.entities.set(entity, None);
            return true;
        };

        self.set_pearls(entity, new_pearls);
        true
    }

    /// Destroys `entity` in this world and drops all of its associated [`Pearl`](crate::Pearl)
    /// objects. Returns `false` if `entity` does not exist in this world.
    pub fn destroy(&mut self, entity: Entity) -> bool {
        let Some(data_option) = self.entities.remove(entity) else { return false };
        let Some(entity_data) = data_option else { return true };

        let archetype = &mut self.archetypes[entity_data.arch_index];
        archetype.swap_drop(entity_data.pearl_index);
        if let Some(swapped_entity) = archetype.entities().get(entity_data.pearl_index) {
            self.entities.set(*swapped_entity, Some(entity_data));
        }

        true
    }

    /// Returns a reference to the [`Archetype`] associated with the given [`PearlIdSet`].
    /// Returns `None` if there is no related archetype.
    pub fn query_match(&self, ids: &PearlIdSet) -> Option<&Archetype> {
        self.archetypes.get(ids)
    }

    /// Returns mutable a reference to the [`Archetype`] associated with the given [`PearlIdSet`].
    /// Returns `None` if there is no related archetype.
    pub fn query_match_mut(&mut self, ids: &PearlIdSet) -> Option<&mut Archetype> {
        self.archetypes.get_mut(ids)
    }

    /// Returns an iterator ([`QueryContains`]) that iterates over every archetype that contains a
    /// superset of the provided [`PearlIdSet`].
    pub fn query_contains<'a>(&'a mut self, ids: &'a PearlIdSet) -> QueryContains {
        QueryContains::new(ids, self)
    }
}

/// An iterator over every [`Archetype`] in a [`World`] that is a superset of a given
/// [`PearlIdSet`].
pub struct QueryContains<'a> {
    query_ids: &'a PearlIdSet,
    archetypes: indexmap::map::ValuesMut<'a, PearlIdSet, Archetype>,
}

impl<'a> QueryContains<'a> {
    /// Returns a new [`QueryContains`] over the archetypes in `world` using `query_ids`.
    pub fn new(query_ids: &'a PearlIdSet, world: &'a mut World) -> Self {
        Self {
            query_ids,
            archetypes: world.archetypes.values_mut(),
        }
    }
}

impl<'a> Iterator for QueryContains<'a> {
    type Item = &'a mut Archetype;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let arch = self.archetypes.next()?;
            if self.query_ids.is_superset(arch.id_set()) {
                return Some(arch);
            }
        }
    }
}
