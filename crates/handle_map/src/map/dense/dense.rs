use std::{
    marker::PhantomData,
    slice::{Iter, IterMut},
    vec::IntoIter,
};

use crate::{map::sparse::SparseHandleMap, Handle};

use super::{Data, DataMut, DenseAccessMap, DenseInputPredictor, Handles};

/// A storage solution that gives a [`Handle`] to the location of the data.
/// This is optimized for fast access, as the [`Handle`] ensures an array indexing operation.
///
/// This map is ***Dense*** because all the data is stored right next to each other in memory.
/// This means that when accessing with a handle,
/// there needs to be one more level of indirection to get the real location.
/// However, iteration over the map will always be maximally efficient,
/// as the whole map can be iterated over as a tightly packed array.
pub struct DenseHandleMap<T> {
    link_map: SparseHandleMap<usize>,
    data: Vec<(Handle<T>, T)>,
}

impl<T> Default for DenseHandleMap<T> {
    /// Returns a default handle map with a unique id.
    #[inline]
    fn default() -> Self {
        Self {
            link_map: Default::default(),
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
        self.link_map.id()
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
        self.data.push((handle.into_type(), data));
        handle.into_type()
    }

    /// Returns true if `handle` is valid for this map.
    #[inline]
    pub fn contains(&self, handle: Handle<T>) -> bool {
        self.link_map.contains(handle.into_type())
    }

    /// Returns a reference to the data associated with `handle`.
    ///
    /// Returns `None` if the handle is invalid.
    #[inline]
    pub fn get(&self, handle: Handle<T>) -> Option<&T> {
        let index = self.link_map.get(handle.into_type())?;
        Some(&self.data[*index].1)
    }

    /// Returns a mutable reference to the data associated with `handle`.
    ///
    /// Returns `None` if the handle is invalid.
    #[inline]
    pub fn get_mut(&mut self, handle: Handle<T>) -> Option<&mut T> {
        let index = self.link_map.get(handle.into_type())?;
        Some(&mut self.data[*index].1)
    }

    /// Removes and returns the data associated with `handle` from this map.
    ///
    /// Returns `None` if the handle is invalid.
    #[inline]
    pub fn remove(&mut self, handle: Handle<T>) -> Option<T> {
        let index = self.link_map.remove(handle.into_type())?;
        let data = self.data.swap_remove(index).1;
        if let Some((swapped_handle, _)) = self.data.get(index) {
            *self.link_map.get_mut(swapped_handle.into_type()).unwrap() = index;
        }
        Some(data)
    }

    #[inline]
    pub fn iter(&self) -> Iter<(Handle<T>, T)> {
        self.data.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<(Handle<T>, T)> {
        self.data.iter_mut()
    }

    #[inline]
    pub fn into_iter(self) -> IntoIter<(Handle<T>, T)> {
        self.data.into_iter()
    }

    #[inline]
    pub fn data(&self) -> Data<T> {
        Data { iter: self.iter() }
    }

    #[inline]
    pub fn data_mut(&mut self) -> DataMut<T> {
        DataMut {
            iter: self.iter_mut(),
        }
    }

    #[inline]
    pub fn handles(&self) -> Handles<T> {
        Handles { iter: self.iter() }
    }

    pub fn as_access_map(&mut self) -> DenseAccessMap<T> {
        DenseAccessMap {
            link_map: self.link_map.as_access_map(),
            data: &mut self.data,
        }
    }

    pub fn as_insert_predictor(&self) -> DenseInputPredictor<T> {
        DenseInputPredictor {
            link_predictor: self.link_map.as_insert_predictor(),
            _type: PhantomData,
        }
    }

    /// Splits a dense map into two seperate maps that can perform different functions.
    ///
    /// The access map allows for mutable access to the contained data, but no inserting or removing.
    /// While the insert queue allows for insertions, but no access to the data.
    /// The insertion queue will not be reflected until it is merged back in.
    #[inline]
    pub fn split_access_insert(&mut self) -> (DenseAccessMap<T>, DenseInputPredictor<T>) {
        let (sparse_access, link_predictor) = self.link_map.split_access_insert();

        (
            DenseAccessMap {
                link_map: sparse_access,
                data: &mut self.data,
            },
            DenseInputPredictor {
                link_predictor,
                _type: PhantomData,
            },
        )
    }
}
