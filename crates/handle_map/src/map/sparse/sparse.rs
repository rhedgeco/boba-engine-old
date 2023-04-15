use std::collections::VecDeque;

use crate::{map::HandleMapId, Handle};

use super::{IntoIter, Iter, IterMut, SparseAccessMap, SparseInsertPredictor};

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
    data: Vec<(Handle<T>, Option<T>)>,
    open_data: VecDeque<Handle<T>>,
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
            Some(handle) => {
                let mut entry = &mut self.data[handle.uindex()];
                entry.1 = Some(data);
                handle
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
                self.data.push((handle, Some(data)));
                handle
            }
        }
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

    /// Removes and returns the data for `handle`.
    ///
    /// Returns `None` if `handle` is invalid for this map.
    #[inline]
    pub fn remove(&mut self, handle: Handle<T>) -> Option<T> {
        match self.data.get_mut(handle.uindex()) {
            Some(entry) if entry.0 == handle => {
                let (index, gen, meta) = entry.0.into_raw_parts();
                entry.0 = Handle::from_raw_parts(index, gen.wrapping_add(1), meta);
                let data = std::mem::replace(&mut entry.1, None);
                self.open_data.push_back(entry.0);
                data
            }
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

    /// Returns a consuming iterator over the map.
    #[inline]
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            iter: self.data.into_iter(),
        }
    }

    pub fn as_access_map(&mut self) -> SparseAccessMap<T> {
        SparseAccessMap {
            id: self.id,
            data: &mut self.data,
        }
    }

    pub fn as_insert_predictor(&self) -> SparseInsertPredictor<T> {
        SparseInsertPredictor {
            id: self.id,
            open_data: &self.open_data,
            next_push_index: self.data.len() as u32,
        }
    }

    /// Splits a sparse map into two seperate maps that can perform different functions.
    ///
    /// The access map allows for mutable access to the contained data, but no inserting or removing.
    /// While the insert queue allows for insertions, but no access to the data.
    /// The insertion queue will not be reflected until it is merged back in.
    #[inline]
    pub fn split_access_insert(&mut self) -> (SparseAccessMap<T>, SparseInsertPredictor<T>) {
        let next_push_index = self.data.len() as u32;

        (
            SparseAccessMap {
                id: self.id,
                data: &mut self.data,
            },
            SparseInsertPredictor {
                id: self.id,
                open_data: &self.open_data,
                next_push_index,
            },
        )
    }
}
