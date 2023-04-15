use std::any::Any;

use handle_map::{map::HandleMapId, Handle, RawHandle};
use hashbrown::{hash_map::Entry, HashMap};

use crate::{
    pearls::{Pearl, PearlExt, PearlId},
    BobaResources, Event,
};

use super::{
    EventRegistry, Iter, IterMut, PearlAccessMap, PearlLink, PearlQueue, PearlQueueEvents,
};

#[derive(Default)]
pub struct PearlMap {
    inner: InnerPearlMap,
    events: EventRegistry,
}

impl PearlMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<P: Pearl>(&mut self, pearl: P) -> PearlLink<P> {
        self.inner.insert(pearl, || P::register(&mut self.events))
    }

    pub fn get<P: Pearl>(&self, link: PearlLink<P>) -> Option<&P> {
        self.inner.get(link)
    }

    pub fn get_mut<P: Pearl>(&mut self, link: PearlLink<P>) -> Option<&mut P> {
        self.inner.get_mut(link)
    }

    pub fn remove<P: Pearl>(&mut self, link: PearlLink<P>) -> Option<P> {
        self.inner.remove(link)
    }

    pub fn iter<P: Pearl>(&self) -> Option<Iter<P>> {
        self.inner.iter()
    }

    pub fn iter_mut<P: Pearl>(&mut self) -> Option<IterMut<P>> {
        self.inner.iter_mut()
    }

    pub fn as_queue<'a>(&'a self, pearl_queue: &'a mut PearlQueueEvents) -> PearlQueue {
        self.inner.as_queue(pearl_queue)
    }

    pub fn as_access_map(&mut self) -> PearlAccessMap {
        self.inner.as_access_map()
    }

    pub fn split_queue_access<'a>(
        &'a mut self,
        pearl_queue: &'a mut PearlQueueEvents,
    ) -> (PearlQueue, PearlAccessMap) {
        self.inner.split_queue_access(pearl_queue)
    }

    pub fn trigger<E: Event>(&mut self, event: &E, resources: &mut BobaResources) {
        let mut pearl_queue = PearlQueueEvents::new();
        self.events
            .run_event(event, &mut self.inner, resources, &mut pearl_queue);
        pearl_queue.merge_into(self);
    }
}

pub(super) struct InnerPearlMap {
    pub(super) map_id: u16,
    pub(super) pearl_link: HashMap<PearlId, usize>,
    pub(super) data_links: Vec<Vec<(RawHandle, Option<usize>)>>,
    pub(super) open_links: Vec<Vec<RawHandle>>,
    pub(super) pearl_counts: Vec<usize>,
    pub(super) pearl_data: Vec<Box<dyn Any>>,
}

impl Default for InnerPearlMap {
    fn default() -> Self {
        Self {
            map_id: HandleMapId::generate(),
            pearl_link: Default::default(),
            data_links: Default::default(),
            open_links: Default::default(),
            pearl_counts: Default::default(),
            pearl_data: Default::default(),
        }
    }
}

impl InnerPearlMap {
    pub fn insert<P: Pearl>(&mut self, pearl: P, new_map_callback: impl FnOnce()) -> PearlLink<P> {
        let map_index = self.get_or_create_pearl_index::<P>(new_map_callback);
        let data_link = &mut self.data_links[map_index];
        let open_link = &mut self.open_links[map_index];
        let pearl_data = self.pearl_data[map_index]
            .downcast_mut::<Vec<(Handle<P>, P)>>()
            .unwrap();

        self.pearl_counts[map_index] = self.pearl_counts[map_index] + 1;
        match open_link.pop() {
            Some(open_pearl_index) => {
                let (handle, pearl_link) = &mut data_link[open_pearl_index.uindex()];
                *pearl_link = Some(pearl_data.len());
                pearl_data.push((handle.into_type(), pearl));
                PearlLink::new(map_index, handle.into_type())
            }
            None => {
                let pearl_index = data_link.len();
                let handle = RawHandle::from_raw_parts_usize(pearl_index, 0, self.map_id);
                data_link.push((handle, Some(pearl_data.len())));
                pearl_data.push((handle.into_type(), pearl));
                PearlLink::new(map_index, handle.into_type())
            }
        }
    }

