use std::slice::{Iter, IterMut};

use crate::{
    pearl::{id::PearlIdSet, set::PearlMatrix, PearlSet},
    Entity, Pearl,
};

#[derive(Debug)]
pub struct Archetype {
    entities: Vec<Entity>,
    pearls: PearlMatrix,
}

impl Archetype {
    pub fn from_set(entity: Entity, set: PearlSet) -> Self {
        Self {
            entities: vec![entity],
            pearls: PearlMatrix::from_set(set),
        }
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    pub fn id_set(&self) -> &PearlIdSet {
        self.pearls.id_set()
    }

    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }

    pub fn iter<P: Pearl>(&self) -> Option<Iter<P>> {
        self.pearls.iter()
    }

    pub fn iter_mut<P: Pearl>(&mut self) -> Option<IterMut<P>> {
        self.pearls.iter_mut()
    }

    pub fn fetch_iter(&mut self) -> IterFetcher {
        IterFetcher::new(self)
    }

    pub fn push(&mut self, entity: Entity, set: PearlSet) {
        self.pearls.push(set);
        self.entities.push(entity);
    }

    pub fn swap_remove(&mut self, index: usize) -> (Entity, PearlSet) {
        let entity = self.entities.swap_remove(index);
        let set = self.pearls.swap_remove(index);
        (entity, set)
    }

    pub fn swap_drop(&mut self, index: usize) -> Entity {
        let entity = self.entities.swap_remove(index);
        self.pearls.swap_drop(index);
        entity
    }
}

pub struct IterFetcher<'a> {
    fetched_entities: bool,
    entities: &'a Vec<Entity>,
    inner: crate::pearl::set::IterFetcher<'a>,
}

impl<'a> IterFetcher<'a> {
    pub fn new(archetype: &'a mut Archetype) -> Self {
        Self {
            fetched_entities: false,
            entities: &archetype.entities,
            inner: archetype.pearls.fetch_iter(),
        }
    }

    pub fn entities(&mut self) -> Option<&[Entity]> {
        if self.fetched_entities {
            return None;
        }

        self.fetched_entities = true;
        Some(unsafe { std::mem::transmute(self.entities.as_slice()) })
    }

    pub fn get<P: Pearl>(&mut self) -> Option<IterMut<P>> {
        self.inner.get()
    }
}
