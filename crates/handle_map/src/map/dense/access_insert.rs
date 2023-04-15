use std::{
    marker::PhantomData,
    slice::{Iter, IterMut},
};

use crate::{
    map::sparse::{SparseAccessMap, SparseInsertPredictor},
    Handle,
};

use super::{Data, DataMut, ExclusiveAccessMap, ExclusiveAccessStream, Handles};

pub struct DenseAccessMap<'a, T> {
    pub(super) link_map: SparseAccessMap<'a, usize>,
    pub(super) data: &'a mut Vec<(Handle<T>, T)>,
}

impl<'a, T> DenseAccessMap<'a, T> {
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

    #[inline]
    pub fn iter(&self) -> Iter<(Handle<T>, T)> {
        self.data.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<(Handle<T>, T)> {
        self.data.iter_mut()
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

    #[inline]
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

pub struct DenseInputPredictor<'a, T> {
    pub(super) link_predictor: SparseInsertPredictor<'a, usize>,
    pub(super) _type: PhantomData<*const T>,
}

impl<'a, T> DenseInputPredictor<'a, T> {
    pub fn predict(&self, num: u32) -> Handle<T> {
        self.link_predictor.predict(num).into_type()
    }
}
