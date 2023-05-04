use std::{
    any::{Any, TypeId},
    collections::hash_map,
    slice::{Iter, IterMut},
};

use fxhash::{FxHashMap, FxHashSet};
use indexmap::{map, IndexMap};

use crate::{
    pearl::map::{
        ExclusiveIter, ExclusiveIterMut, Handle, InsertDestroyQueue, PearlData, PearlMapQueue,
        RawHandle,
    },
    pearl::{map::ExclusivePearlAccess, PearlExt},
    BobaResources, Event, EventListener, EventRegistrar, Pearl, PearlId, PearlMap,
};

#[derive(Default)]
pub struct BobaPearls {
    pearls: PearlMap,
    events: EventRegistry,
}

impl BobaPearls {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<P: Pearl>(&mut self, pearl: P) -> Handle<P> {
        self.pearls.insert(pearl, || P::register(&mut self.events))
    }

    pub fn remove<P: Pearl>(&mut self, handle: Handle<P>) -> Option<P> {
        self.pearls.remove(handle)
    }

    pub fn get<P: Pearl>(&self, handle: Handle<P>) -> Option<&P> {
        self.pearls.get(handle)
    }

    pub fn get_mut<P: Pearl>(&mut self, handle: Handle<P>) -> Option<&mut P> {
        self.pearls.get_mut(handle)
    }

    pub fn iter<P: Pearl>(&self) -> Iter<PearlData<P>> {
        self.pearls.iter()
    }

    pub fn iter_mut<P: Pearl>(&mut self) -> IterMut<PearlData<P>> {
        self.pearls.iter_mut()
    }

    pub fn trigger<E: Event>(&mut self, event_data: E::Data<'_>, resources: &mut BobaResources) {
        let mut queue = PearlQueue::new();
        self.events
            .trigger::<E>(event_data, &mut self.pearls, resources, &mut queue);
        queue.consume_into(self);
    }
}

struct PearlInsertQueue {
    pearls: Box<dyn Any>,
    insert_function: fn(Box<dyn Any>, &mut BobaPearls),
}

impl PearlInsertQueue {
    pub fn new<P: Pearl>() -> Self {
        Self {
            pearls: Box::new(Vec::<P>::new()),
            insert_function: Self::_insert_function::<P>,
        }
    }

    pub fn insert<P: Pearl>(&mut self, pearl: P) {
        let pearls = self
            .pearls
            .downcast_mut::<Vec<P>>()
            .expect("Tried inserting pearl into mismatched queue");

        pearls.push(pearl);
    }

    pub fn consume_into(self, map: &mut BobaPearls) {
        (self.insert_function)(self.pearls, map);
    }

    fn _insert_function<P: Pearl>(pearls: Box<dyn Any>, map: &mut BobaPearls) {
        let pearls = pearls.downcast::<Vec<P>>().unwrap();
        for pearl in pearls.into_iter() {
            map.insert(pearl);
        }
    }
}

struct PearlDestroyQueue {
    id: PearlId,
    handles: Vec<RawHandle>,
    destroy_function: fn(Vec<RawHandle>, &mut BobaPearls),
}

impl PearlDestroyQueue {
    pub fn new<P: Pearl>() -> Self {
        Self {
            id: P::id(),
            handles: Vec::new(),
            destroy_function: Self::_destroy_function::<P>,
        }
    }

    pub fn destroy<P: Pearl>(&mut self, handle: Handle<P>) {
        if self.id != P::id() {
            panic!("Tried destroying pearl with mismatched queue")
        }

        self.handles.push(handle.into_raw());
    }

    pub fn consume_into(self, map: &mut BobaPearls) {
        (self.destroy_function)(self.handles, map);
    }

    fn _destroy_function<P: Pearl>(handles: Vec<RawHandle>, map: &mut BobaPearls) {
        for handle in handles.into_iter() {
            let typed_handle = handle.into_type::<P>();
            map.remove(typed_handle);
        }
    }
}

#[derive(Default)]
struct PearlQueue {
    insert_queues: IndexMap<PearlId, PearlInsertQueue>,
    destroy_queues: IndexMap<PearlId, PearlDestroyQueue>,
}

impl PearlQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn consume_into(self, map: &mut BobaPearls) {
        for insert_queue in self.insert_queues.into_iter() {
            insert_queue.1.consume_into(map);
        }

        for destroy_queue in self.destroy_queues.into_iter() {
            destroy_queue.1.consume_into(map);
        }
    }
}

impl InsertDestroyQueue for PearlQueue {
    fn queue_insert<P: Pearl>(&mut self, pearl: P) {
        let queue = match self.insert_queues.entry(P::id()) {
            map::Entry::Occupied(e) => e.into_mut(),
            map::Entry::Vacant(e) => e.insert(PearlInsertQueue::new::<P>()),
        };

        queue.insert(pearl);
    }

