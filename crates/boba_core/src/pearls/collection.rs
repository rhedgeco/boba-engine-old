use std::any::Any;

use handle_map::{
    map::{
        dense::{self, Data, DataMut, DenseHandleMap},
        sparse::SparseHandleMap,
    },
    RawHandle,
};
use hashbrown::{hash_map::Entry, HashMap};

use super::{Link, Pearl, PearlId};

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

    /// Returns a [`SplitStep`] over the pearls of type `P` in this collection.
    ///
    /// Returns `None` there are no pearls.
    pub fn split_step<P: Pearl>(&mut self) -> Option<SplitStep<P>> {
        SplitStep::new(self)
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

/// A wrapper over a [`PearlCollection`] that only allows accessing items and not mutating the collection.
/// It also prevents access to a single pearl as it is the main type used in a [`SplitStep`].
pub struct ExclusivePearlProvider<'a> {
    pub(crate) exclude: RawHandle,
    collection: &'a mut PearlCollection,
}

impl<'a> ExclusivePearlProvider<'a> {
    /// Returns a reference to the pearl that `link` points to.
    ///
    /// Returns `None` if the link is invalid, or if the pearl has been excluded.
    pub fn get<P: Pearl>(&self, link: &Link<P>) -> Option<&P> {
        if &self.exclude == link.pearl.as_raw() {
            return None;
        }

        self.collection.get(link)
    }

    /// Returns a mutable reference to the pearl that `link` points to.
    ///
    /// Returns `None` if the link is invalid, or if the pearl has been excluded.
    pub fn get_mut<P: Pearl>(&mut self, link: &Link<P>) -> Option<&mut P> {
        if &self.exclude == link.pearl.as_raw() {
            return None;
        }

        self.collection.get_mut(link)
    }
}

type Iter<'a, P> = Data<'a, P>;
type IterMut<'a, P> = DataMut<'a, P>;

/// A "iterator" of sorts that provides access to pearls in an iterator fashion.
/// But it also gives an [`ExclusivePearlProvider`] that allows indexing into all
/// other pearls using [`Link`]. This is useful to be able to iterate over all pearls while
/// also allowing access to the other pearls in case they need to reference eachother.
pub struct SplitStep<'a, P: Pearl> {
    pub(crate) map_link: RawHandle,
    iter: dense::IterMut<'a, P>,
    collection: &'a mut PearlCollection,
}

impl<'a, P: Pearl> SplitStep<'a, P> {
    /// Creates and returns a new split step over `collection`.
    ///
    /// Returns `None` if there are no pearls of type `P`.
    pub fn new(collection: &'a mut PearlCollection) -> Option<Self> {
        // SAFETY: We split the collection to have a reference to itself and its iterator.
        // while this technically aliases over the collection twice, the split step does not access both at the same time.
        // Furthermore, for each step of next, this gives back a single mutable pearl and an
        // exclusive collection that excludes the other pearl that is being returned.
        // This ensures that while we have techincally have multimple exclusive access,
        // all methods of accessing the data does not have any overlap, and as such is safe to use.
        let ptr = collection as *mut PearlCollection;
        let map_link = *collection.map_links.get(&PearlId::of::<P>())?;
        let iter = collection.get_map_mut(&map_link)?.iter_mut();
        let collection = unsafe { &mut *ptr };
        Some(Self {
            map_link,
            iter,
            collection,
        })
    }

    /// Gets the next pearl, and the matching [`ExclusivePearlProvider`].
    ///
    /// This diverges from typical iterators as the returned items must go out scope
    /// before `next` is called again. This is due to providing access to the whole
    /// collection through the exclusive provider.
    pub fn next(&mut self) -> Option<(&mut P, ExclusivePearlProvider)> {
        let (handle, pearl) = self.iter.next()?;
        Some((
            pearl,
            ExclusivePearlProvider {
                exclude: handle.into_raw(),
                collection: self.collection,
            },
        ))
    }
}
