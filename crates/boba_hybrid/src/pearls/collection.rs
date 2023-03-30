use std::any::Any;

use handle_map::{map::dense::DenseHandleMap, Handle};
use hashbrown::{hash_map::Entry, HashMap};

use super::{Pearl, PearlId};

/// A storage solution for [`Pearl`] objects.
/// Stored pearls in a densly packed array, for quick iteration.
/// But also provides a handle for their location for quick access.
#[derive(Default)]
pub struct PearlCollection {
    pearls: HashMap<PearlId, Box<dyn Any>>,
}

impl PearlCollection {
    /// Returns a new collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a [`Pearl`] to this collection, returning a handle to its location.
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

    /// Returns true if `handle` is valid for this collection.
    pub fn contains<T: Pearl>(&self, handle: &Handle<T>) -> bool {
        match self.get_map::<T>() {
            Some(map) => map.contains(handle),
            _ => false,
        }
    }

    /// Returns true if a pearl of type `T` is stored in this collection.
    pub fn contains_type<T: Pearl>(&self) -> bool {
        self.pearls.contains_key(&PearlId::of::<T>())
    }

    /// Returns a reference to the pearl that `handle` points to.
    ///
    /// Returns `None` if the handle is invalid.
    pub fn get<T: Pearl>(&self, handle: &Handle<T>) -> Option<&T> {
        let map = self.get_map::<T>()?;
        map.get_data(handle)
    }

    /// Returns a mutable reference to the pearl that `handle` points to.
    ///
    /// Returns `None` if the handle is invalid.
    pub fn get_mut<T: Pearl>(&mut self, handle: &Handle<T>) -> Option<&mut T> {
        let map = self.get_map_mut::<T>()?;
        map.get_data_mut(handle)
    }

    /// Removes and returns the pearl that `handle` points to.
    ///
    /// Returns `None` if the handle is invalid.
    pub fn remove<T: Pearl>(&mut self, handle: &Handle<T>) -> Option<T> {
        let map = self.get_map_mut::<T>()?;
        map.remove(handle)
    }

    /// Returns the densly packed slice reference for all pearls of type `T`.
    ///
    /// Returns `None` if the handle is invalid.
    pub fn get_slice<T: Pearl>(&self) -> Option<&[T]> {
        let map = self.get_map::<T>()?;
        Some(map.as_slice())
    }

    /// Returns the densly packed mutable slice reference for all pearls of type `T`.
    ///
    /// Returns `None` if the handle is invalid.
    pub fn get_slice_mut<T: Pearl>(&mut self) -> Option<&mut [T]> {
        let map = self.get_map_mut::<T>()?;
        Some(map.as_slice_mut())
    }

    /// Returns the densly packed slice for all handles of type `T` in this collection.
    ///
    /// Returns `None` if the handle is invalid.
    pub fn get_handles<T: Pearl>(&self) -> Option<&[Handle<T>]> {
        let map = self.get_map::<T>()?;
        Some(map.as_handles_slice())
    }

    fn get_map<T: Pearl>(&self) -> Option<&DenseHandleMap<T>> {
        let map = self.pearls.get(&PearlId::of::<T>())?;
        Some(map.downcast_ref::<DenseHandleMap<T>>().unwrap())
    }

    fn get_map_mut<T: Pearl>(&mut self) -> Option<&mut DenseHandleMap<T>> {
        let map = self.pearls.get_mut(&PearlId::of::<T>())?;
        Some(map.downcast_mut::<DenseHandleMap<T>>().unwrap())
    }
}
