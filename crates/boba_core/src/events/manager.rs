use std::any::{Any, TypeId};

use hashbrown::{hash_map::Entry, HashMap};
use indexmap::IndexMap;

use crate::{
    events::{Event, EventListener, EventRegistrar},
    pearls::{ExclusivePearlProvider, Pearl, PearlCollection, PearlId},
    BobaResources,
};

/// A window into the resources stored in a world.
///
/// This is used internally in [`EventListener`] callbacks
///  for events to provide access to the other pearls and resources in the world.
pub struct WorldView<'a> {
    pub pearls: &'a mut ExclusivePearlProvider<'a>,
    pub resources: &'a mut BobaResources,
    pub commands: &'a mut WorldCommands,
}

impl<'a> WorldView<'a> {
    pub fn new(
        pearls: &'a mut ExclusivePearlProvider<'a>,
        resources: &'a mut BobaResources,
        commands: &'a mut WorldCommands,
    ) -> Self {
        Self {
            pearls,
            resources,
            commands,
        }
    }
}

type WorldCommand = fn(&mut PearlCollection, &mut BobaResources);

/// A collection for storing commands to be run later on a [`BobaWorld`].
pub struct WorldCommands {
    commands: Vec<WorldCommand>,
}

impl WorldCommands {
    fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    /// Inserts a [`WorldCommand`] to be executed later.
    pub fn insert(&mut self, command: WorldCommand) {
        self.commands.push(command);
    }
}

type EventCallback<E> = fn(&E, &mut PearlCollection, &mut BobaResources);

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
        let commands = &mut WorldCommands::new();
        while let Some((pearl, mut provider)) = split_step.next() {
            let world = WorldView::new(&mut provider, resources, commands);
            pearl.callback(event, world);
        }

        // execute all the commands after iteration.
        for command in commands.commands.iter() {
            command(pearls, resources);
        }
    }
}
