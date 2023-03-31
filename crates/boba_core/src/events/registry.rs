use std::any::{Any, TypeId};

use hashbrown::{hash_map, HashMap};
use indexmap::{map, IndexMap};

use crate::{
    pearls::{Pearl, PearlCollection, PearlId},
    BobaResources, World,
};

use super::{Event, EventData, EventListener, EventRegistrar};

/// A collection of event runners that can be used to operate on a [`World`].
#[derive(Default)]
pub struct EventRegistry {
    callbacks: HashMap<TypeId, IndexMap<PearlId, Box<dyn Any>>>,
}

impl EventRegistry {
    /// Returns a new empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Triggers an event that runs the callback for all registered [`Pearl`] objects in `world`.
    pub fn trigger<E: Event>(&self, data: &E, world: &mut World) {
        let Some(map) = self.callbacks.get(&TypeId::of::<E>()) else { return };
        let mut commands = PearlCommands::new();
        for any_callback in map.values() {
            let callback_runner = any_callback.downcast_ref::<CallbackRunner<E>>().unwrap();
            callback_runner.call(data, &world.pearls, &mut commands, &mut world.resources);
        }
        commands.execute_commands(&mut world.pearls);
    }
}

impl<T: Pearl> EventRegistrar<T> for EventRegistry {
    /// Registers a pearl of type `T` to listen for events of type `E`.
    fn listen_for<E: Event>(&mut self)
    where
        T: EventListener<E>,
    {
        // get or create callback map for event E
        let callback_map = match self.callbacks.entry(TypeId::of::<E>()) {
            hash_map::Entry::Occupied(e) => e.into_mut(),
            hash_map::Entry::Vacant(e) => e.insert(IndexMap::new()),
        };

        // add event iterator callback to map if it is not already there
        if let map::Entry::Vacant(entry) = callback_map.entry(PearlId::of::<T>()) {
            entry.insert(Box::new(CallbackRunner::<E>::new::<T>()));
        }
    }
}

pub trait PearlCommand: 'static {
    fn execute(&self, pearls: &mut PearlCollection);
}

pub struct PearlCommands {
    commands: Vec<Box<dyn PearlCommand>>,
}

impl PearlCommands {
    fn new() -> Self {
        let commands = Vec::new();
        Self { commands }
    }

    pub fn insert(&mut self, command: impl PearlCommand) {
        self.commands.push(Box::new(command));
    }

    fn execute_commands(self, pearls: &mut PearlCollection) {
        for command in self.commands.into_iter() {
            command.execute(pearls);
        }
    }
}

struct CallbackRunner<E: Event> {
    function: fn(&E, &PearlCollection, &mut PearlCommands, &mut BobaResources),
}

impl<E: Event> CallbackRunner<E> {
    pub fn new<P: EventListener<E>>() -> Self {
        let function = Self::callback::<P>;
        Self { function }
    }

    #[inline]
    pub fn call(
        &self,
        data: &E,
        pearls: &PearlCollection,
        commands: &mut PearlCommands,
        resources: &mut BobaResources,
    ) {
        (self.function)(data, pearls, commands, resources);
    }

    fn callback<P: EventListener<E>>(
        data: &E,
        pearls: &PearlCollection,
        commands: &mut PearlCommands,
        resources: &mut BobaResources,
    ) {
        let Some(handles) = pearls.get_handles::<P>() else { return };
        let handles = handles.iter().copied().collect::<Vec<_>>();
        for handle in handles.iter() {
            P::callback(
                &handle,
                EventData {
                    data,
                    pearls,
                    commands,
                    resources,
                },
            );
        }
    }
}
