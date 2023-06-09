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

#[derive(Debug, Default)]
pub struct World {
    entities: EntityMap<Option<EntityData>>,
    archetypes: IndexMap<PearlIdSet, Archetype>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        self.entities.is_alive(entity)
    }

    pub fn spawn(&mut self) -> Entity {
        self.entities.spawn(None)
    }

    pub fn spawn_with(&mut self, set: PearlSet) -> Entity {
        let entity = self.spawn();
        self.set_pearls(entity, set);
        entity
    }

    pub fn set_pearls(&mut self, entity: Entity, set: PearlSet) -> bool {
        let Some(data_option) = self.entities.get(entity) else { return false };

        if let Some(entity_data) = data_option {
            let archetype = &mut self.archetypes[entity_data.arch_index];
            archetype.swap_drop(entity_data.pearl_index);
            if let Some(swapped_entity) = archetype.entities().get(entity_data.pearl_index) {
                self.entities.set(*swapped_entity, Some(*entity_data));
            }
        }

        use indexmap::map::Entry::*;
        match self.archetypes.entry(set.id_set().clone()) {
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
                archetype.push(entity, set);
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
                e.insert(Archetype::from_set(entity, set));
            }
        }

        true
    }

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

    pub fn query_match(&self, ids: &PearlIdSet) -> Option<&Archetype> {
        self.archetypes.get(ids)
    }

    pub fn query_match_mut(&mut self, ids: &PearlIdSet) -> Option<&mut Archetype> {
        self.archetypes.get_mut(ids)
    }

    pub fn query_contains<'a>(&'a mut self, ids: &'a PearlIdSet) -> QueryContains {
        QueryContains::new(ids, self)
    }
}

pub struct QueryContains<'a> {
    query_ids: &'a PearlIdSet,
    archetypes: indexmap::map::ValuesMut<'a, PearlIdSet, Archetype>,
}

impl<'a> QueryContains<'a> {
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
            if arch.id_set().is_subset(self.query_ids) {
                return Some(arch);
            }
        }
    }
}
