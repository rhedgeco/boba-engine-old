use std::{collections::hash_map::Entry, ops::AddAssign};

use fxhash::FxHashMap;

use crate::pearl::{Pearl, PearlExt, PearlId};

use super::{Handle, RawHandle};

pub trait InsertDestroyQueue {
    fn queue_insert<P: Pearl>(&mut self, pearl: P);
    fn queue_destroy<P: Pearl>(&mut self, handle: Handle<P>);
}

struct InsertTracker {
    map_index: u16,
    length: u32,
}

pub struct PearlMapQueue<'a, T: InsertDestroyQueue> {
    map_indices: &'a FxHashMap<PearlId, u16>,
    open_routes: &'a Vec<Vec<RawHandle>>,
    map_lengths: &'a Vec<u32>,

    map_trackers: FxHashMap<PearlId, InsertTracker>,
    new_maps: usize,

    queue: &'a mut T,
}

impl<'a, T: InsertDestroyQueue> PearlMapQueue<'a, T> {
    pub(super) fn new(
        map_indices: &'a FxHashMap<PearlId, u16>,
        open_routes: &'a Vec<Vec<RawHandle>>,
        map_lengths: &'a Vec<u32>,
        queue: &'a mut T,
    ) -> Self {
        Self {
            map_indices,
            open_routes,
            map_lengths,
            map_trackers: Default::default(),
            new_maps: 0,
            queue,
        }
    }

    pub fn destroy(&mut self, handle: Handle<impl Pearl>) {
        self.queue.queue_destroy(handle);
    }

    pub fn insert<P: Pearl>(&mut self, pearl: P) -> Handle<P> {
        self.queue.queue_insert(pearl);

        let tracker = match self.map_trackers.entry(P::id()) {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => {
                let (map_index, length) = match self.map_indices.get(&P::id()) {
                    Some(map_index) => {
                        let length = self.map_lengths[*map_index as usize];
                        (*map_index, length)
                    }
                    None => {
                        let map_index = u16::try_from(self.map_lengths.len() + self.new_maps)
                            .expect(
                                "PearlQueue type capacity overflow. Pearl types are over u16::MAX",
                            );

                        self.new_maps.add_assign(1);
                        (map_index, 0)
                    }
                };

                e.insert(InsertTracker { map_index, length })
            }
        };

        let map_index = tracker.map_index as usize;
        if map_index > self.open_routes.len() {
            RawHandle::from_raw_parts(tracker.length, tracker.map_index, 0).into_type()
        } else {
            let open_routes = &self.open_routes[map_index];
            let map_length = self.map_lengths[map_index];

            let open_length = open_routes.len() as u32;
            let raw_handle = if tracker.length < open_length {
                open_routes[open_routes.len() - 1 - tracker.length as usize]
            } else {
                let pearl_index = tracker.length + map_length - open_length;
                RawHandle::from_raw_parts(pearl_index, map_index as u16, 0)
            };

            raw_handle.into_type()
        }
    }
}
