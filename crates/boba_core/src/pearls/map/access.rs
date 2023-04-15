use std::any::Any;

use handle_map::{Handle, RawHandle};
use hashbrown::HashMap;

use crate::pearls::{Pearl, PearlExt, PearlId};

use super::{ExclusiveStream, Iter, IterMut, PearlLink};

pub struct PearlAccessMap<'a> {
    pub(super) pearl_link: &'a HashMap<PearlId, usize>,
    pub(super) data_links: &'a Vec<Vec<(RawHandle, Option<usize>)>>,
    pub(super) pearl_data: &'a mut Vec<Box<dyn Any>>,
}

impl<'a> PearlAccessMap<'a> {
    pub fn get<P: Pearl>(&self, link: PearlLink<P>) -> Option<&P> {
        let map_index = self.validate_link(link)?;
        let pearl_map = self.get_map(link.map_index)?;
        Some(&pearl_map[map_index].1)
    }

    pub fn get_mut<P: Pearl>(&mut self, link: PearlLink<P>) -> Option<&mut P> {
        let map_index = self.validate_link(link)?;
        let pearl_map = self.get_map_mut(link.map_index)?;
        Some(&mut pearl_map[map_index].1)
    }

    pub fn iter<P: Pearl>(&self) -> Option<Iter<P>> {
        let map_index = *self.pearl_link.get(&P::id())?;
        let pearl_map = self.get_map(map_index)?;
        Some(Iter {
            map_index,
            inner: pearl_map.iter(),
        })
    }

    pub fn iter_mut<P: Pearl>(&mut self) -> Option<IterMut<P>> {
        let map_index = *self.pearl_link.get(&P::id())?;
        let pearl_map = self.get_map_mut(map_index)?;
        Some(IterMut {
            map_index,
            inner: pearl_map.iter_mut(),
        })
    }

    pub fn exclusive_stream<P: Pearl>(&'a mut self) -> Option<ExclusiveStream<P>> {
        ExclusiveStream::new(self)
    }

    fn validate_link(&self, link: PearlLink<impl Pearl>) -> Option<usize> {
        let data_link = self.data_links.get(link.map_index)?;
        let (handle, pearl_index) = data_link.get(link.handle.uindex())?;
        if *handle != link.handle.into_raw() {
            return None;
        }

        *pearl_index
    }

    fn get_map<P: Pearl>(&self, map_index: usize) -> Option<&Vec<(Handle<P>, P)>> {
        self.pearl_data[map_index].downcast_ref::<Vec<(Handle<P>, P)>>()
    }

    fn get_map_mut<P: Pearl>(&mut self, map_index: usize) -> Option<&mut Vec<(Handle<P>, P)>> {
        self.pearl_data[map_index].downcast_mut::<Vec<(Handle<P>, P)>>()
    }
}
