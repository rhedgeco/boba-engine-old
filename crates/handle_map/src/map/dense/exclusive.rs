use std::slice::IterMut;

use crate::Handle;

use super::DenseAccessMap;

pub struct ExclusiveAccessStream<'a, T> {
    iter: IterMut<'a, (Handle<T>, T)>,
    access: &'a mut DenseAccessMap<'a, T>,
}

impl<'a, T> ExclusiveAccessStream<'a, T> {
    pub(super) fn new(access: &'a mut DenseAccessMap<'a, T>) -> Self {
        let access_ptr = access as *mut DenseAccessMap<T>;
        Self {
            iter: access.iter_mut(),
            access: unsafe { &mut *access_ptr },
        }
    }

    pub fn next<'data>(
        &'data mut self,
    ) -> Option<(&'data mut T, Handle<T>, ExclusiveAccessMap<'a, 'data, T>)> {
        let (handle, data) = self.iter.next()?;
        let exclusive_access = ExclusiveAccessMap {
            exclude: *handle,
            access: self.access,
        };
        Some((data, *handle, exclusive_access))
    }
}

pub struct ExclusiveAccessMap<'a, 'data, T> {
    exclude: Handle<T>,
    access: &'data mut DenseAccessMap<'a, T>,
}

impl<'a, 'data, T> ExclusiveAccessMap<'a, 'data, T> {
    pub fn new(
        handle: Handle<T>,
        access: &'data mut DenseAccessMap<'a, T>,
    ) -> Option<(&'data mut T, Handle<T>, Self)> {
        let access_ptr = access as *mut DenseAccessMap<'a, T>;
        let data = access.get_mut(handle)?;
        let map = Self {
            exclude: handle,
            access: unsafe { &mut *access_ptr },
        };
        Some((data, handle, map))
    }

    /// Returns the undelying id for this map.
    #[inline]
    pub fn id(&self) -> u16 {
        self.access.id()
    }

    /// Returns the number of items in the map.
    #[inline]
    pub fn len(&self) -> usize {
        self.access.len()
    }

    /// Returns `true` if the map is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.access.is_empty()
    }

    /// Returns true if `handle` is valid for this map.
    #[inline]
    pub fn contains(&self, handle: Handle<T>) -> bool {
        self.access.contains(handle)
    }

    /// Returns a reference to the data associated with `handle`.
    ///
    /// Returns `None` if the handle is invalid, or the handle has been excluded.
    pub fn get(&self, handle: Handle<T>) -> Option<&T> {
        if self.exclude == handle {
            return None;
        }

        self.access.get(handle)
    }

    /// Returns a mutable reference to the data associated with `handle`.
    ///
    /// Returns `None` if the handle is invalid, or the handle has been excluded.
    pub fn get_mut(&mut self, handle: Handle<T>) -> Option<&mut T> {
        if self.exclude == handle {
            return None;
        }

        self.access.get_mut(handle)
    }
}