    pub fn remove<P: Pearl>(&mut self, link: PearlLink<P>) -> Option<P> {
        let data_link = self.data_links.get_mut(link.map_index)?; // get the link vector
        let (handle, pearl_index) = data_link.get_mut(link.handle.uindex())?; // get the link out of the vector
        if *handle != link.handle.into_raw() {
            return None; // check to make sure link is valid
        }

        *handle = handle.increment_generation(); // increment handle, so it cant be used again
        let pearl_index = std::mem::replace(pearl_index, None)?; // get the index of the pearl to remove
        self.open_links[link.map_index].push(*handle); // add link index to open links for re-use
        let pearl_data = self.pearl_data[link.map_index]
            .downcast_mut::<Vec<(Handle<P>, P)>>()
            .unwrap(); // get the pearl data vec

        let (_, pearl) = pearl_data.swap_remove(pearl_index); // swap remove the pearl to keep vec packed
        if let Some((swapped_handle, _)) = pearl_data.get_mut(pearl_index) {
            // fix the link for the swapped handle if there is one
            data_link[swapped_handle.uindex()].1 = Some(pearl_index);
        }

        self.pearl_counts[link.map_index] = self.pearl_counts[link.map_index] - 1;
        Some(pearl)
    }

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

    pub fn as_queue<'a>(&'a self, queue_events: &'a mut PearlQueueEvents) -> PearlQueue {
        PearlQueue {
            map_id: self.map_id,
            pearl_link: &self.pearl_link,
            open_links: &self.open_links,
            pearl_counts: &self.pearl_counts,
            queue_events,
        }
    }

    pub fn as_access_map(&mut self) -> PearlAccessMap {
        PearlAccessMap {
            pearl_link: &self.pearl_link,
            data_links: &self.data_links,
            pearl_data: &mut self.pearl_data,
        }
    }

    pub fn split_queue_access<'a>(
        &'a mut self,
        pearl_queue: &'a mut PearlQueueEvents,
    ) -> (PearlQueue, PearlAccessMap) {
        (
            PearlQueue {
                map_id: self.map_id,
                pearl_link: &self.pearl_link,
                open_links: &self.open_links,
                pearl_counts: &self.pearl_counts,
                queue_events: pearl_queue,
            },
            PearlAccessMap {
                pearl_link: &self.pearl_link,
                data_links: &self.data_links,
                pearl_data: &mut self.pearl_data,
            },
        )
    }

    fn get_map<P: Pearl>(&self, map_index: usize) -> Option<&Vec<(Handle<P>, P)>> {
        self.pearl_data[map_index].downcast_ref::<Vec<(Handle<P>, P)>>()
    }

    fn get_map_mut<P: Pearl>(&mut self, map_index: usize) -> Option<&mut Vec<(Handle<P>, P)>> {
        self.pearl_data[map_index].downcast_mut::<Vec<(Handle<P>, P)>>()
    }

    fn validate_link(&self, link: PearlLink<impl Pearl>) -> Option<usize> {
        let data_link = self.data_links.get(link.map_index)?;
        let (handle, pearl_index) = data_link.get(link.handle.uindex())?;
        if *handle != link.handle.into_raw() {
            return None;
        }

        *pearl_index
    }

    fn get_or_create_pearl_index<P: Pearl>(&mut self, new_map_callback: impl FnOnce()) -> usize {
        match self.pearl_link.entry(P::id()) {
            Entry::Occupied(e) => *e.into_mut(),
            Entry::Vacant(e) => {
                new_map_callback();
                let index = self.data_links.len();
                self.data_links.push(Vec::new());
                self.open_links.push(Vec::new());
                self.pearl_counts.push(0);
                self.pearl_data.push(Box::new(Vec::<(Handle<P>, P)>::new()));
                *e.insert(index)
            }
        }
    }
}
