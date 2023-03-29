use std::any::{Any, TypeId};

use hashbrown::HashMap;

#[derive(Default)]
pub struct BobaResources {
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl BobaResources {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<T: 'static>(&mut self, resource: T) {
        self.resources.insert(TypeId::of::<T>(), Box::new(resource));
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        let any = self.resources.get(&TypeId::of::<T>())?;
        Some(any.downcast_ref::<T>().unwrap())
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        let any = self.resources.get_mut(&TypeId::of::<T>())?;
        Some(any.downcast_mut::<T>().unwrap())
    }

    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        let any = self.resources.remove(&TypeId::of::<T>())?;
        Some(*any.downcast::<T>().unwrap())
    }
}
