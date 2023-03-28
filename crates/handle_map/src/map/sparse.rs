use std::collections::VecDeque;

use crate::{map::HandleMapId, Handle};

struct SparseEntry<T> {
    handle: Handle<T>,
    data: Option<T>,
}

impl<T> SparseEntry<T> {
    #[inline]
    pub fn new(handle: Handle<T>, data: T) -> Self {
        Self {
            handle,
            data: Some(data),
        }
    }
}

/// A storage solution that gives a [`Handle`] to the location of the data.
/// This is optimized for fast access, as the [`Handle`] ensures an array indexing operation.
///
/// This map is ***Sparse*** because when an item is removed,
/// the indices of the map are kept in place, and a hole is left where the item used to be.
/// That hole will then be filled again when new items are inserted into the map.
/// However, if many items are removed and there are no inserts afterwards,
/// the map will take up as much space as the max amount of items that used to be inside.
///
/// Since the map uses a [`Handle`] for indexing, the max length of the map is limited to `u32::MAX`.
pub struct SparseHandleMap<T> {
    id: u16,
    data: Vec<SparseEntry<T>>,
    open_data: VecDeque<usize>,
}

impl<T> Default for SparseHandleMap<T> {
    /// Returns a default handle map with a unique id.
    #[inline]
    fn default() -> Self {
        Self {
            id: HandleMapId::generate(),
            data: Default::default(),
            open_data: Default::default(),
        }
    }
}

impl<T> SparseHandleMap<T> {
    /// Returns a default handle map with a unique id.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the id for this manager.
    #[inline]
    pub fn id(&self) -> u16 {
        self.id
    }

    /// Returns the length of the map.
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len() - self.open_data.len()
    }

    /// Returns `true` if the map is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.len() == self.open_data.len()
    }

    /// Inserts `data` into the map and returns a [`Handle`] to its location.
    #[inline]
    pub fn insert(&mut self, data: T) -> Handle<T> {
        match self.open_data.pop_front() {
            Some(index) => {
                let entry = &self.data[index];
                entry.handle.clone()
            }
            None => {
                let index = self.data.len();
                if index > u32::MAX as usize {
                    // even though the index may be a usize, it gets stored as a u32.
                    // so if the length of the backing vec increases past u32::MAX,
                    // then a capacity overflow panic must be thrown.
                    panic!("SparseHandleMap capacity overflow");
                }

                let handle = Handle::from_raw_parts(index as u32, 0, self.id);
                self.data.push(SparseEntry::new(handle.clone(), data));
                handle
            }
        }
    }

    /// Returns true if `handle` is valid for this map.
    #[inline]
    pub fn contains(&self, handle: &Handle<T>) -> bool {
        match self.data.get(handle.uindex()) {
            Some(other) => &other.handle == handle,
            None => false,
        }
    }

    /// Returns a reference to the data associated with `handle`.
    ///
    /// Returns `None` if `handle` is invalid for this map.
    #[inline]
    pub fn get_data(&self, handle: &Handle<T>) -> Option<&T> {
        match self.data.get(handle.uindex()) {
            Some(entry) if &entry.handle == handle => entry.data.as_ref(),
            _ => None,
        }
    }

    /// Returns a mutable reference to the data associated with `handle`.
    ///
    /// Returns `None` if `handle` is invalid for this map.
    #[inline]
    pub fn get_data_mut(&mut self, handle: &Handle<T>) -> Option<&mut T> {
        match self.data.get_mut(handle.uindex()) {
            Some(entry) if &entry.handle == handle => entry.data.as_mut(),
            _ => None,
        }
    }

    /// Removes and returns the data for `handle`.
    ///
    /// Returns `None` if `handle` is invalid for this map.
    #[inline]
    pub fn remove(&mut self, handle: &Handle<T>) -> Option<T> {
        match self.data.get_mut(handle.uindex()) {
            Some(entry) if &entry.handle == handle => {
                let (index, gen, meta) = entry.handle.clone().into_raw_parts();
                entry.handle = Handle::from_raw_parts(index, gen.wrapping_add(1), meta);
                let data = std::mem::replace(&mut entry.data, None);
                self.open_data.push_back(handle.uindex());
                data
            }
            _ => None,
        }
    }

    /// Returns an iterator over the map.
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            inner: self.data.iter(),
        }
    }

    /// Returns an iterator that allows modifying each value.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            inner: self.data.iter_mut(),
        }
    }

    /// Returns a consuming iterator over the map.
    #[inline]
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            inner: self.data.into_iter(),
        }
    }
}

pub struct Iter<'a, T> {
    inner: std::slice::Iter<'a, SparseEntry<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(data) = &self.inner.next()?.data {
                return Some(data);
            }
        }
    }
}

pub struct IterMut<'a, T> {
    inner: std::slice::IterMut<'a, SparseEntry<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(data) = &mut self.inner.next()?.data {
                return Some(data);
            }
        }
    }
}

pub struct IntoIter<T> {
    inner: std::vec::IntoIter<SparseEntry<T>>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(data) = self.inner.next()?.data {
                return Some(data);
            }
        }
    }
}
