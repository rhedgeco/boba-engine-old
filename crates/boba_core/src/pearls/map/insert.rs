use std::{any::Any, ops::AddAssign};

use handle_map::RawHandle;
use hashbrown::HashMap;
use indexmap::{map::Entry, IndexMap};

use crate::pearls::{Pearl, PearlExt, PearlId};

use super::{PearlLink, PearlMap};

pub struct PearlInsertQueue<'a> {
    pub(super) map_id: u16,
    pub(super) pearl_link: &'a HashMap<PearlId, usize>,
    pub(super) open_links: &'a Vec<Vec<RawHandle>>,
    pub(super) pearl_counts: &'a Vec<usize>,
    pub(super) pearl_queue: &'a mut PearlQueue,
}

impl<'a> PearlInsertQueue<'a> {
    pub fn insert<P: Pearl>(&mut self, pearl: P) -> PearlLink<P> {
        let map_queue = self.get_or_create_map_queue::<P>();
        let insert_index = map_queue.insert(pearl);
        match map_queue.map_type {
            MapType::New(map_index) => {
                let raw_handle = RawHandle::from_raw_parts_usize(insert_index, 0, self.map_id);
                PearlLink::new(map_index, raw_handle.into_type())
            }
            MapType::Existing(map_index) => {
                let open_links = &self.open_links[map_index];
                let pearl_count = self.pearl_counts[map_index];

                let raw_handle = if insert_index < open_links.len() {
                    open_links[open_links.len() - 1 - insert_index]
                } else {
                    let pearl_index = insert_index + pearl_count - open_links.len();
                    RawHandle::from_raw_parts_usize(pearl_index, 0, self.map_id)
                };

                PearlLink::new(map_index, raw_handle.into_type())
            }
        }
    }

    pub fn destroy<P: Pearl>(&mut self, link: PearlLink<P>) {
        self.pearl_queue.destroy_queue.push(DestroyPearl::new(link));
    }

    fn get_or_create_map_queue<P: Pearl>(&mut self) -> &mut PearlMapQueue {
        match self.pearl_queue.insert_queues.entry(P::id()) {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => {
                let map_type = match self.pearl_link.get(&P::id()) {
                    Some(index) => MapType::Existing(*index),
                    None => {
                        let index = self.pearl_counts.len() + self.pearl_queue.new_maps;
                        self.pearl_queue.new_maps.add_assign(1);
                        MapType::New(index)
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

    pub fn merge_into(self, map: &mut PearlMap) {
        for (_, queue) in self.insert_queues.into_iter() {
            queue.finalize(map);
        }

        for destroy in self.destroy_queue.into_iter() {
            destroy.finalize(map);
        }
    }
}

enum MapType {
    New(usize),
    Existing(usize),
}

struct PearlMapQueue {
    map_type: MapType,
    pearls: Box<dyn Any>,
    insert_function: fn(Box<dyn Any>, &mut PearlMap),
}

impl PearlMapQueue {
    pub fn new<P: Pearl>(map_type: MapType) -> Self {
        Self {
            map_type,
            pearls: Box::new(Vec::<P>::new()),
            insert_function: Self::_insert_function::<P>,
        }
    }

    pub fn insert<P: Pearl>(&mut self, pearl: P) -> usize {
        let pearls = self
            .pearls
            .downcast_mut::<Vec<P>>()
            .expect("Invalid internal pearl queue cast");
        let index = pearls.len();
        pearls.push(pearl);
        index
    }

    pub fn finalize(self, map: &mut PearlMap) {
        (self.insert_function)(self.pearls, map);
    }

    fn _insert_function<P: Pearl>(pearls: Box<dyn Any>, map: &mut PearlMap) {
        let pearls = *pearls.downcast::<Vec<P>>().unwrap();
        for pearl in pearls.into_iter() {
            map.insert(pearl);
        }
    }
}

struct DestroyPearl {
    link: Box<dyn Any>,
    destroy_function: fn(Box<dyn Any>, &mut PearlMap),
}

impl DestroyPearl {
    pub fn new<P: Pearl>(link: PearlLink<P>) -> Self {
        Self {
            link: Box::new(link),
            destroy_function: Self::_destroy_function::<P>,
        }
    }

    pub fn finalize(self, map: &mut PearlMap) {
        (self.destroy_function)(self.link, map);
    }

    fn _destroy_function<P: Pearl>(link: Box<dyn Any>, map: &mut PearlMap) {
        let link = *link.downcast::<PearlLink<P>>().unwrap();
        map.remove(link);
    }
}
