use std::{
    collections::VecDeque,
    sync::atomic::{AtomicU16, Ordering},
};

use crate::Entity;

struct EntityEntry {
    entity: Entity,
    _data: (), // to be used later
}

impl EntityEntry {
    pub fn new(entity: Entity) -> Self {
        Self { entity, _data: () }
    }
}

#[derive(Default)]
pub struct World {
    id: u16,
    entities: Vec<EntityEntry>,
    open_entities: VecDeque<usize>,
}

impl World {
    #[inline]
    pub fn new() -> Self {
        static ID_GEN: AtomicU16 = AtomicU16::new(0);

        Self {
            id: ID_GEN.fetch_add(1, Ordering::Relaxed),
            entities: Vec::new(),
            open_entities: VecDeque::new(),
        }
    }

    pub fn entity(&mut self) -> Entity {
        match self.open_entities.pop_front() {
            Some(index) => {
                let entry = &self.entities[index];
                entry.entity
            }
            None => {
                let index = self.entities.len() as u32;
                if index > u32::MAX {
                    panic!("Entity Capacity Overflow");
                }

                let entity = unsafe { Entity::from_raw_parts(index, 0, self.id) };
                self.entities.push(EntityEntry::new(entity));
                entity
            }
        }
    }

    pub fn has_entity(&self, entity: &Entity) -> bool {
        match self.entities.get(entity.uindex()) {
            Some(other) => other.entity.into_raw() == entity.into_raw(),
            None => false,
        }
    }

    pub fn destroy_entity(&mut self, entity: &Entity) -> bool {
        match self.entities.get_mut(entity.uindex()) {
            Some(other) if &other.entity == entity => {
                let (index, gen, meta) = other.entity.into_raw_parts();
                other.entity = unsafe { Entity::from_raw_parts(index, gen.wrapping_add(1), meta) };
                self.open_entities.push_back(entity.uindex());
                return true;
            }
            _ => return false,
        }
    }
}
