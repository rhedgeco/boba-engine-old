use uuid::Uuid;

use crate::{BobaComponent, BobaObject, ObjectLink};

pub struct BobaWorld {
    pub(crate) objects: Vec<Option<BobaObject>>,
    pub(crate) components: Vec<Option<BobaComponent>>,
}

impl BobaWorld {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            components: Vec::new(),
        }
    }

    pub fn spawn_object(&mut self) -> ObjectLink {
        let uuid = Uuid::new_v4();
        let world_index = self.objects.len();
        self.objects.push(Some(BobaObject {
            uuid,
            components: Vec::new(),
        }));
        ObjectLink { uuid, world_index }
    }
}
