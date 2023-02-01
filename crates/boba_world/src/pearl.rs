use std::{
    any::Any,
    hash::Hash,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::{Handle, HandleMap, HandleResult};

/// A handle to a [`Pearl`] inside a [`PearlMap`]
///
/// This is essentially a simple wrapper around a [`Handle`]
pub struct PearlHandle<T, const ID: usize> {
    inner: Handle<Box<dyn Any>, ID>,
    _type: PhantomData<*const T>,
}

impl<T, const ID: usize> Eq for PearlHandle<T, ID> {}

impl<T, const ID: usize> PartialEq for PearlHandle<T, ID> {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl<T, const ID: usize> Hash for PearlHandle<T, ID> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

impl<T, const ID: usize> Deref for PearlHandle<T, ID> {
    type Target = Handle<Box<dyn Any>, ID>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T, const ID: usize> Clone for PearlHandle<T, ID> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _type: PhantomData,
        }
    }
}

/// A collection of [`Pearl`] objects that produces [`PearlHandle`] links
#[derive(Default)]
pub struct PearlMap<const ID: usize> {
    pearl_map: HandleMap<Box<dyn Any>, ID>,
}

impl<const ID: usize> PearlMap<ID> {
    /// Creates a new map
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts `pearl` into the map and returns a [`PearlHandle`] to its location
    pub fn insert<T: 'static>(&mut self, pearl: Pearl<T>) -> PearlHandle<T, ID> {
        let inner = self.pearl_map.insert(Box::new(pearl));
        PearlHandle {
            inner,
            _type: PhantomData,
        }
    }

    /// Gets a reference to a [`Pearl`] in this map that is associated with `handle`
    ///
    /// ## Warning
    /// Trying to use a handle on a map that the handle did not come from is ***undefined behaviour***
    /// and may sometimes result in a panic
    pub fn get<T: 'static>(&self, handle: &PearlHandle<T, ID>) -> HandleResult<&Pearl<T>> {
        let any = self.pearl_map.get(&handle.inner)?;
        Ok(any.downcast_ref::<Pearl<T>>().unwrap())
    }

    /// Gets a mutable reference to a [`Pearl`] in this map that is associated with `handle`
    ///
    /// ## Warning
    /// Trying to use a handle on a map that the handle did not come from is ***undefined behaviour***
    /// and may sometimes result in a panic
    pub fn get_mut<T: 'static>(
        &mut self,
        handle: &PearlHandle<T, ID>,
    ) -> HandleResult<&mut Pearl<T>> {
        let any = self.pearl_map.get_mut(&handle.inner)?;
        Ok(any.downcast_mut::<Pearl<T>>().unwrap())
    }

    /// Removes a [`Pearl`] from this map that is associated with `handle`, and then invalidates the handle.
    ///
    /// ## Warning
    /// Trying to use a handle on a map that the handle did not come from is ***undefined behaviour***
    /// and may sometimes result in a panic
    pub fn remove<T: 'static>(
        &mut self,
        handle: &PearlHandle<T, ID>,
    ) -> HandleResult<Option<Pearl<T>>> {
        let any = self.pearl_map.remove(&handle.inner)?;
        let Some(any) = any else { return Ok(None); };
        Ok(Some(*any.downcast::<Pearl<T>>().unwrap()))
    }

    /// Invalidates every handle and drops every [`Pearl`] from the map
    pub fn clear(&mut self) {
        self.pearl_map.clear()
    }
}

/// A wrapper around `T` to allow insertion into a [`PearlMap`]
pub struct Pearl<T> {
    data: T,
}

impl<T> Deref for Pearl<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Pearl<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> Pearl<T> {
    /// Creates a new pearl containing `data`
    pub fn new(data: T) -> Self {
        Self { data }
    }

    /// Consumes the pearl and returns the underlying data
    pub fn into_inner(self) -> T {
        self.data
    }
}
