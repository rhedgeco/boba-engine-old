use std::collections::VecDeque;

use crate::Handle;

use super::{ExclusiveAccessMap, ExclusiveAccessStream, Iter, IterMut};

pub struct SparseAccessMap<'a, T> {
    pub(super) id: u16,
    pub(super) data: &'a mut Vec<(Handle<T>, Option<T>)>,
}

impl<'a, T> SparseAccessMap<'a, T> {
    /// Returns the id for this manager.
    #[inline]
    pub fn id(&self) -> u16 {
        self.id
    }

    /// Returns true if `handle` is valid for this map.
    #[inline]
    pub fn contains(&self, handle: Handle<T>) -> bool {
        match self.data.get(handle.uindex()) {
            Some(other) => other.0 == handle,
            None => false,
        }
    }

    /// Returns a reference to the data associated with `handle`.
    ///
    /// Returns `None` if `handle` is invalid for this map.
    #[inline]
    pub fn get(&self, handle: Handle<T>) -> Option<&T> {
        match self.data.get(handle.uindex()) {
            Some(entry) if entry.0 == handle => entry.1.as_ref(),
            _ => None,
        }
    }

    /// Returns a mutable reference to the data associated with `handle`.
    ///
    /// Returns `None` if `handle` is invalid for this map.
    #[inline]
    pub fn get_mut(&mut self, handle: Handle<T>) -> Option<&mut T> {
        match self.data.get_mut(handle.uindex()) {
            Some(entry) if entry.0 == handle => entry.1.as_mut(),
            _ => None,
        }
    }

    /// Returns an iterator over the map.
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            iter: self.data.iter(),
        }
    }

    /// Returns an iterator that allows modifying each value.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            iter: self.data.iter_mut(),
        }
    }

    pub fn exclusive_access<'data>(
        &'a mut self,
        handle: Handle<T>,
    ) -> Option<(&'a mut T, Handle<T>, ExclusiveAccessMap<T>)> {
        ExclusiveAccessMap::new(handle, self)
    }

    #[inline]
    pub fn stream(&'a mut self) -> ExclusiveAccessStream<'a, T> {
        ExclusiveAccessStream::new(self)
    }
}

pub struct SparseInsertPredictor<'a, T> {
    pub(super) id: u16,
    pub(super) open_data: &'a VecDeque<Handle<T>>,
    pub(super) next_push_index: u32,
}

impl<'a, T> SparseInsertPredictor<'a, T> {
    pub fn predict(&self, num: u32) -> Handle<T> {
        match self.open_data.get(num as usize) {
            Some(handle) => *handle,
            None => {
                let index = self.next_push_index + num - self.open_data.len() as u32;
                Handle::from_raw_parts(index, 0, self.id)
            }
        }
    }
}
