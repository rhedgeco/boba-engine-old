use std::sync::atomic::{AtomicU16, Ordering};

use crate::Entity;

pub struct EntityCollection<T> {
    collection_id: u16,
    entities: Vec<(Entity, Option<T>)>,
    open: Vec<u32>,
}

impl<T> Default for EntityCollection<T> {
    fn default() -> Self {
        static ID_COUNTER: AtomicU16 = AtomicU16::new(0);

        Self {
            collection_id: ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            entities: Vec::new(),
            open: Vec::new(),
        }
    }
}

impl<T> EntityCollection<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.entities.len() - self.open.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entities.len() == self.open.len()
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        match self.entities.get(entity.uindex()) {
            Some(stored_entity) if stored_entity.0 == entity => true,
            _ => false,
        }
    }

    pub fn get_data(&self, entity: Entity) -> Option<&T> {
        let (stored_entity, data) = self.entities.get(entity.uindex())?;
        if stored_entity != &entity {
            return None;
        }

        data.as_ref()
    }

    pub fn get_data_mut(&mut self, entity: Entity) -> Option<&mut T> {
        let (stored_entity, data) = self.entities.get_mut(entity.uindex())?;
        if stored_entity != &entity {
            return None;
        }

        data.as_mut()
    }

    pub fn spawn(&mut self, data: T) -> Entity {
        match self.open.pop() {
            Some(index) => self.entities[index as usize].0,
            None => {
                let uindex = self.entities.len();
                let index = u32::try_from(uindex).expect("EntityCollection overflow.");
                let entity = Entity::from_raw_parts(index, 0, self.collection_id);
                self.entities.push((entity, Some(data)));
                entity
            }
        }
    }

    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        let (stored_entity, data) = self.entities.get_mut(entity.uindex())?;
        if stored_entity != &entity {
            return None;
        }

        stored_entity.increment_generation();
        self.open.push(stored_entity.index());
        data.take()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length() {
        let mut collection = EntityCollection::new();
        assert!(collection.len() == 0);
        assert!(collection.is_empty());

        let e1 = collection.spawn(());
        let e2 = collection.spawn(());
        assert!(collection.len() == 2);
        assert!(!collection.is_empty());

        collection.remove(e1);
        assert!(collection.len() == 1);
        assert!(!collection.is_empty());

        collection.remove(e2);
        assert!(collection.len() == 0);
        assert!(collection.is_empty());
    }

    #[test]
    fn from_open() {
        let mut collection = EntityCollection::new();
        let e1 = collection.spawn(());
        let e2 = collection.spawn(());
        assert!(e1.index() == 0);
        assert!(e2.index() == 1);

        collection.remove(e1);
        let e1 = collection.spawn(());

        assert!(e1.index() == 0);
    }

    #[test]
    fn alive() {
        let mut collection = EntityCollection::new();
        let e1 = collection.spawn(());
        let e2 = collection.spawn(());
        assert!(collection.is_alive(e1));
        assert!(collection.is_alive(e2));

        collection.remove(e1);
        assert!(!collection.is_alive(e1));
        assert!(collection.is_alive(e2));
    }
}
