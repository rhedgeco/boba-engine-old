// use std::any::{Any, TypeId};

// use handle_map::{
//     map::{
//         dense::{DenseHandleMap, DenseInputPredictor},
//         sparse,
//     },
//     RawHandle,
// };
// use hashbrown::{hash_map::Entry, HashMap, HashSet};

// use crate::{
//     pearls::{Pearl, PearlExt, PearlId},
//     BobaResources, Event, EventListener, EventRegistrar,
// };

// use super::{InnerPearlMap, Link, PearlMap, PearlMut};

// #[derive(Default)]
// pub(super) struct EventRegistry {
//     dispatchers: HashMap<TypeId, Box<dyn Any>>,
// }

// impl EventRegistry {
//     pub fn trigger<E: Event>(
//         &self,
//         event: &E,
//         map: &mut InnerPearlMap,
//         resources: &mut BobaResources,
//     ) -> Option<PearlQueue> {
//         let Some(any) = self.dispatchers.get(&TypeId::of::<E>()) else { return None };
//         let dispatcher = any.downcast_ref::<EventDispatcher<E>>().unwrap();
//         let mut pearl_queue = PearlQueue::new();
//         dispatcher.dispatch(event, map, &mut pearl_queue, resources);
//         Some(pearl_queue)
//     }
// }

// impl<P: Pearl> EventRegistrar<P> for EventRegistry {
//     fn listen_for<E: Event>(&mut self)
//     where
//         P: EventListener<E>,
//     {
//         match self.dispatchers.entry(TypeId::of::<E>()) {
//             Entry::Occupied(e) => {
//                 let dispatcher = e.into_mut().downcast_mut::<EventDispatcher<E>>().unwrap();
//                 dispatcher.register::<P>()
//             }
//             Entry::Vacant(e) => {
//                 let mut dispatcher = EventDispatcher::new();
//                 dispatcher.register::<P>();
//                 e.insert(Box::new(dispatcher));
//             }
//         }
//     }
// }

// type EventRunner<E> = fn(&E, &mut InnerPearlMap, &mut PearlQueue, &mut BobaResources);

// struct EventDispatcher<E: Event> {
//     pearls: HashSet<PearlId>,
//     runners: Vec<EventRunner<E>>,
// }

// impl<E: Event> EventDispatcher<E> {
//     pub fn new() -> Self {
//         Self {
//             pearls: HashSet::new(),
//             runners: Vec::new(),
//         }
//     }

//     pub fn register<L: EventListener<E>>(&mut self) {
//         if !self.pearls.contains(&L::id()) {
//             self.runners.push(Self::_callback::<L>);
//         }
//     }

//     pub fn dispatch(
//         &self,
//         event: &E,
//         map: &mut InnerPearlMap,
//         pearl_queue: &mut PearlQueue,
//         resources: &mut BobaResources,
//     ) {
//         for runner in self.runners.iter() {
//             runner(event, map, pearl_queue, resources);
//         }
//     }

//     fn _callback<'a, L: EventListener<E>>(
//         event: &E,
//         map: &mut InnerPearlMap,
//         pearl_queue: &mut PearlQueue,
//         resources: &mut BobaResources,
//     ) {
//     }
// }

// pub struct EventData<'a, 'data, E: Event, P: Pearl> {
//     pub event: &'a E,
//     pub pearls: &'a mut PearlEventQueue<'a, 'data, P>,
//     pub resources: &'a mut BobaResources,
// }

// pub struct PearlEventQueue<'a, 'data, P: Pearl> {
//     pearl_queue: &'data mut PearlQueue,
//     pearl_map_insert: &'data mut DenseInputPredictor<'a, P>,
//     other_maps: &'data mut sparse::ExclusiveAccessMap<'a, 'data, Box<dyn Any>>,
// }

// #[derive(Default)]
// pub(super) struct PearlQueue {
//     map_queues: HashMap<PearlId, Box<dyn Any>>,
// }

// impl PearlQueue {
//     pub fn new() -> Self {
//         todo!()
//     }
// }
