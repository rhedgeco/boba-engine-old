use std::{
    any::Any,
    slice::{Iter, IterMut},
};

use fxhash::FxHashMap;

use crate::pearls::{Pearl, PearlExt, PearlId};

use super::{ExclusiveStream, Handle, PearlData, PearlLocation};

pub struct PearlAccessMap<'a> {
    map_ids: &'a FxHashMap<PearlId, u16>,
    locations: &'a Vec<Vec<PearlLocation>>,
    pearls: &'a mut Vec<Box<dyn Any>>,
}

impl<'a> PearlAccessMap<'a> {
    pub(super) fn new(
        map_ids: &'a FxHashMap<PearlId, u16>,
        locations: &'a Vec<Vec<PearlLocation>>,
        pearls: &'a mut Vec<Box<dyn Any>>,
    ) -> Self {
        Self {
            map_ids,
            locations,
            pearls,
        }
    }

    pub fn get<P: Pearl>(&self, handle: Handle<P>) -> Option<&P> {
        let pearl_index = self.validate_handle(handle)?;
        let pearl_map = self.get_map(handle.map_id())?;
        Some(&pearl_map[pearl_index])
    }

    pub fn get_mut<P: Pearl>(&mut self, handle: Handle<P>) -> Option<&mut P> {
        let pearl_index = self.validate_handle(handle)?;
        let pearl_map = self.get_map_mut(handle.map_id())?;
        Some(&mut pearl_map[pearl_index])
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
    pub fn exclusive_stream<P: Pearl>(&'a mut self) -> Option<ExclusiveStream<P>> {
        ExclusiveStream::new(self)
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
        if location.handle != handle.into_raw() {
            return None; // check to make sure link is valid
        }

        location.index
    }
}
