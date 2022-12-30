use std::any::{Any, TypeId};

use hashbrown::HashMap;

pub trait ResourceCollector {
    fn add<T>(&mut self, resource: T) -> Option<T>
    where
        T: 'static;
}

#[derive(Default)]
pub struct BobaResources {
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl ResourceCollector for BobaResources {
    fn add<T>(&mut self, resource: T) -> Option<T>
    where
        T: 'static,
    {
        let old_any = self
            .resources
            .insert(TypeId::of::<T>(), Box::new(resource))?;

        Some(*old_any.downcast::<T>().unwrap())
    }
}

impl BobaResources {
    pub fn get<T>(&self) -> Option<&T>
    where
        T: 'static,
    {
        let any = self.resources.get(&TypeId::of::<T>())?;
        Some(any.as_ref().downcast_ref::<T>().unwrap())
    }

    pub fn get_mut<T>(&mut self) -> Option<&mut T>
    where
        T: 'static,
    {
        let any = self.resources.get_mut(&TypeId::of::<T>())?;
        Some(any.as_mut().downcast_mut::<T>().unwrap())
    }

    pub fn remove<T>(&mut self) -> Option<T>
    where
        T: 'static,
    {
        let any = self.resources.remove(&TypeId::of::<T>())?;
        Some(*any.downcast::<T>().unwrap())
    }
}
