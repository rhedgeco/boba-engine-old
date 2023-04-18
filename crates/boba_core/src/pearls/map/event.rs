use std::{
    any::{Any, TypeId},
    collections::hash_map::Entry,
    ops::Deref,
};

use fxhash::{FxHashMap, FxHashSet};

use crate::{
    pearls::{Pearl, PearlExt, PearlId},
    BobaResources, Event, EventListener, EventRegistrar,
};

use super::{
    ExclusivePearlAccess, Handle, PearlEventQueue, PearlProvider, PearlQueue, RawHandle,
    RawPearlMap,
};

pub struct EventData<'a, 'access, E: Event> {
    event: &'a E,
    pub pearls: EventPearls<'a, 'access>,
    pub resources: &'access mut BobaResources,
}

impl<'a, 'access, E: Event> Deref for EventData<'a, 'access, E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        self.event
    }
}

pub struct EventPearls<'a, 'access> {
    exclusive_access: ExclusivePearlAccess<'a, 'access>,
    insert_queue: &'access mut PearlEventQueue<'a>,
}

impl<'a, 'access> PearlProvider for EventPearls<'a, 'access> {
    fn get<P: Pearl>(&self, handle: Handle<P>) -> Option<&P> {
        self.exclusive_access.get(handle)
    }

    fn get_mut<P: Pearl>(&mut self, handle: Handle<P>) -> Option<&mut P> {
        self.exclusive_access.get_mut(handle)
    }
}

impl<'a, 'access> EventPearls<'a, 'access> {
    pub fn get_excluded_handle(access: &EventPearls) -> RawHandle {
        ExclusivePearlAccess::get_excluded_handle(&access.exclusive_access)
    }

    pub fn queue_insert<P: Pearl>(&mut self, pearl: P) -> Handle<P> {
        self.insert_queue.insert(pearl)
    }

    pub fn queue_destroy<P: Pearl>(&mut self, handle: Handle<P>) {
        self.insert_queue.destroy(handle);
    }

    pub fn get<P: Pearl>(&self, handle: Handle<P>) -> Option<&P> {
        <Self as PearlProvider>::get(self, handle)
    }

    pub fn get_mut<P: Pearl>(&mut self, handle: Handle<P>) -> Option<&mut P> {
        <Self as PearlProvider>::get_mut(self, handle)
    }
}

#[derive(Default)]
pub(super) struct EventRegistry {
    dispatchers: FxHashMap<TypeId, Box<dyn Any>>,
}

impl EventRegistry {
    pub fn run_event<E: Event>(
        &self,
        event: &E,
        map: &mut RawPearlMap,
        resources: &mut BobaResources,
        pearl_queue: &mut PearlQueue,
    ) {
        let Some(any) = self.dispatchers.get(&TypeId::of::<E>()) else { return };
        let dispatcher = any.downcast_ref::<EventDispatcher<E>>().unwrap();
        dispatcher.dispatch(event, map, resources, pearl_queue);
    }
}

impl<P: Pearl> EventRegistrar<P> for EventRegistry {
    fn listen_for<E: Event>(&mut self)
    where
        P: EventListener<E>,
    {
        match self.dispatchers.entry(TypeId::of::<E>()) {
            Entry::Occupied(e) => {
                let dispatcher = e.into_mut().downcast_mut::<EventDispatcher<E>>().unwrap();
                dispatcher.insert::<P>();
            }
            Entry::Vacant(e) => {
                let mut dispatcher = EventDispatcher::<E>::new();
                dispatcher.insert::<P>();
                e.insert(Box::new(dispatcher));
            }
        };
    }
}

type EventRunner<E> = fn(&E, &mut RawPearlMap, &mut BobaResources, &mut PearlQueue);

struct EventDispatcher<E: Event> {
    pearls: FxHashSet<PearlId>,
    runners: Vec<EventRunner<E>>,
}

impl<E: Event> EventDispatcher<E> {
    pub fn new() -> Self {
        Self {
            pearls: Default::default(),
            runners: Vec::new(),
        }
    }

    pub fn insert<L: EventListener<E>>(&mut self) {
        if !self.pearls.contains(&L::id()) {
            self.runners.push(Self::_runner::<L>);
        }
    }

    pub fn dispatch(
        &self,
        event: &E,
        map: &mut RawPearlMap,
        resources: &mut BobaResources,
        pearl_queue: &mut PearlQueue,
    ) {
        for runner in self.runners.iter() {
            runner(event, map, resources, pearl_queue);
        }
    }

    fn _runner<L: EventListener<E>>(
        event: &E,
        map: &mut RawPearlMap,
        resources: &mut BobaResources,
        pearl_queue: &mut PearlQueue,
    ) {
        let (mut insert_queue, mut access_map) = map.split_queue_access(pearl_queue);
        let Some(mut access_stream) = access_map.exclusive_stream::<L>() else { return };
        while let Some((pearl, exclusive_access)) = access_stream.next() {
            let pearls = EventPearls {
                exclusive_access,
                insert_queue: &mut insert_queue,
            };

            let world_view = EventData {
                event,
                pearls,
                resources,
            };

            L::callback(pearl, world_view);
        }
    }
}
