use std::{
    any::Any,
    collections::hash_map::Entry,
    ops::{AddAssign, SubAssign},
    slice::{Iter, IterMut},
};

use fxhash::FxHashMap;

use crate::{
    pearls::{Pearl, PearlExt, PearlId},
    BobaResources, Event,
};

use super::{
    EventRegistry, Handle, PearlAccessMap, PearlData, PearlEventQueue, PearlQueue, RawHandle,
};

pub trait PearlProvider {
    fn get<P: Pearl>(&self, handle: Handle<P>) -> Option<&P>;
    fn get_mut<P: Pearl>(&mut self, handle: Handle<P>) -> Option<&mut P>;
}

#[derive(Default)]
pub struct BobaPearls {
    inner: RawPearlMap,
    events: EventRegistry,
}

impl PearlProvider for BobaPearls {
    fn get<P: Pearl>(&self, handle: Handle<P>) -> Option<&P> {
        self.inner.get(handle)
    }

    fn get_mut<P: Pearl>(&mut self, handle: Handle<P>) -> Option<&mut P> {
        self.inner.get_mut(handle)
    }
}

impl BobaPearls {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<P: Pearl>(&mut self, pearl: P) -> Handle<P> {
        self.inner.insert(pearl, || P::register(&mut self.events))
    }

    pub fn get<P: Pearl>(&self, handle: Handle<P>) -> Option<&P> {
        <Self as PearlProvider>::get(self, handle)
    }

    pub fn get_mut<P: Pearl>(&mut self, handle: Handle<P>) -> Option<&mut P> {
        <Self as PearlProvider>::get_mut(self, handle)
    }

    pub fn remove<P: Pearl>(&mut self, handle: Handle<P>) -> Option<P> {
        self.inner.remove(handle)
    }

    pub fn iter<P: Pearl>(&self) -> Option<Iter<PearlData<P>>> {
        self.inner.iter()
    }

    pub fn iter_mut<P: Pearl>(&mut self) -> Option<IterMut<PearlData<P>>> {
        self.inner.iter_mut()
    }

    pub fn trigger<E: Event>(&mut self, event: &E, resources: &mut BobaResources) {
        let mut pearl_queue = PearlQueue::new();
        self.events
            .run_event(event, &mut self.inner, resources, &mut pearl_queue);
        pearl_queue.merge_into(self);
    }
}

pub struct PearlLocation {
    pub handle: RawHandle,
    pub index: Option<usize>,
}

impl PearlLocation {
    pub fn new(handle: RawHandle, index: usize) -> Self {
        Self {
            handle,
            index: Some(index),
        }
    }
}

#[derive(Default)]
pub struct RawPearlMap {
    map_ids: FxHashMap<PearlId, u16>,
    locations: Vec<Vec<PearlLocation>>,
    available: Vec<Vec<RawHandle>>,
    map_sizes: Vec<u32>,
    pearls: Vec<Box<dyn Any>>,
}

impl PearlProvider for RawPearlMap {
    fn get<P: Pearl>(&self, handle: Handle<P>) -> Option<&P> {
        let pearl_index = self.validate_handle(handle)?;
        let pearl_map = self.get_map(handle.map_id())?;
        Some(&pearl_map[pearl_index])
    }

    fn get_mut<P: Pearl>(&mut self, handle: Handle<P>) -> Option<&mut P> {
        let pearl_index = self.validate_handle(handle)?;
        let pearl_map = self.get_map_mut(handle.map_id())?;
        Some(&mut pearl_map[pearl_index])
    }
}

impl RawPearlMap {
    pub fn insert<P: Pearl>(&mut self, pearl: P, on_new_map: impl FnOnce()) -> Handle<P> {
        let map_index = self.get_or_create_pearl_index::<P>(on_new_map) as usize;
        let locations = &mut self.locations[map_index];
        let available = &mut self.available[map_index];
        let pearls = self.pearls[map_index]
            .downcast_mut::<Vec<PearlData<P>>>()
            .unwrap();

        self.map_sizes[map_index].add_assign(1);
        match available.pop() {
            Some(available_index) => {
                let location = &mut locations[available_index.uindex()];
                location.index = Some(pearls.len());
                let handle = location.handle;
                pearls.push(PearlData::new(pearl, handle.into_type()));
                P::on_insert(handle.into_type(), self);
                handle.into_type()
            }
            None => {
                let pearl_index =
                    u32::try_from(locations.len()).expect("PearlMap capacity overflow");
                let handle = RawHandle::from_raw_parts(pearl_index, map_index as u16, 0);
                locations.push(PearlLocation::new(handle, pearls.len()));
                pearls.push(PearlData::new(pearl, handle.into_type()));
                P::on_insert(handle.into_type(), self);
                handle.into_type()
            }
        }
    }

