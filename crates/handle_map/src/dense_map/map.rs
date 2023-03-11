use std::{
    collections::VecDeque,
    sync::atomic::{AtomicU16, Ordering},
};

use crate::{AnyHandle, Handle};

struct HandleLink {
    handle: AnyHandle,
    entry: u32,
}

impl HandleLink {
    #[inline]
    fn new(handle: AnyHandle, entry: u32) -> Self {
        Self { handle, entry }
    }
}

/// A dense map that is indexed by [`Handle`] objects
///
/// When items are removed, the map is modified so that the removed space is packed with existing items.
/// To do this and keep handles alive, it uses a special linking layer that matches indices to their item location.
/// This keeps all items as the most densly packed array as possible providing fast iteration.
pub struct DenseHandleMap<T> {
    id: u16,
    link: Vec<HandleLink>,
    free_links: VecDeque<u32>,
    back_links: Vec<u32>,
    items: Vec<T>,
}

impl<T> Default for DenseHandleMap<T> {
    /// Creates a new `DenseHandleMap` with type `T`
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> DenseHandleMap<T> {
    /// Creates a new `DenseHandleMap` with type `T`
    pub fn new() -> Self {
        static ID: AtomicU16 = AtomicU16::new(0);
        Self {
            id: ID.fetch_add(1, Ordering::Relaxed),
            link: Vec::new(),
            free_links: VecDeque::new(),
            back_links: Vec::new(),
            items: Vec::new(),
        }
    }

    /// Returns a reference to the item associated with `handle`
    ///
    /// If the handle is invalid, returns `None`
    #[inline]
    pub fn get(&self, handle: &Handle<T>) -> Option<&T> {
        let item_index = self.get_and_validate_index(handle)?;
        self.items.get(item_index as usize)
    }

    /// Returns a mutable reference to the item associated with `handle`
    ///
    /// If the handle is invalid, returns `None`
    #[inline]
    pub fn get_mut(&mut self, handle: &Handle<T>) -> Option<&mut T> {
        let item_index = self.get_and_validate_index(handle)?;
        self.items.get_mut(item_index as usize)
    }

    /// Returns a reference to the underlying slice used in the map
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.items
    }

    /// Returns a mutable reference to the underlying slice used in the map
    #[inline]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        &mut self.items
    }

    /// Inserts an item into the map, returning a [`Handle`] to its location
    pub fn insert(&mut self, item: T) -> Handle<T> {
        // add item to list, and panic if overflow
        let new_index = self.items.len();
        if new_index > u32::MAX as usize {
            panic!("capacity overflow");
        }
        let new_index = new_index as u32;
        self.items.push(item);

        // set up links and get handle
        let handle = match self.free_links.pop_front() {
            Some(link_index) => {
                self.link[link_index as usize].entry = new_index;
                let any_handle = self.link[link_index as usize].handle;
                Handle::<T>::from_raw(any_handle.into_raw())
            }
            None => {
                let new_handle_index = self.link.len() as u32;
                let new_handle = Handle::<T>::from_raw_parts(new_handle_index, 0, self.id);
                self.link.push(HandleLink::new(new_handle.any(), new_index));
                new_handle
            }
        };

        // add back link and return item
        self.back_links.push(handle.index());
        handle
    }

    /// Removes the item from the map that is associated with [`Handle`] and returns it
    ///
    /// Returns `None` if the handle is invalid
    pub fn remove(&mut self, handle: &Handle<T>) -> Option<T> {
        // validate the index
        let item_index = self.get_and_validate_index(handle)?;

        // remove item at index and swap it with the last item to maintain tight packing
        let item = self.items.swap_remove(item_index as usize);

        // swap remove items in the backlink array
        let drop_link_index = self.back_links.swap_remove(item_index as usize);
        let swap_link_index = self.back_links[item_index as usize];

        // increment the generation for the removed item and add it to the free list
        let mut drop_handle = self.link[drop_link_index as usize].handle;
        drop_handle.modify(|i, g, m| (i, g.wrapping_add(1), m));
        self.free_links.push_back(drop_link_index as u32);

        // fix the entry for the swapped item
        self.link[swap_link_index as usize].entry = item_index;

        // return the item
        Some(item)
    }

    #[inline]
    fn get_and_validate_index(&self, handle: &Handle<T>) -> Option<u32> {
        let link = self.link.get(handle.uindex())?;
        if link.handle != handle.any() {
            return None;
        }

        Some(link.entry)
    }
}

#[cfg(test)]
mod tests {
    use super::DenseHandleMap;

    struct TestItem {
        v1: u32,
        v2: u64,
    }

    impl TestItem {
        fn new(v1: u32, v2: u64) -> Self {
            Self { v1, v2 }
        }
    }

    #[test]
    fn insert_and_get() {
        let mut map = DenseHandleMap::<TestItem>::new();
        let handle = map.insert(TestItem::new(420, 69));
        let item_ref = map.get(&handle).unwrap();
        assert!(item_ref.v1 == 420);
        assert!(item_ref.v2 == 69);
    }

    #[test]
    fn insert_and_get_many() {
        let mut map = DenseHandleMap::<TestItem>::new();
        let handle1 = map.insert(TestItem::new(420, 69));
        let handle2 = map.insert(TestItem::new(421, 70));
        let handle3 = map.insert(TestItem::new(422, 71));
        let item_ref1 = map.get(&handle1).unwrap();
        let item_ref2 = map.get(&handle2).unwrap();
        let item_ref3 = map.get(&handle3).unwrap();
        assert!(item_ref1.v1 == 420);
        assert!(item_ref1.v2 == 69);
        assert!(item_ref2.v1 == 421);
        assert!(item_ref2.v2 == 70);
        assert!(item_ref3.v1 == 422);
        assert!(item_ref3.v2 == 71);
    }

    #[test]
    fn insert_remove_replace() {
        let mut map = DenseHandleMap::<TestItem>::new();
        let handle1 = map.insert(TestItem::new(420, 69));
        let handle2 = map.insert(TestItem::new(421, 70));
        let handle3 = map.insert(TestItem::new(422, 71));
        map.remove(&handle2).unwrap();
        let handle2 = map.insert(TestItem::new(423, 72));
        let handle4 = map.insert(TestItem::new(424, 73));
        let item_ref1 = map.get(&handle1).unwrap();
        let item_ref2 = map.get(&handle2).unwrap();
        let item_ref3 = map.get(&handle3).unwrap();
        let item_ref4 = map.get(&handle4).unwrap();
        assert!(item_ref1.v1 == 420);
        assert!(item_ref1.v2 == 69);
        assert!(item_ref3.v1 == 422);
        assert!(item_ref3.v2 == 71);
        assert!(item_ref2.v1 == 423);
        assert!(item_ref2.v2 == 72);
        assert!(item_ref4.v1 == 424);
        assert!(item_ref4.v2 == 73);
    }
}
