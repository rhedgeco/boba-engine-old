use std::any::{Any, TypeId};

use handle_map::{map::dense::DenseHandleMap, Handle};
use hashbrown::{hash_map::Entry, HashMap};

use crate::event::EventRegistrar;

pub trait Pearl: Sized + 'static {
    fn register(registrar: &mut impl EventRegistrar<Self>);
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PearlId(TypeId);

impl PearlId {
    /// Returns the id for pearl of type `T`
    #[inline]
    pub fn of<T: Pearl>() -> Self {
        Self(TypeId::of::<T>())
    }

    /// Returns the underlying [`TypeId`]
    #[inline]
    pub fn into_raw(self) -> TypeId {
        self.0
    }
}

pub trait PearlAccess {
    fn get<T: Pearl>(&self, handle: &Handle<T>) -> Option<&T>;
    fn get_mut<T: Pearl>(&mut self, handle: &Handle<T>) -> Option<&mut T>;
    fn get_resource<T: 'static>(&self) -> Option<&T>;
    fn get_resource_mut<T: 'static>(&mut self) -> Option<&mut T>;
}

#[derive(Default)]
pub struct PearlCollection {
    pearls: HashMap<PearlId, Box<dyn Any>>,
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl PearlCollection {
    fn get_map<T: Pearl>(&self) -> Option<&DenseHandleMap<T>> {
        let map = self.pearls.get(&PearlId::of::<T>())?;
        Some(map.downcast_ref::<DenseHandleMap<T>>().unwrap())
    }

    fn get_map_mut<T: Pearl>(&mut self) -> Option<&mut DenseHandleMap<T>> {
        let map = self.pearls.get_mut(&PearlId::of::<T>())?;
        Some(map.downcast_mut::<DenseHandleMap<T>>().unwrap())
    }
}

impl PearlAccess for PearlCollection {
    fn get<T: Pearl>(&self, handle: &Handle<T>) -> Option<&T> {
        let map = self.get_map::<T>()?;
        map.get_data(handle)
    }

    fn get_mut<T: Pearl>(&mut self, handle: &Handle<T>) -> Option<&mut T> {
        let map = self.get_map_mut::<T>()?;
        map.get_data_mut(handle)
    }

    fn get_resource<T: 'static>(&self) -> Option<&T> {
        let any = self.resources.get(&TypeId::of::<T>())?;
        Some(any.downcast_ref::<T>().unwrap())
    }

    fn get_resource_mut<T: 'static>(&mut self) -> Option<&mut T> {
        let any = self.resources.get_mut(&TypeId::of::<T>())?;
        Some(any.downcast_mut::<T>().unwrap())
    }
}

impl PearlCollection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn contains<T: Pearl>(&self) -> bool {
        self.pearls.contains_key(&PearlId::of::<T>())
    }

    pub fn insert<T: Pearl>(&mut self, pearl: T) -> Handle<T> {
        let map = match self.pearls.entry(PearlId::of::<T>()) {
            Entry::Occupied(e) => e.into_mut().downcast_mut::<DenseHandleMap<T>>().unwrap(),
            Entry::Vacant(e) => {
                let any = e.insert(Box::new(DenseHandleMap::<T>::new()));
                any.downcast_mut::<DenseHandleMap<T>>().unwrap()
            }
        };

        map.insert(pearl)
    }

    pub fn insert_resource<T: 'static>(&mut self, resource: T) {
        self.resources.insert(TypeId::of::<T>(), Box::new(resource));
    }

    pub fn remove<T: Pearl>(&mut self, handle: &Handle<T>) -> Option<T> {
        let map = self.get_map_mut::<T>()?;
        map.remove(handle)
    }

    pub fn remove_resource<T: 'static>(&mut self) -> Option<T> {
        let any = self.resources.remove(&TypeId::of::<T>())?;
        Some(*any.downcast::<T>().unwrap())
    }

    pub fn as_slice<T: Pearl>(&self) -> Option<&[T]> {
        let map = self.get_map::<T>()?;
        Some(map.as_slice())
    }

    pub fn as_slice_mut<T: Pearl>(&mut self) -> Option<&mut [T]> {
        let map = self.get_map_mut::<T>()?;
        Some(map.as_slice_mut())
    }

    pub fn as_slice_handles<T: Pearl>(&self) -> Option<&[Handle<T>]> {
        let map = self.get_map::<T>()?;
        Some(map.as_slice_handles())
    }
}