    pub fn remove<P: Pearl>(&mut self, handle: Handle<P>) -> Option<P> {
        let locations = self.locations.get_mut(handle.umap_id())?; // get the link vector
        let location = locations.get_mut(handle.uindex())?; // get the link out of the vector
        if location.handle != handle {
            return None; // check to make sure link is valid
        }

        location.handle.increment_generation(); // increment handle, so it cant be used again
        let pearl_index = std::mem::replace(&mut location.index, None)?; // get the index of the pearl to remove
        self.available[handle.umap_id()].push(location.handle); // add link index to open links for re-use
        let pearls = self.pearls[handle.umap_id()].downcast_mut::<Vec<PearlData<P>>>()?; // get the pearl data vec

        let mut pearl_data = pearls.swap_remove(pearl_index); // swap remove the pearl to keep vec packed
        if let Some(swapped_data) = pearls.get_mut(pearl_index) {
            // fix the link for the swapped handle if there is one
            locations[swapped_data.handle().uindex()].index = Some(pearl_index);
        }

        self.map_sizes[handle.umap_id()].sub_assign(1); // decrement the tracked count
        P::on_remove(&mut pearl_data, self);
        Some(pearl_data.into_data().0)
    }

    pub fn iter<P: Pearl>(&self) -> Option<Iter<PearlData<P>>> {
        let map_index = *self.map_ids.get(&P::id())?;
        let pearl_map = self.get_map(map_index)?;
        Some(pearl_map.iter())
    }

    pub fn iter_mut<P: Pearl>(&mut self) -> Option<IterMut<PearlData<P>>> {
        let map_index = *self.map_ids.get(&P::id())?;
        let pearl_map = self.get_map_mut(map_index)?;
        Some(pearl_map.iter_mut())
    }

    pub fn as_queue<'a>(&'a self, queue: &'a mut PearlQueue) -> PearlEventQueue {
        PearlEventQueue::new(&self.map_ids, &self.available, &self.map_sizes, queue)
    }

    pub fn as_access_map(&mut self) -> PearlAccessMap {
        PearlAccessMap::new(&self.map_ids, &self.locations, &mut self.pearls)
    }

    pub fn split_queue_access<'a>(
        &'a mut self,
        queue: &'a mut PearlQueue,
    ) -> (PearlEventQueue, PearlAccessMap) {
        (
            PearlEventQueue::new(&self.map_ids, &self.available, &self.map_sizes, queue),
            PearlAccessMap::new(&self.map_ids, &self.locations, &mut self.pearls),
        )
    }

    fn get_map<P: Pearl>(&self, map_index: u16) -> Option<&Vec<PearlData<P>>> {
        self.pearls[map_index as usize].downcast_ref::<Vec<PearlData<P>>>()
    }

    fn get_map_mut<P: Pearl>(&mut self, map_index: u16) -> Option<&mut Vec<PearlData<P>>> {
        self.pearls[map_index as usize].downcast_mut::<Vec<PearlData<P>>>()
    }

    fn validate_handle(&self, handle: Handle<impl Pearl>) -> Option<usize> {
        let locations = self.locations.get(handle.umap_id())?;
        let location = locations.get(handle.uindex())?;
        if location.handle != handle {
            return None; // check to make sure link is valid
        }

        location.index
    }

    fn get_or_create_pearl_index<P: Pearl>(&mut self, on_new_map: impl FnOnce()) -> u16 {
        match self.map_ids.entry(P::id()) {
            Entry::Occupied(e) => *e.into_mut(),
            Entry::Vacant(e) => {
                let map_id = u16::try_from(self.locations.len())
                    .expect("PearlMap type capacity overflow. Pearl types are over u16::MAX");
                self.locations.push(Vec::new());
                self.available.push(Vec::new());
                self.map_sizes.push(0);
                self.pearls.push(Box::new(Vec::<PearlData<P>>::new()));
                let map_id = *e.insert(map_id);
                on_new_map();
                map_id
            }
        }
    }
}
