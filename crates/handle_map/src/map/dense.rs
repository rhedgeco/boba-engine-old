use std::{
    slice::{Iter, IterMut},
    vec::IntoIter,
};

use crate::Handle;

use super::{sparse::SparseHandleMap, HandleMapId};

/// A storage solution that gives a [`Handle`] to the location of the data.
/// This is optimized for fast access, as the [`Handle`] ensures an array indexing operation.
///
/// This map is ***Dense*** because all the data is stored right next to eachother in memory.
/// This means that when accessing with a handle, there needs to be one more level of indirection under the covers.
/// However, iteration over the map will always be maximally efficient,
/// as the whole map can be used as a tightly packed array slice.
pub struct DenseHandleMap<T> {
    id: u16,
    link_map: SparseHandleMap<usize>,
    back_link: Vec<Handle<usize>>,
    data: Vec<T>,
}

impl<T> Default for DenseHandleMap<T> {
    /// Returns a default handle map with a unique id.
    #[inline]
    fn default() -> Self {
        Self {
            id: HandleMapId::generate(),
            link_map: Default::default(),
            back_link: Default::default(),
            data: Default::default(),
        }
    }
}

impl<T> DenseHandleMap<T> {
    /// Returns a new handle map with a unique id.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the undelying id for this map.
    #[inline]
    pub fn id(&self) -> u16 {
        self.id
    }

    /// Returns the number of items in the map.
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the map is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Inserts `data` into the map, and returns a [`Handle`] to its location.
    #[inline]
    pub fn insert(&mut self, data: T) -> Handle<T> {
        let handle = self.link_map.insert(self.data.len());
        self.back_link.push(handle);
        self.data.push(data);
        handle.into_type::<T>()
    }

    /// Returns true if `handle` is valid for this map.
    #[inline]
    pub fn contains(&self, handle: &Handle<T>) -> bool {
        self.link_map.contains(handle.as_type::<usize>())
    }

    /// Returns a reference to the data associated with `handle`.
    ///
    /// Returns `None` if the handle is invalid.
    #[inline]
    pub fn get_data(&self, handle: &Handle<T>) -> Option<&T> {
        let index = self.link_map.get_data(handle.as_type::<usize>())?;
        Some(&self.data[*index])
    }

    /// Returns a mutable reference to the data associated with `handle`.
    ///
    /// Returns `None` if the handle is invalid.
    #[inline]
    pub fn get_data_mut(&mut self, handle: &Handle<T>) -> Option<&mut T> {
        let index = self.link_map.get_data(handle.as_type::<usize>())?;
        Some(&mut self.data[*index])
    }

    /// Removes and returns the data associated with `handle` from this map.
    ///
    /// Returns `None` if the handle is invalid.
    #[inline]
    pub fn remove(&mut self, handle: &Handle<T>) -> Option<T> {
        // get the index for the handle
        let index = self.link_map.remove(handle.as_type::<usize>())?;

        // The data will be swap removed from its vec,
        // so the back link should also be swap_removed.
        // If other data would be swapped into a new location,
        // we need to reflect that back in the link map
        // so that handles will still be valid.
        self.back_link.swap_remove(index);
        if let Some(handle) = self.back_link.get(index) {
            *self.link_map.get_data_mut(handle).unwrap() = index;
        }

        // finally, swap remove the data and return it
        Some(self.data.swap_remove(index))
    }

    /// Returns a reference to the underlying packed slice of data
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    /// Returns a reference to the underlying packed slice of data handles
    #[inline]
    pub fn get_handles(&self) -> &[Handle<T>] {
        unsafe { std::mem::transmute(self.back_link.as_slice()) }
    }

    /// Returns a mutable reference to the underlying packed slice of data
    #[inline]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    /// Consumes the map, and returns the underlying vec of items
    #[inline]
    pub fn into_vec(self) -> Vec<T> {
        self.data
    }

    /// Returns an iterator over the map.
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        self.data.iter()
    }

    /// Returns an iterator that allows modifying each value.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.data.iter_mut()
    }

    /// Returns a consuming iterator over the map.
    #[inline]
    pub fn into_iter(self) -> IntoIter<T> {
        self.data.into_iter()
    }
}
