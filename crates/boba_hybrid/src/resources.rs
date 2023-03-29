use std::any::{Any, TypeId};

use hashbrown::HashMap;

pub trait ResourceManager {
    fn insert<T: 'static>(&mut self, resource: T);
    fn get<T: 'static>(&self) -> Option<&T>;
    fn get_mut<T: 'static>(&mut self) -> Option<&mut T>;
    fn remove<T: 'static>(&mut self) -> Option<T>;
}

#[derive(Default)]
pub struct BobaResources {
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl BobaResources {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ResourceManager for BobaResources {
    fn insert<T: 'static>(&mut self, resource: T) {
        self.resources.insert(TypeId::of::<T>(), Box::new(resource));
    }

    fn get<T: 'static>(&self) -> Option<&T> {
        let any = self.resources.get(&TypeId::of::<T>())?;
        Some(any.downcast_ref::<T>().unwrap())
    }

    fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        let any = self.resources.get_mut(&TypeId::of::<T>())?;
        Some(any.downcast_mut::<T>().unwrap())
    }

    fn remove<T: 'static>(&mut self) -> Option<T> {
        let any = self.resources.remove(&TypeId::of::<T>())?;
        Some(*any.downcast::<T>().unwrap())
    }
}
