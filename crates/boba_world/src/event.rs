use std::{
    any::{type_name, Any, TypeId},
    marker::PhantomData,
};

use indexmap::{map::Entry, IndexMap};
use log::error;

use crate::{BobaId, BobaResult, PearlHandle, World};

pub trait OnEvent<Data>: Sized + 'static {
    fn update<const ID: usize>(
        pearl: &PearlHandle<Self, ID>,
        world: &mut World<ID>,
        data: &Data,
    ) -> BobaResult;
}

pub trait EventRegistrar<const ID: usize> {
    fn add_listener<Data: 'static>(&mut self, listener: &PearlHandle<impl OnEvent<Data>, ID>);
}

pub trait RegisterListeners: Sized + 'static {
    fn register<const ID: usize>(
        pearl: &PearlHandle<Self, ID>,
        registrar: &mut impl EventRegistrar<ID>,
    );
}

trait AnyEventRunner<Data, const ID: usize> {
    fn update(&self, world: &mut World<ID>, data: &Data);
}

impl<Data, Event: OnEvent<Data>, const ID: usize> AnyEventRunner<Data, ID>
    for PearlHandle<Event, ID>
{
    fn update(&self, world: &mut World<ID>, data: &Data) {
        if let Err(e) = Event::update(self, world, data) {
            let name = type_name::<Event>();
            error!("There was an error while running Pearl<{name}>. Error: {e}");
        }
    }
}

struct ListenerSet<Data, const ID: usize> {
    listeners: IndexMap<BobaId, Box<dyn AnyEventRunner<Data, ID>>>,
    _type: PhantomData<Data>,
}

impl<Data, const ID: usize> Default for ListenerSet<Data, ID> {
    fn default() -> Self {
        Self {
            listeners: Default::default(),
            _type: Default::default(),
        }
    }
}

impl<Data, const ID: usize> ListenerSet<Data, ID> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, listener: &PearlHandle<impl OnEvent<Data>, ID>) {
        self.listeners
            .insert(listener.id().clone(), Box::new(listener.clone()));
    }
}

pub struct EventListeners<const ID: usize> {
    listener_sets: IndexMap<TypeId, Box<dyn Any>>,
}

impl<const ID: usize> Default for EventListeners<ID> {
    fn default() -> Self {
        Self {
            listener_sets: Default::default(),
        }
    }
}

impl<const ID: usize> EventRegistrar<ID> for EventListeners<ID> {
    fn add_listener<Data: 'static>(&mut self, listener: &PearlHandle<impl OnEvent<Data>, ID>) {
        match self.listener_sets.entry(TypeId::of::<Data>()) {
            Entry::Occupied(e) => {
                let any_set = e.into_mut();
                let set = any_set.downcast_mut::<ListenerSet<Data, ID>>().unwrap();
                set.insert(listener);
            }
            Entry::Vacant(e) => {
                let mut set = ListenerSet::<Data, ID>::new();
                set.insert(listener);
                e.insert(Box::new(set));
            }
        }
    }
}

impl<const ID: usize> EventListeners<ID> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<T: RegisterListeners>(&mut self, pearl: &PearlHandle<T, ID>) {
        T::register(pearl, self);
    }
}
