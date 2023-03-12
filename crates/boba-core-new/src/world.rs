use std::any::{Any, TypeId};

use handle_map::{dense_map::DenseHandleMap, Handle};
use hashbrown::{hash_map::Entry, HashMap};

#[derive(Default)]
pub struct World {
    pearls: HashMap<TypeId, Box<dyn Any>>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_pearl<T: 'static>(&mut self, item: T) -> Handle<T> {
        match self.pearls.entry(TypeId::of::<T>()) {
            Entry::Occupied(e) => {
                let map = e.into_mut().downcast_mut::<DenseHandleMap<T>>().unwrap();
                map.insert(item)
            }
            Entry::Vacant(e) => {
                let mut map = DenseHandleMap::new();
                let handle = map.insert(item);
                e.insert(Box::new(map));
                handle
            }
        }
    }

    #[inline]
    pub fn get_pearl<T: 'static>(&self, handle: &Handle<T>) -> Option<&T> {
        let any_map = self.pearls.get(&TypeId::of::<T>())?;
        let map = any_map.downcast_ref::<DenseHandleMap<T>>().unwrap();
        map.get(handle)
    }

    #[inline]
    pub fn get_pearl_mut<T: 'static>(&mut self, handle: &Handle<T>) -> Option<&mut T> {
        let any_map = self.pearls.get_mut(&TypeId::of::<T>())?;
        let map = any_map.downcast_mut::<DenseHandleMap<T>>().unwrap();
        map.get_mut(handle)
    }

    #[inline]
    pub fn remove_pearl<T: 'static>(&mut self, handle: &Handle<T>) -> Option<T> {
        let any_map = self.pearls.get_mut(&TypeId::of::<T>())?;
        let map = any_map.downcast_mut::<DenseHandleMap<T>>().unwrap();
        map.remove(handle)
    }
}
