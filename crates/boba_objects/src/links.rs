use std::marker::PhantomData;

use uuid::Uuid;

use crate::{BobaComponent, BobaObject, BobaWorld};

pub struct ObjectLink {
    pub(crate) uuid: Uuid,
    pub(crate) world_index: usize,
}

pub struct UntypedComponentLink {
    pub(crate) uuid: Uuid,
    pub(crate) world_index: usize,
}

pub struct ComponentLink<T> {
    pub(crate) link: UntypedComponentLink,
    _phantom: PhantomData<T>,
}

impl ObjectLink {
    pub fn extract<'a>(&self, world: &'a BobaWorld) -> Option<&'a BobaObject> {
        match world.objects.get(self.world_index) {
            Some(Some(object)) if object.uuid == self.uuid => Some(object),
            _ => None,
        }
    }

    pub fn extract_mut<'a>(&self, world: &'a mut BobaWorld) -> Option<&'a mut BobaObject> {
        match world.objects.get_mut(self.world_index) {
            Some(Some(object)) if object.uuid == self.uuid => Some(object),
            _ => None,
        }
    }
}

impl<T> ComponentLink<T> {
    pub(crate) fn new(uuid: Uuid, world_index: usize) -> Self {
        Self {
            link: UntypedComponentLink { uuid, world_index },
            _phantom: PhantomData,
        }
    }

    pub fn extract<'a>(&self, world: &'a BobaWorld) -> Option<&'a BobaComponent> {
        match world.components.get(self.link.world_index) {
            Some(Some(component)) if component.uuid == self.link.uuid => Some(component),
            _ => None,
        }
    }

    pub fn extract_mut<'a>(&self, world: &'a mut BobaWorld) -> Option<&'a mut BobaComponent> {
        match world.components.get_mut(self.link.world_index) {
            Some(Some(component)) if component.uuid == self.link.uuid => Some(component),
            _ => None,
        }
    }
}
