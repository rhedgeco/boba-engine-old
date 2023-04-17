use std::{any::Any, ops::AddAssign};

use fxhash::FxHashMap;
use indexmap::{map::Entry, IndexMap};

use crate::pearls::{Pearl, PearlExt, PearlId};

use super::{BobaPearls, Handle, RawHandle};

pub struct PearlEventQueue<'a> {
    map_ids: &'a FxHashMap<PearlId, u16>,
    available: &'a Vec<Vec<RawHandle>>,
    map_sizes: &'a Vec<u32>,
    queue: &'a mut PearlQueue,
}

impl<'a> PearlEventQueue<'a> {
    pub(super) fn new(
        map_ids: &'a FxHashMap<PearlId, u16>,
        available: &'a Vec<Vec<RawHandle>>,
        map_sizes: &'a Vec<u32>,
        queue: &'a mut PearlQueue,
    ) -> Self {
        Self {
            map_ids,
            available,
            map_sizes,
            queue,
        }
    }

    pub fn insert<P: Pearl>(&mut self, pearl: P) -> Handle<P> {
        let map_queue = self.get_or_create_map_queue::<P>();
        let insert_index = map_queue.insert(pearl);
        match map_queue.map_type {
            MapType::New(map_index) => {
                let raw_handle = RawHandle::from_raw_parts(insert_index, map_index, 0);
                raw_handle.into_type()
            }
            MapType::Existing(map_index) => {
                let available = &self.available[map_index as usize];
                let map_size = self.map_sizes[map_index as usize];

                let available_length = available.len() as u32;
                let raw_handle = if insert_index < available_length {
                    available[available.len() - 1 - insert_index as usize]
                } else {
                    let pearl_index = insert_index + map_size - available_length;
                    RawHandle::from_raw_parts(pearl_index, map_index, 0)
                };

                raw_handle.into_type()
            }
        }
    }

    pub fn destroy<P: Pearl>(&mut self, handle: Handle<P>) {
        self.queue.destroy_queue.push(DestroyPearl::new(handle));
    }

    fn get_or_create_map_queue<P: Pearl>(&mut self) -> &mut PearlMapQueue {
        match self.queue.insert_queues.entry(P::id()) {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => {
                let map_type = match self.map_ids.get(&P::id()) {
                    Some(index) => MapType::Existing(*index),
                    None => {
                        let map_index = u16::try_from(self.map_sizes.len() + self.queue.new_maps)
                            .expect(
                                "PearlQueue type capacity overflow. Pearl types are over u16::MAX",
                            );
                        self.queue.new_maps.add_assign(1);
                        MapType::New(map_index)
                    }
                };

                e.insert(PearlMapQueue::new::<P>(map_type))
            }
        }
    }
}

#[derive(Default)]
pub struct PearlQueue {
    insert_queues: IndexMap<PearlId, PearlMapQueue>,
    destroy_queue: Vec<DestroyPearl>,
    new_maps: usize,
}

impl PearlQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn merge_into(self, map: &mut BobaPearls) {
        for (_, queue) in self.insert_queues.into_iter() {
            queue.finalize(map);
        }

        for destroy in self.destroy_queue.into_iter() {
            destroy.finalize(map);
        }
    }
}

enum MapType {
    New(u16),
    Existing(u16),
}

struct PearlMapQueue {
    map_type: MapType,
    pearls: Box<dyn Any>,
    insert_function: fn(Box<dyn Any>, &mut BobaPearls),
}

impl PearlMapQueue {
    pub fn new<P: Pearl>(map_type: MapType) -> Self {
        Self {
            map_type,
            pearls: Box::new(Vec::<P>::new()),
            insert_function: Self::_insert_function::<P>,
        }
    }

    pub fn insert<P: Pearl>(&mut self, pearl: P) -> u32 {
        let pearls = self
            .pearls
            .downcast_mut::<Vec<P>>()
            .expect("Invalid internal pearl queue cast");
        let index = u32::try_from(pearls.len()).expect("PearlQueue capacity overflow.");
        pearls.push(pearl);
        index
    }

    pub fn finalize(self, map: &mut BobaPearls) {
        (self.insert_function)(self.pearls, map);
    }

    fn _insert_function<P: Pearl>(pearls: Box<dyn Any>, map: &mut BobaPearls) {
        let pearls = *pearls.downcast::<Vec<P>>().unwrap();
        for pearl in pearls.into_iter() {
            map.insert(pearl);
        }
    }
}

struct DestroyPearl {
    link: Box<dyn Any>,
    destroy_function: fn(Box<dyn Any>, &mut BobaPearls),
}

impl DestroyPearl {
    pub fn new<P: Pearl>(handle: Handle<P>) -> Self {
        Self {
            link: Box::new(handle),
            destroy_function: Self::_destroy_function::<P>,
        }
    }

    pub fn finalize(self, map: &mut BobaPearls) {
        (self.destroy_function)(self.link, map);
    }

    fn _destroy_function<P: Pearl>(link: Box<dyn Any>, map: &mut BobaPearls) {
        let link = *link.downcast::<Handle<P>>().unwrap();
        map.remove(link);
    }
}
