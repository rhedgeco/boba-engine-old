use std::collections::VecDeque;

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
    pub fn is_alive(&self, entities: &Entities) -> bool {
        match entities.entities.get(self.uindex()) {
            Some(entity) if self == entity => true,
            _ => false,
        }
    }
}

pub struct Entities {
    entities: Vec<EntityId>,
    dead: VecDeque<usize>,
}

impl Entities {
    pub(crate) fn new() -> Self {
        Self {
            entities: Vec::new(),
            dead: VecDeque::new(),
        }
    }

    pub fn create(&mut self) -> EntityId {
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

    pub fn destroy(&mut self, entity: &EntityId) {
        let index = entity.uindex();
        // if the entity is in the list and matches
        if let Some(self_entity) = self.entities.get_mut(index) {
            if self_entity != entity {
                return;
            }

            // increment the id to stop it from matching in the future
            self_entity.increment_id();

            // add its index to the dropped queue
            self.dead.push_back(index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_entity() {
        let mut entities = Entities::new();
        let entity0 = entities.create();
        let entity1 = entities.create();
        assert!(entities.entities.len() == 2);
        assert!(entity0.index() == 0);
        assert!(entity1.index() == 1);
    }

    #[test]
    fn destroy_entity() {
        let mut entities = Entities::new();
        let entity0 = entities.create();
        entities.destroy(&entity0);
        assert!(entities.entities.len() == 1);
        assert!(entities.dead.len() == 1);
        let entity1 = entities.create();
        assert!(entities.entities.len() == 1);
        assert!(entities.dead.len() == 0);
        assert!(!entity0.is_alive(&entities));
        assert!(entity1.is_alive(&entities));
    }
}