    fn queue_destroy<P: Pearl>(&mut self, handle: Handle<P>) {
        let queue = match self.destroy_queues.entry(P::id()) {
            map::Entry::Occupied(e) => e.into_mut(),
            map::Entry::Vacant(e) => e.insert(PearlDestroyQueue::new::<P>()),
        };

        queue.destroy(handle);
    }
}

type EventRunner<E> =
    fn(&mut <E as Event>::Data<'_>, &mut PearlMap, &mut BobaResources, &mut PearlQueue);

#[derive(Default)]
struct EventRegistry {
    dispatchers: FxHashMap<TypeId, Box<dyn Any>>,
}

impl EventRegistry {
    pub fn trigger<E: Event>(
        &mut self,
        event_data: E::Data<'_>,
        pearl_map: &mut PearlMap,
        resources: &mut BobaResources,
        queue: &mut PearlQueue,
    ) {
        let Some(dispatcher) = self.dispatchers.get_mut(&TypeId::of::<E>()) else { return };
        let dispatcher = dispatcher.downcast_mut::<EventDispatcher<E>>().unwrap();
        dispatcher.dispatch(event_data, pearl_map, resources, queue)
    }
}

impl<P: Pearl> EventRegistrar<P> for EventRegistry {
    fn listen_for<E: crate::Event>(&mut self)
    where
        P: crate::EventListener<E>,
    {
        match self.dispatchers.entry(TypeId::of::<E>()) {
            hash_map::Entry::Occupied(e) => {
                let dispatcher = e.into_mut().downcast_mut::<EventDispatcher<E>>().unwrap();
                dispatcher.insert::<P>();
            }
            hash_map::Entry::Vacant(e) => {
                let mut dispatcher = EventDispatcher::<E>::new();
                dispatcher.insert::<P>();
                e.insert(Box::new(dispatcher));
            }
        }
    }
}

struct EventDispatcher<E: Event> {
    dispatch_ids: FxHashSet<PearlId>,
    runners: Vec<EventRunner<E>>,
}

impl<E: Event> Default for EventDispatcher<E> {
    fn default() -> Self {
        Self {
            dispatch_ids: Default::default(),
            runners: Default::default(),
        }
    }
}

impl<E: Event> EventDispatcher<E> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<P: EventListener<E>>(&mut self) {
        if !self.dispatch_ids.contains(&P::id()) {
            self.runners.push(Self::_runner::<P>);
        }
    }

    pub fn dispatch(
        &self,
        mut event_data: E::Data<'_>,
        pearl_map: &mut PearlMap,
        resources: &mut BobaResources,
        queue: &mut PearlQueue,
    ) {
        for runner in self.runners.iter() {
            runner(&mut event_data, pearl_map, resources, queue);
        }
    }

    fn _runner<P: EventListener<E>>(
        event_data: &mut E::Data<'_>,
        pearl_map: &mut PearlMap,
        resources: &mut BobaResources,
        queue: &mut PearlQueue,
    ) {
        let (mut access, mut queue) = pearl_map.split_access_queue(queue);
        let mut access_stream = access.exclusive_stream::<P>();
        while let Some((pearl, exlusive_access)) = access_stream.next() {
            let event_data = BobaEventData {
                event: event_data,
                pearls: BobaEventPearls {
                    exclusive: exlusive_access,
                    queue: &mut queue,
                },
                resources,
            };

            P::callback(pearl, event_data);
        }
    }
}

pub struct BobaEventData<'a, 'data, 'access, 'queue, E: Event> {
    pub event: &'a mut E::Data<'data>,
    pub pearls: BobaEventPearls<'access, 'a, 'queue>,
    pub resources: &'a mut BobaResources,
}

pub struct BobaEventPearls<'access, 'a, 'queue> {
    exclusive: ExclusivePearlAccess<'access, 'a>,
    queue: &'queue mut PearlMapQueue<'access, PearlQueue>,
}

impl<'access, 'a, 'queue> BobaEventPearls<'access, 'a, 'queue> {
    pub fn queue_insert<P: Pearl>(&mut self, pearl: P) -> Handle<P> {
        self.queue.insert(pearl)
    }

    pub fn queue_destroy<P: Pearl>(&mut self, handle: Handle<P>) {
        self.queue.destroy(handle);
    }

    pub fn get<P: Pearl>(&self, handle: Handle<P>) -> Option<&P> {
        self.exclusive.get(handle)
    }

    pub fn get_mut<P: Pearl>(&mut self, handle: Handle<P>) -> Option<&mut P> {
        self.exclusive.get_mut(handle)
    }

    pub fn iter<P: Pearl>(&self) -> ExclusiveIter<P> {
        self.exclusive.iter()
    }

    pub fn iter_mut<P: Pearl>(&mut self) -> ExclusiveIterMut<P> {
        self.exclusive.iter_mut()
    }
}
