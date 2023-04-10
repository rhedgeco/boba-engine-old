use std::any::{Any, TypeId};

use hashbrown::{hash_map::Entry, HashMap};
use indexmap::IndexMap;

use crate::{
    events::{Event, EventListener, EventRegistrar},
    pearls::{Pearl, PearlCollection, PearlId},
    BobaResources,
};

use super::{commands::CommandCollection, EventData};

#[derive(Default)]
pub struct EventManager {
    dispatchers: HashMap<TypeId, Box<dyn Any>>,
}

impl EventManager {
    pub fn trigger<E: Event>(
        &self,
        event: &E,
        pearls: &mut PearlCollection,
        resources: &mut BobaResources,
    ) -> CommandCollection {
        let mut commands = CommandCollection::new();
        let Some(dispatcher) = self.dispatchers.get(&TypeId::of::<E>()) else { return commands };
        let dispatcher = dispatcher.downcast_ref::<EventDispatcher<E>>().unwrap();
        dispatcher.call(event, pearls, resources, &mut commands);
        commands
    }
}

impl<P: Pearl> EventRegistrar<P> for EventManager {
    fn listen_for<E: Event>(&mut self)
    where
        P: EventListener<E>,
    {
        match self.dispatchers.entry(TypeId::of::<E>()) {
            Entry::Occupied(e) => {
                let dispatcher = e.into_mut().downcast_mut::<EventDispatcher<E>>().unwrap();
                dispatcher.add_callback::<P>();
            }
            Entry::Vacant(e) => {
                let mut dispatcher = EventDispatcher::<E>::new();
                dispatcher.add_callback::<P>();
                e.insert(Box::new(dispatcher));
            }
        }
    }
}

type EventCallback<E> = fn(&E, &mut PearlCollection, &mut BobaResources, &mut CommandCollection);

struct EventDispatcher<E: Event> {
    callbacks: IndexMap<PearlId, EventCallback<E>>,
}

impl<E: Event> Default for EventDispatcher<E> {
    #[inline]
    fn default() -> Self {
        Self {
            callbacks: IndexMap::default(),
        }
    }
}

impl<E: Event> EventDispatcher<E> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn add_callback<L: EventListener<E>>(&mut self) {
        self.callbacks
            .insert(PearlId::of::<L>(), Self::callback::<L>);
    }

    #[inline]
    pub fn call(
        &self,
        event: &E,
        pearls: &mut PearlCollection,
        resources: &mut BobaResources,
        commands: &mut CommandCollection,
    ) {
        for callback in self.callbacks.values() {
            callback(event, pearls, resources, commands);
        }
    }

    fn callback<L: EventListener<E>>(
        event: &E,
        pearls: &mut PearlCollection,
        resources: &mut BobaResources,
        commands: &mut CommandCollection,
    ) {
        // iterate over all the pearls and collect commands if necessary.
        let Some(mut split_step) = pearls.exclusive_stream::<L>() else { return };
        let event_commands = commands.create();
        while let Some((pearl, mut provider)) = split_step.next() {
            let event_data = EventData::new(event, &mut provider, resources, event_commands);
            L::callback(pearl, event_data);
        }
    }
}
