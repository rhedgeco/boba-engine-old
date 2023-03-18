use crate::{Entity, EntityManager};

#[derive(Default)]
pub struct World {
    entities: EntityManager<()>,
}

impl World {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn create_entity(&mut self) -> Entity {
        self.entities.create(())
    }

    #[inline]
    pub fn has_entity(&self, entity: &Entity) -> bool {
        self.entities.contains(entity)
    }

    #[inline]
    pub fn destroy_entity(&mut self, entity: &Entity) -> bool {
        self.entities.destroy(entity)
    }
}
