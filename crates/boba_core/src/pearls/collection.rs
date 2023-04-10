use super::{Link, Pearl, PearlId, PearlLink};
use crate::{
    events::{Event, EventManager},
    BobaResources,
};
use handle_map::{
    map::{
        dense::{self, Data, DataMut, DenseHandleMap},
        sparse::SparseHandleMap,
    },
    RawHandle,
};
use hashbrown::{hash_map::Entry, HashMap};
use std::{
    any::Any,
    ops::{Deref, DerefMut},
};

type Iter<'a, P> = Data<'a, P>;
type IterMut<'a, P> = DataMut<'a, P>;

/// A wrapper around a [`PearlCollectionNew`] that provides access to modify the collection and run events.
#[derive(Default)]
pub struct PearlManager {
    events: EventManager,
    pearls: PearlCollection,
}

impl DerefMut for PearlManager {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pearls
    }
}

impl Deref for PearlManager {
    type Target = PearlCollection;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.pearls
    }
}

impl PearlManager {
    /// Returns a new pearl manager.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts `pearl` into the collection, and returns a [`Link`] to its location.
    pub fn insert<P: Pearl>(&mut self, pearl: P) -> Link<P> {
        let (map_handle, pearl_handle) = match self.pearls.map_links.entry(PearlId::of::<P>()) {
            Entry::Occupied(e) => {
                let map_handle = *e.get();
                let any_map = self.pearls.maps.get_data_mut(map_handle.as_type()).unwrap();
                let map = any_map.downcast_mut::<DenseHandleMap<P>>().unwrap();
                let pearl_handle = map.insert(pearl);
                (map_handle, pearl_handle)
            }
            Entry::Vacant(e) => {
                P::register(&mut self.events);
                let mut map = DenseHandleMap::<P>::new();
                let pearl_handle = map.insert(pearl);
                let map_handle = self.pearls.maps.insert(Box::new(map)).into_raw();
                e.insert(map_handle);
                (map_handle, pearl_handle)
            }
        };

        Link::new(map_handle, pearl_handle)
    }

    /// Removes and returns the pearl that `link` points to.
    ///
    /// Returns `None` if the link is invalid.
    pub fn remove<P: Pearl>(&mut self, link: &Link<P>) -> Option<P> {
        let map = self.pearls.get_map_mut::<P>(&link.map)?;
        map.remove(&link.pearl)
    }

    pub fn trigger<E: Event>(&mut self, event: &E, resources: &mut BobaResources) {
        let commands = self.events.trigger(event, &mut self.pearls, resources);
        commands.execute(self, resources);
    }
}

/// A collection of pearls that can be queried in a number of ways.
///
/// The collection layout cannot be modified.
/// The only way to modify a collection is within a [`PearlManager`].
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

    /// Returns an iterator over all pearls of type `P`
    ///
    /// Returns `None` if there are no pearls
    pub fn pearls<P: Pearl>(&self) -> Option<Iter<P>> {
        let map_link = self.map_links.get(&PearlId::of::<P>())?;
        Some(self.get_map(map_link)?.data())
    }

    /// Returns a mutable iterator over all pearls of type `P`
    ///
    /// Returns `None` if there are no pearls
    pub fn pearls_mut<P: Pearl>(&mut self) -> Option<IterMut<P>> {
        let map_link = *self.map_links.get(&PearlId::of::<P>())?;
        Some(self.get_map_mut(&map_link)?.data_mut())
    }

    /// Returns an exclusive stream over pearls of type `P`.
    ///
    /// Returns `None` if there are no pearls.
    #[inline]
    pub fn exclusive_stream<P: Pearl>(&mut self) -> Option<ExclusiveStream<P>> {
        ExclusiveStream::new(self)
    }

    /// Returns a reference to the [`DenseHandleMap`] for pearl `P`.
    ///
    /// Returns `None` if one does not exist.
    fn get_map<P: Pearl>(&self, handle: &RawHandle) -> Option<&DenseHandleMap<P>> {
        let any_map = self.maps.get_data(handle.as_type())?;
        any_map.downcast_ref::<DenseHandleMap<P>>()
    }

    /// Returns a mutable reference to the [`DenseHandleMap`] for pearl `P`.
    ///
    /// Returns `None` if one does not exist.
    fn get_map_mut<P: Pearl>(&mut self, handle: &RawHandle) -> Option<&mut DenseHandleMap<P>> {
        let any_map = self.maps.get_data_mut(handle.as_type())?;
        any_map.downcast_mut::<DenseHandleMap<P>>()
    }
}

pub struct ExclusivePearlCollection<'a> {
    exclude: RawHandle,
    pearls: &'a mut PearlCollection,
}

impl<'a> ExclusivePearlCollection<'a> {
    /// Returns true if `link` is valid for this collection.
    pub fn contains<P: Pearl>(&self, link: &Link<P>) -> bool {
        match self.pearls.get_map::<P>(&link.map) {
            Some(map) => map.contains(&link.pearl),
            _ => false,
        }
    }

    /// Returns true if a pearl of type `P` is stored in this collection.
    pub fn contains_type<P: Pearl>(&self) -> bool {
        self.pearls.map_links.contains_key(&PearlId::of::<P>())
    }

    /// Returns a reference to the pearl that `link` points to.
    ///
    /// Returns `None` if the link is invalid, or the pearl has been excluded.
    pub fn get<P: Pearl>(&self, link: &Link<P>) -> Option<&P> {
        if link.pearl.as_raw() == &self.exclude {
            return None;
        }

        let map = self.pearls.get_map::<P>(&link.map)?;
        map.get_data(&link.pearl)
    }

    /// Returns a mutable reference to the pearl that `link` points to.
    ///
    /// Returns `None` if the link is invalid, or the pearl has been excluded.
    pub fn get_mut<P: Pearl>(&mut self, link: &Link<P>) -> Option<&mut P> {
        if link.pearl.as_raw() == &self.exclude {
            return None;
        }

        let map = self.pearls.get_map_mut::<P>(&link.map)?;
        map.get_data_mut(&link.pearl)
    }
}

pub struct ExclusiveStream<'a, P: Pearl> {
    map: &'a RawHandle,
    iter: dense::IterMut<'a, P>,
    collection: &'a mut PearlCollection,
}

impl<'a, P: Pearl> ExclusiveStream<'a, P> {
    pub fn new(collection: &'a mut PearlCollection) -> Option<Self> {
        let ptr = collection as *mut PearlCollection;
        let map_link = collection.map_links.get(&PearlId::of::<P>())?;
        let map = collection.maps.get_data_mut(map_link.as_type())?;
        let map = map.downcast_mut::<DenseHandleMap<P>>()?;
        Some(Self {
            map: map_link,
            iter: map.iter_mut(),
            collection: unsafe { &mut *ptr },
        })
    }

    pub fn next(&mut self) -> Option<(PearlLink<P>, ExclusivePearlCollection)> {
        let (handle, pearl) = self.iter.next()?;
        Some((
            PearlLink::new(pearl, Link::new(*self.map, *handle)),
            ExclusivePearlCollection {
                exclude: handle.into_raw(),
                pearls: self.collection,
            },
        ))
    }
}
