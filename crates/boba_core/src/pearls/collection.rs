use std::any::Any;

use handle_map::{
    map::{
        dense::{Data, DataMut, DenseHandleMap, IterMut},
        sparse::SparseHandleMap,
    },
    Handle, RawHandle,
};
use hashbrown::{hash_map::Entry, HashMap};

use super::{Pearl, PearlId};

/// Represents a link to a single pearl in a [`PearlCollection`].
pub struct Link<P: Pearl> {
    pub map: RawHandle,
    pub pearl: Handle<P>,
}

impl<P: Pearl> Link<P> {
    fn new(map: RawHandle, pearl: Handle<P>) -> Self {
        Self { map, pearl }
    }
}

/// A storage solution for [`Pearl`] objects.
/// Stored pearls in a densly packed array, for quick iteration.
/// But also provides a link for their location for quick access.
#[derive(Default)]
pub struct PearlCollection {
    map_links: HashMap<PearlId, RawHandle>,
    maps: SparseHandleMap<Box<dyn Any>>,
}

impl PearlCollection {
    /// Returns a new collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a [`Pearl`] to this collection, returning a [`Link`] to its location.
    pub fn insert<P: Pearl>(&mut self, pearl: P) -> Link<P> {
        let (map_handle, pearl_handle) = match self.map_links.entry(PearlId::of::<P>()) {
            Entry::Occupied(e) => {
                let map_handle = *e.get();
                let any_map = self.maps.get_data_mut(map_handle.as_type()).unwrap();
                let map = any_map.downcast_mut::<DenseHandleMap<P>>().unwrap();
                let pearl_handle = map.insert(pearl);
                (map_handle, pearl_handle)
            }
            Entry::Vacant(e) => {
                let mut map = DenseHandleMap::<P>::new();
                let pearl_handle = map.insert(pearl);
                let map_handle = self.maps.insert(Box::new(map)).into_raw();
                e.insert(map_handle);
                (map_handle, pearl_handle)
            }
        };

        Link::new(map_handle, pearl_handle)
    }

    /// Returns true if `link` is valid for this collection.
    pub fn contains<P: Pearl>(&self, link: &Link<P>) -> bool {
        match self.get_map::<P>(&link.map) {
            Some(map) => map.contains(&link.pearl),
            _ => false,
        }
    }

    /// Returns true if a pearl of type `P` is stored in this collection.
    pub fn contains_type<P: Pearl>(&self) -> bool {
        self.map_links.contains_key(&PearlId::of::<P>())
    }

    /// Returns a reference to the pearl that `link` points to.
    ///
    /// Returns `None` if the link is invalid.
    pub fn get<P: Pearl>(&self, link: &Link<P>) -> Option<&P> {
        let map = self.get_map::<P>(&link.map)?;
        map.get_data(&link.pearl)
    }

    /// Returns a mutable reference to the pearl that `link` points to.
    ///
    /// Returns `None` if the link is invalid.
    pub fn get_mut<P: Pearl>(&mut self, link: &Link<P>) -> Option<&mut P> {
        let map = self.get_map_mut::<P>(&link.map)?;
        map.get_data_mut(&link.pearl)
    }

    /// Removes and returns the pearl that `link` points to.
    ///
    /// Returns `None` if the link is invalid.
    pub fn remove<P: Pearl>(&mut self, link: &Link<P>) -> Option<P> {
        let map = self.get_map_mut::<P>(&link.map)?;
        map.remove(&link.pearl)
    }

    /// Returns an iterator over all pearls of type `P`
    ///
    /// Returns `None` if there are no pearls
    pub fn pearls<P: Pearl>(&self) -> Option<Data<P>> {
        let map_link = self.map_links.get(&PearlId::of::<P>())?;
        Some(self.get_map(map_link)?.data())
    }

    /// Returns a mutable iterator over all pearls of type `P`
    ///
    /// Returns `None` if there are no pearls
    pub fn pearls_mut<P: Pearl>(&mut self) -> Option<DataMut<P>> {
        let map_link = *self.map_links.get(&PearlId::of::<P>())?;
        Some(self.get_map_mut(&map_link)?.data_mut())
    }

    // Currently scoped to this crate, as it exposes the handle import
    pub(crate) fn iter_mut<P: Pearl>(&mut self) -> Option<IterMut<P>> {
        let map_link = *self.map_links.get(&PearlId::of::<P>())?;
        Some(self.get_map_mut(&map_link)?.iter_mut())
    }

    fn get_map<P: Pearl>(&self, handle: &RawHandle) -> Option<&DenseHandleMap<P>> {
        let any_map = self.maps.get_data(handle.as_type())?;
        any_map.downcast_ref::<DenseHandleMap<P>>()
    }

    fn get_map_mut<P: Pearl>(&mut self, handle: &RawHandle) -> Option<&mut DenseHandleMap<P>> {
        let any_map = self.maps.get_data_mut(handle.as_type())?;
        any_map.downcast_mut::<DenseHandleMap<P>>()
    }
}
