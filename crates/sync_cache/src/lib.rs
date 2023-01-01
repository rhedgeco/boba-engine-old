use std::{
    hash::Hash,
    sync::{Arc, RwLock},
};

use hashbrown::{hash_map::Entry, HashMap};

/// A caching system similar to OnceCell, but for a map of data.
#[derive(Clone)]
pub struct SyncCache<K, V> {
    cache: Arc<RwLock<HashMap<K, V>>>,
}

impl<K, V> Default for SyncCache<K, V> {
    fn default() -> Self {
        Self {
            cache: Default::default(),
        }
    }
}

impl<K, V> SyncCache<K, V> {
    /// Either gets the item stored in the map, or uses the provided function to create a new item and cache it.
    pub fn get_or_init<'a, F>(&'a self, key: &K, f: F) -> &V
    where
        K: Eq + Hash + Clone,
        F: FnOnce() -> V,
    {
        // First try only reading from the map. This will only lock if there is a write currently happening.
        // This means that accesses should be fairly fast, because the map will only be locked to read when there is already a write happening.
        // If the cache data is there, a reference to it will be returned.
        // The reference is transmuted to change the lifetime to be associated with the SyncCache, not the lock guard.
        // This is safe because the data, once in the hashmap, will never be modified or destroyed, and will live as long as the SyncCache.
        let map = self.cache.read().unwrap();
        match map.get(key) {
            Some(shader) => return unsafe { std::mem::transmute(shader) },
            _ => (),
        }

        // If the shader has not been uploaded before, we need get a write lock and create the new entry for the shader.
        let mut map = self.cache.write().unwrap();
        return match map.entry(key.clone()) {
            // if somehow the shader has been compiled between locks, well great! We will return it now.
            Entry::Occupied(e) => unsafe { std::mem::transmute(&*e.into_mut()) },
            Entry::Vacant(e) => unsafe { std::mem::transmute(&*e.insert(f())) },
        };
    }
}
