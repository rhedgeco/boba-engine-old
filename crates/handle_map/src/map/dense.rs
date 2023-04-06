use std::slice;

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
    data: Vec<(Handle<usize>, T)>,
}

impl<T> Default for DenseHandleMap<T> {
    /// Returns a default handle map with a unique id.
    #[inline]
    fn default() -> Self {
        Self {
            id: HandleMapId::generate(),
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
        self.data.push((handle, data));
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
        Some(&self.data[*index].1)
    }

    /// Returns a mutable reference to the data associated with `handle`.
    ///
    /// Returns `None` if the handle is invalid.
    #[inline]
    pub fn get_data_mut(&mut self, handle: &Handle<T>) -> Option<&mut T> {
        let index = self.link_map.get_data(handle.as_type::<usize>())?;
        Some(&mut self.data[*index].1)
    }

    /// Removes and returns the data associated with `handle` from this map.
    ///
    /// Returns `None` if the handle is invalid.
    #[inline]
    pub fn remove(&mut self, handle: &Handle<T>) -> Option<T> {
        let index = self.link_map.remove(handle.as_type::<usize>())?;
        let data = self.data.swap_remove(index).1;
        if let Some((swapped_handle, _)) = self.data.get(index) {
            *self.link_map.get_data_mut(swapped_handle).unwrap() = index;
        }
        Some(data)
    }

    #[inline]
    pub fn iter(&self) -> Iter<T> {
        Iter {
            iter: self.data.iter(),
        }
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            iter: self.data.iter_mut(),
        }
    }

    #[inline]
    pub fn data(&self) -> Data<T> {
        Data {
            iter: self.data.iter(),
        }
    }

    #[inline]
    pub fn data_mut(&mut self) -> DataMut<T> {
        DataMut {
            iter: self.data.iter_mut(),
        }
    }

    #[inline]
    pub fn handles(&self) -> Handles<T> {
        Handles {
            iter: self.data.iter(),
        }
    }
}

pub struct Iter<'a, T> {
    iter: slice::Iter<'a, (Handle<usize>, T)>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (&'a Handle<T>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let (handle, data) = self.iter.next()?;
        Some((handle.as_type(), data))
    }
}

pub struct IterMut<'a, T> {
    iter: slice::IterMut<'a, (Handle<usize>, T)>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = (&'a Handle<T>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        let (handle, data) = self.iter.next()?;
        Some((handle.as_type(), data))
    }
}

pub struct Data<'a, T> {
    iter: slice::Iter<'a, (Handle<usize>, T)>,
}

impl<'a, T> Iterator for Data<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let (_, data) = self.iter.next()?;
        Some(data)
    }
}

pub struct DataMut<'a, T> {
    iter: slice::IterMut<'a, (Handle<usize>, T)>,
}

impl<'a, T> Iterator for DataMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let (_, data) = self.iter.next()?;
        Some(data)
    }
}

pub struct Handles<'a, T> {
    iter: slice::Iter<'a, (Handle<usize>, T)>,
}

impl<'a, T> Iterator for Handles<'a, T> {
    type Item = &'a Handle<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let (handle, _) = self.iter.next()?;
        Some(handle.as_type())
    }
}
