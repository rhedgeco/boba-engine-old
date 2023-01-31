use std::any::{type_name, Any, TypeId};

use indexmap::{map::Entry, IndexMap};
use log::error;

use crate::{BobaId, BobaResult, Pearl};

pub trait OnEvent<Data>: Sized + 'static {
    fn update(pearl: &Pearl<Self>, data: &Data) -> BobaResult;
}

pub trait EventRegistrar {
    fn add_listener<Data: 'static>(&mut self, pearl: &Pearl<impl OnEvent<Data>>);
}

pub trait RegisterEvents: Sized + 'static {
    fn register(pearl: &Pearl<Self>, registrar: &mut impl EventRegistrar);
}

trait EventListener<Data>: 'static {
    fn update(&self, data: &Data);
}

impl<Data, Event: OnEvent<Data>> EventListener<Data> for Pearl<Event> {
    /// Event listener implementation for pearls
    fn update(&self, data: &Data) {
        if let Err(e) = Event::update(self, data) {
            let update_name = type_name::<Data>();
            let pearl_name = type_name::<Event>();
            error!("There was an error while running event `{update_name}` on Pearl<{pearl_name}>. Error: {e}");
        }
    }
}

struct EventSet<Data> {
    listeners: IndexMap<BobaId, Box<dyn EventListener<Data>>>,
}

impl<Data> EventSet<Data> {
    pub fn new() -> Self {
        Self {
            listeners: IndexMap::new(),
        }
    }

    pub fn add_listener(&mut self, pearl: &Pearl<impl OnEvent<Data>>) {
        self.listeners.insert(*pearl.id(), Box::new(pearl.clone()));
    }
}

#[derive(Default)]
pub struct EventListenerCollection {
    event_sets: IndexMap<TypeId, Box<dyn Any>>,
}

impl EventRegistrar for EventListenerCollection {
    fn add_listener<Data: 'static>(&mut self, pearl: &Pearl<impl OnEvent<Data>>) {
        let typeid = TypeId::of::<Data>();
        match self.event_sets.entry(typeid) {
            Entry::Occupied(e) => {
                let set = e.into_mut().downcast_mut::<EventSet<Data>>().unwrap();
                set.add_listener(pearl)
            }
            Entry::Vacant(e) => {
                let mut set = EventSet::<Data>::new();
                set.add_listener(pearl);
                e.insert(Box::new(set));
            }
        }
    }
}

impl EventListenerCollection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_pearl<T: RegisterEvents>(&mut self, pearl: &Pearl<T>) {
        T::register(pearl, self);
    }

    pub fn update<Data: 'static>(&self, data: &Data) {
        let typeid = TypeId::of::<Data>();
        if let Some(any_event_set) = self.event_sets.get(&typeid) {
            let event_set = any_event_set.downcast_ref::<EventSet<Data>>().unwrap();
            for listener in event_set.listeners.values() {
                listener.update(data);
            }
        }
    }
}
