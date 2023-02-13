use crate::Entities;

pub struct World {
    pub entities: Entities,
}

impl Default for World {
    fn default() -> Self {
        Self {
            entities: Entities::new(),
        }
    }
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }
}
