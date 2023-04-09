use std::any::{Any, TypeId};

use hashbrown::{hash_map::Entry, HashMap};
use indexmap::IndexMap;

use crate::{
    events::{Event, EventListener, EventRegistrar},
    pearls::{ExclusivePearlProvider, Pearl, PearlCollection, PearlId},
    BobaResources,
};

use super::commands::EventCommands;

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
    ) {
        let Some(dispatcher) = self.dispatchers.get(&TypeId::of::<E>()) else { return };
        let dispatcher = dispatcher.downcast_ref::<EventDispatcher<E>>().unwrap();
        dispatcher.call(event, pearls, resources);
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

type EventCallback<E> = fn(&E, &mut PearlCollection, &mut BobaResources);

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
    pub fn call(&self, event: &E, pearls: &mut PearlCollection, resources: &mut BobaResources) {
        for callback in self.callbacks.values() {
            callback(event, pearls, resources);
        }
    }

    fn callback<L: EventListener<E>>(
        event: &E,
        pearls: &mut PearlCollection,
        resources: &mut BobaResources,
    ) {
        // iterate over all the pearls and collect commands if necessary.
        let Some(mut split_step) = pearls.split_step::<L>() else { return };
        let mut commands = EventCommands::new();
        while let Some((pearl, mut provider)) = split_step.next() {
            let world = EventView::new(&mut provider, resources, &mut commands);
            pearl.callback(event, world);
        }

        // execute all collected commands after iteration
        commands.execute(pearls, resources);
    }
}

/// A window into the resources stored in a world.
///
/// This is used internally in [`EventListener`] callbacks
///  for events to provide access to the other pearls and resources in the world.
pub struct EventView<'a> {
    pub pearls: &'a mut ExclusivePearlProvider<'a>,
    pub resources: &'a mut BobaResources,
    pub commands: &'a mut EventCommands,
}

impl<'a> EventView<'a> {
    pub fn new(
        pearls: &'a mut ExclusivePearlProvider<'a>,
        resources: &'a mut BobaResources,
        commands: &'a mut EventCommands,
    ) -> Self {
        Self {
            pearls,
            resources,
            commands,
        }
    }
}
