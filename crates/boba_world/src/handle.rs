use std::{cell::Cell, marker::PhantomData, rc::Rc};

use thiserror::Error;

pub type HandleResult<T> = Result<T, InvalidHandleError>;

/// The inner representation of a [`Handle`]
struct InnerHandle {
    valid: Cell<bool>,
    index: Cell<usize>,
}

/// A handle to some data `T` inside a [`HandleMap`]
pub struct Handle<T, const ID: usize> {
    inner: Rc<InnerHandle>,
    _type: PhantomData<*const T>,
}

impl<T, const ID: usize> Clone for Handle<T, ID> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _type: PhantomData,
        }
    }
}

impl<T, const ID: usize> Handle<T, ID> {
    /// Creates a new handle at `index`
    fn new(index: usize) -> Self {
        let inner = InnerHandle {
            valid: Cell::new(true),
            index: Cell::new(index),
        };

        Self {
            inner: Rc::new(inner),
            _type: PhantomData,
        }
    }

    /// Invalidates all handles to this data
    fn invalidate(&self) {
        self.inner.valid.set(false)
    }

    /// Gets the index for this handle
    fn index(&self) -> usize {
        self.inner.index.get()
    }

    /// Overrides the index for all handles to this data
    fn reset_index(&self, index: usize) {
        self.inner.index.set(index)
    }

    /// Checks if this handle's data id still valid in its [`HandleMap`]
    pub fn is_valid(&self) -> bool {
        self.inner.valid.get()
    }
}

/// Structure used to manage entries in a [`HandleMap`]
struct HandleMapItem<T, const ID: usize> {
    handle: Handle<T, ID>,
    item: T,
}

impl<T, const ID: usize> HandleMapItem<T, ID> {
    /// Creates a new map item with `handle` and `item`
    fn new(handle: Handle<T, ID>, item: T) -> Self {
        Self { handle, item }
    }
}

/// Error type for invalid access of [`HandleMap`]
#[derive(Debug, Error)]
#[error("Tried to access data in HandleMap using invalid handle")]
pub struct InvalidHandleError;

/// A collection of `T` that produces [`Handle`] links
pub struct HandleMap<T, const ID: usize> {
    items: Vec<HandleMapItem<T, ID>>,
    _type: PhantomData<T>,
}

impl<T, const ID: usize> Default for HandleMap<T, ID> {
    fn default() -> Self {
        Self {
            items: Default::default(),
            _type: PhantomData,
        }
    }
}

impl<T, const ID: usize> HandleMap<T, ID> {
    /// Creates a new `HandleMap`
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts `item` into the map and returns a [`Handle`] to its location
    pub fn insert(&mut self, item: T) -> Handle<T, ID> {
        let index = self.items.len();
        let handle = Handle::new(index);
        let handle_item = HandleMapItem::new(handle.clone(), item);
        self.items.push(handle_item);
        handle
    }

    /// Gets a reference to the item in this map that is associated with `handle`
    ///
    /// ## Warning
    /// Trying to use a handle on a map that the handle did not come from is ***undefined behaviour***
    /// and may sometimes result in a panic
    pub fn get(&self, handle: &Handle<T, ID>) -> HandleResult<&T> {
        match handle.is_valid() {
            false => Err(InvalidHandleError),
            true => Ok(&self.items[handle.index()].item),
        }
    }

    /// Gets a mutable reference to the item in this map that is associated with `handle`
    ///
    /// ## Warning
    /// Trying to use a handle on a map that the handle did not come from is ***undefined behaviour***
    /// and may sometimes result in a panic
    pub fn get_mut(&mut self, handle: &Handle<T, ID>) -> HandleResult<&mut T> {
        match handle.is_valid() {
            false => Err(InvalidHandleError),
            true => Ok(&mut self.items[handle.index()].item),
        }
    }

    /// Removes the item in this map that is associated with `handle`, and then invalidates the handle.
    ///
    /// ## Warning
    /// Trying to use a handle on a map that the handle did not come from is ***undefined behaviour***
    /// and may sometimes result in a panic
    pub fn remove(&mut self, handle: &Handle<T, ID>) -> HandleResult<T> {
        match handle.is_valid() {
            false => Err(InvalidHandleError),
            true => {
                let index = handle.index();
                let item = self.items.swap_remove(index);
                self.items[index].handle.reset_index(index);
                handle.invalidate();
                Ok(item.item)
            }
        }
    }

    /// Invalidates every handle and drops every item from the map
    pub fn clear(&mut self) {
        for item in self.items.iter() {
            item.handle.invalidate();
        }
        self.items.clear();
    }
}
