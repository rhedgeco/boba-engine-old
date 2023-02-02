use std::{cell::Cell, hash::Hash, marker::PhantomData, rc::Rc};

use thiserror::Error;

use crate::BobaId;

pub type HandleResult<T> = Result<T, InvalidHandleError>;

/// The inner representation of a [`Handle`]
struct InnerHandle {
    id: BobaId,
    map_id: BobaId,
    valid: Cell<bool>,
    index: Cell<usize>,
}

/// A handle to some data `T` inside a [`HandleMap`]
pub struct Handle<T> {
    inner: Rc<InnerHandle>,
    _type: PhantomData<*const T>,
}

impl<T> Eq for Handle<T> {}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl<T> Hash for Handle<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _type: PhantomData,
        }
    }
}

impl<T> Handle<T> {
    /// Creates a new handle at `index`
    fn new(index: usize, map_id: BobaId) -> Self {
        let inner = InnerHandle {
            map_id,
            id: BobaId::new(),
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

    /// Gets the [`BobaId`] for this handle
    pub fn id(&self) -> &BobaId {
        &self.inner.id
    }

    /// Gets the [`BobaId`] associated with this handle's [`HandleMap`]
    pub fn map_id(&self) -> &BobaId {
        &self.inner.map_id
    }

    /// Checks if this handle's data id still valid in its [`HandleMap`]
    pub fn is_valid(&self) -> bool {
        self.inner.valid.get()
    }
}

/// Structure used to manage entries in a [`HandleMap`]
struct HandleMapItem<T> {
    handle: Handle<T>,
    item: T,
}

impl<T> HandleMapItem<T> {
    /// Creates a new map item with `handle` and `item`
    fn new(handle: Handle<T>, item: T) -> Self {
        Self { handle, item }
    }
}

/// Error type for invalid access of [`HandleMap`]
#[derive(Debug, Error)]
#[error("Tried to access data in HandleMap using invalid Handle")]
pub struct InvalidHandleError;

/// A collection of `T` that produces [`Handle`] links
pub struct HandleMap<T> {
    id: BobaId,
    items: Vec<HandleMapItem<T>>,
    _type: PhantomData<T>,
}

impl<T> Default for HandleMap<T> {
    fn default() -> Self {
        Self {
            id: BobaId::new(),
            items: Default::default(),
            _type: PhantomData,
        }
    }
}

impl<T> HandleMap<T> {
    /// Creates a new `HandleMap`
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the [`BobaId`] for this map
    pub fn id(&self) -> &BobaId {
        &self.id
    }

    /// Returns the number of elements in the map
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Inserts `item` into the map and returns a [`Handle`] to its location
    pub fn insert(&mut self, item: T) -> Handle<T> {
        let index = self.items.len();
        let handle = Handle::new(index, self.id);
        let handle_item = HandleMapItem::new(handle.clone(), item);
        self.items.push(handle_item);
        handle
    }

    /// Gets a reference to the item in this map that is associated with `handle`
    ///
    /// # Panics
    /// This will panic if `handle` was created by a different map
    pub fn get(&self, handle: &Handle<T>) -> HandleResult<&T> {
        self.validate_id_or_panic(handle.id());

        match handle.is_valid() {
            false => Err(InvalidHandleError),
            // SAFETY: Checks for map id and handle validity are performed before this
            true => Ok(unsafe { self.get_unchecked(handle) }),
        }
    }

    /// Gets a reference to the item in this map that is associated with `handle`
    ///
    /// Potentially accessing random or uninitialized memory if any pre-checks are avoided.
    /// ## How to use safely ⚠️
    /// The following checks are not performed when using this method, and should be done externally beforehand.
    /// - Check if the [`BobaId`] of the map and the `map_id` of the handle are equivilant.
    /// - Check if the [`Handle`] is still valid using `is_valid`.
    pub unsafe fn get_unchecked(&self, handle: &Handle<T>) -> &T {
        &self.items.get_unchecked(handle.index()).item
    }

    /// Gets a mutable reference to the item in this map that is associated with `handle`
    ///
    /// # Panics
    /// This will panic if `handle` was created by a different map
    pub fn get_mut(&mut self, handle: &Handle<T>) -> HandleResult<&mut T> {
        self.validate_id_or_panic(handle.id());

        match handle.is_valid() {
            false => Err(InvalidHandleError),
            // SAFETY: Checks for map id and handle validity are performed before this
            true => Ok(unsafe { self.get_unchecked_mut(handle) }),
        }
    }

    /// Gets a mutable reference to the item in this map that is associated with `handle`
    ///
    /// Potentially accessing random or uninitialized memory if any pre-checks are avoided.
    /// ## How to use safely ⚠️
    /// The following checks are not performed when using this method, and should be done externally beforehand.
    /// - Check if the [`BobaId`] of the map and the `map_id` of the handle are equivilant.
    /// - Check if the [`Handle`] is still valid using `is_valid`.
    pub unsafe fn get_unchecked_mut(&mut self, handle: &Handle<T>) -> &mut T {
        &mut self.items.get_unchecked_mut(handle.index()).item
    }

    /// Removes the item in this map that is associated with `handle`, and then invalidates the handle.
    ///
    /// # Panics
    /// This will panic if `handle` was created by a different map
    pub fn remove(&mut self, handle: &Handle<T>) -> HandleResult<Option<T>> {
        self.validate_id_or_panic(handle.id());

        match handle.is_valid() {
            false => Err(InvalidHandleError),
            // SAFETY: Checks for map id and handle validity are performed before this
            true => Ok(unsafe { self.remove_unchecked(handle) }),
        }
    }

    /// Removes the item in this map that is associated with `handle`, and then invalidates the handle.
    /// Returns `None` if the map is empty
    ///
    /// Potentially accessing random or uninitialized memory if any pre-checks are avoided.
    ///
    /// ## How to use safely ⚠️
    /// The following checks are not performed when using this method, and should be done externally beforehand.
    /// - Check if the [`BobaId`] of the map and the `map_id` of the handle are equivilant.
    /// - Check if the [`Handle`] is still valid using `is_valid`.
    pub unsafe fn remove_unchecked(&mut self, handle: &Handle<T>) -> Option<T> {
        handle.invalidate();
        let length = self.len();
        match length {
            // none if zero
            0 => None,
            // if only one item, set length to zero and read item
            1 => {
                self.items.set_len(0);
                Some(std::ptr::read(self.items.as_ptr()).item)
            }
            // if many items, swap item with last item
            length => {
                let hole: *mut HandleMapItem<T> = self.items.get_unchecked_mut(handle.index());
                self.items.set_len(length - 1);
                let last = std::ptr::read(self.items.as_ptr().add(self.items.len()));
                last.handle.reset_index(handle.index());
                Some(std::ptr::replace(hole, last).item)
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

    /// Validates a id matches this map.
    /// If it doesn't match the system panics.
    fn validate_id_or_panic(&self, id: &BobaId) {
        if id != self.id() {
            panic!("Tried using a handle to access data, but using the wrong map.")
        }
    }
}
