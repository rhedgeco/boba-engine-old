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

    pub fn insert(&mut self, set: PearlSet) -> usize {
        assert!(self.pearls.len() == set.id_set().len());
        let index = self.len;
        for (i, imposter) in set.into_vec().into_iter().enumerate() {
            self.pearls[i].push_imposter(imposter).unwrap();
        }
        self.len += 1;
        index
    }

    pub fn swap_destroy(&mut self, index: usize) -> Entity {
        assert!(index < self.len);
        for vec in self.pearls.iter_mut() {
            vec.swap_drop(index);
        }
        self.len -= 1;
        self.entities.swap_remove(index)
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
                let pearl_index = archetype.insert(set);
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
            let swapped_entity = self.archetypes[link_data.archetype].swap_destroy(link_data.pearl);
            let swapped_data = self.entities.get_data_mut(&swapped_entity).unwrap();
            swapped_data.pearl = link_data.pearl;
        }

        true
    }
}
