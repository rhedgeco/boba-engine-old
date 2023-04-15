use std::any::{Any, TypeId};

use hashbrown::{hash_map::Entry, HashMap, HashSet};

use crate::{
    pearls::{Pearl, PearlExt, PearlId},
    BobaResources, Event, EventListener, EventRegistrar,
};

use super::{ExclusivePearlAccess, InnerPearlMap, PearlLink, PearlQueue, PearlQueueEvents};

pub struct EventWorldView<'a, 'access, E: Event> {
    pub event: &'a E,
    pub pearls: EventPearls<'a, 'access>,
    pub resources: &'access mut BobaResources,
}

pub struct EventPearls<'a, 'access> {
    exclusive_access: ExclusivePearlAccess<'a, 'access>,
    insert_queue: &'access mut PearlQueue<'a>,
}

impl<'a, 'access> EventPearls<'a, 'access> {
    pub fn queue_insert<P: Pearl>(&mut self, pearl: P) -> PearlLink<P> {
        self.insert_queue.insert(pearl)
    }

    pub fn queue_destroy<P: Pearl>(&mut self, link: PearlLink<P>) {
        self.insert_queue.destroy(link);
    }

    pub fn get<P: Pearl>(&self, link: PearlLink<P>) -> Option<&P> {
        self.exclusive_access.get(link)
    }

    pub fn get_mut<P: Pearl>(&mut self, link: PearlLink<P>) -> Option<&mut P> {
        self.exclusive_access.get_mut(link)
    }
}

#[derive(Default)]
pub(super) struct EventRegistry {
    dispatchers: HashMap<TypeId, Box<dyn Any>>,
}

impl EventRegistry {
    pub fn run_event<E: Event>(
        &self,
        event: &E,
        map: &mut InnerPearlMap,
        resources: &mut BobaResources,
        pearl_queue: &mut PearlQueueEvents,
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

type EventRunner<E> = fn(&E, &mut InnerPearlMap, &mut BobaResources, &mut PearlQueueEvents);

struct EventDispatcher<E: Event> {
    pearls: HashSet<PearlId>,
    runners: Vec<EventRunner<E>>,
}

impl<E: Event> EventDispatcher<E> {
    pub fn new() -> Self {
        Self {
            pearls: HashSet::new(),
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
        map: &mut InnerPearlMap,
        resources: &mut BobaResources,
        pearl_queue: &mut PearlQueueEvents,
    ) {
        for runner in self.runners.iter() {
            runner(event, map, resources, pearl_queue);
        }
    }

    fn _runner<L: EventListener<E>>(
        event: &E,
        map: &mut InnerPearlMap,
        resources: &mut BobaResources,
        pearl_queue: &mut PearlQueueEvents,
    ) {
        let (mut insert_queue, mut access_map) = map.split_queue_access(pearl_queue);
        let Some(mut access_stream) = access_map.exclusive_stream::<L>() else { return };
        while let Some((pearl, exclusive_access)) = access_stream.next() {
            let pearls = EventPearls {
                exclusive_access,
                insert_queue: &mut insert_queue,
            };

            let world_view = EventWorldView {
                event,
                pearls,
                resources,
            };

            L::callback(pearl, world_view);
        }
    }
}
