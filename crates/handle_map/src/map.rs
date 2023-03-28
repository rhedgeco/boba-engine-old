use std::{
    collections::VecDeque,
    sync::atomic::{AtomicU16, Ordering},
};

use crate::Handle;

struct HandleEntry<T> {
    handle: Handle<T>,
    data: Option<T>,
}

impl<T> HandleEntry<T> {
    pub fn new(handle: Handle<T>, data: T) -> Self {
        Self {
            handle,
            data: Some(data),
        }
    }
}

/// A sparse storage solution that gives a [`Handle`] to the location of the data.
/// This is optimized for fast access, as the [`Handle`] ensures an array indexing operation.
///
/// This map is ***Sparse*** because when an item is removed,
/// the indices of the map are kept in place, and a hole is left where the item used to be.
/// That hole will then be filled again when new items are inserted into the map.
/// However, if many items are removed and there are no inserts afterwards,
/// the map will take up as much space as the max amount of items that used to be inside.
pub struct SparseHandleMap<T> {
    id: u16,
    data: Vec<HandleEntry<T>>,
    open_data: VecDeque<usize>,
}

impl<T> Default for SparseHandleMap<T> {
    /// Returns a default handle map with a unique id
    #[inline]
    fn default() -> Self {
        static ID_GEN: AtomicU16 = AtomicU16::new(0);

        Self {
            id: ID_GEN.fetch_add(1, Ordering::Relaxed),
            data: Default::default(),
            open_data: Default::default(),
        }
    }
}

impl<T> SparseHandleMap<T> {
    /// Returns a default handle map with a unique id
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the id for this manager
    #[inline]
    pub fn id(&self) -> u16 {
        self.id
    }

    /// Inserts `data` into the map and returns a [`Handle`] to its location
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
                self.data.push(HandleEntry::new(handle.clone(), data));
                handle
            }
        }
    }

    /// Returns true if `handle` is valid for this map
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
}
