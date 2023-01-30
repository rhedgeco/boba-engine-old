use std::{
    any::{type_name, Any, TypeId},
    cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut},
    hash::Hash,
    ops::Deref,
    rc::Rc,
};

use indexmap::{map::Entry, IndexMap};
use log::error;

use crate::{BobaId, BobaResult, Node};

/// The internal structure that is shared between [`Pearl`] and [`PearlLink`]
pub struct PearlCore<T> {
    id: BobaId,
    data: RefCell<T>,
}

impl<T> PearlCore<T> {
    /// Gets the [`BobaId`] for this pearl
    pub fn id(&self) -> &BobaId {
        &self.id
    }

    /// Borrows the underlying data from this pearl
    pub fn borrow(&self) -> Result<Ref<T>, BorrowError> {
        self.data.try_borrow()
    }

    /// Mutably borrows the underlying data from this pearl
    pub fn borrow_mut(&self) -> Result<RefMut<T>, BorrowMutError> {
        self.data.try_borrow_mut()
    }
}

/// Core struct for wrapping data to connected to a node
///
/// While this could technically be cloned, it is prevented from doing so as to not connect the same pearl to multiple [`Node`]s.
/// Instead, the pearl can be built into a [`PearlLink`] which has essentially the same functionality, but can be cloned and used across anywhere it is needed.
/// However, links cannot be added directly to a [`Node`].
pub struct Pearl<T> {
    core: Rc<PearlCore<T>>,
}

impl<T> PartialEq<PearlLink<T>> for Pearl<T> {
    fn eq(&self, other: &PearlLink<T>) -> bool {
        self.core.id == other.core.id
    }
}

impl<T> Deref for Pearl<T> {
    type Target = PearlCore<T>;

    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

impl<T> Pearl<T> {
    /// Creates a new `Pearl` containing `data`
    pub fn new(data: T) -> Self {
        let inner = PearlCore {
            id: BobaId::new(),
            data: RefCell::new(data),
        };

        Self {
            core: Rc::new(inner),
        }
    }

    /// Creates a new [`PearlLink`] for this pearl
    pub fn new_link(&self) -> PearlLink<T> {
        PearlLink {
            core: self.core.clone(),
        }
    }

    /// Creates a clone of the pearl struct
    ///
    /// This is for internal management only
    fn sealed_clone(&self) -> Pearl<T> {
        Self {
            core: self.core.clone(),
        }
    }
}

/// A link to the core [`Pearl`] struct.
///
/// This can be cloned, and should be used when data needs to be linked across pearls.
#[derive(Clone)]
pub struct PearlLink<T> {
    core: Rc<PearlCore<T>>,
}

impl<T> Eq for PearlLink<T> {}

impl<T> PartialEq for PearlLink<T> {
    fn eq(&self, other: &Self) -> bool {
        self.core.id == other.core.id
    }
}

impl<T> PartialEq<Pearl<T>> for PearlLink<T> {
    fn eq(&self, other: &Pearl<T>) -> bool {
        self.core.id == other.core.id
    }
}

impl<T> Hash for PearlLink<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.core.id.hash(state);
    }
}

impl<T> Deref for PearlLink<T> {
    type Target = PearlCore<T>;

    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

pub trait NodeEvent<Data>: Sized + 'static {
    fn update(pearl: &Pearl<Self>, node: &Node, data: &Data) -> BobaResult;
}

pub trait EventRegistrar {
    fn add_listener<Data: 'static>(&mut self, pearl: &Pearl<impl NodeEvent<Data>>);
}

pub trait RegisterEvents: Sized + 'static {
    fn register(pearl: Pearl<Self>, registrar: &mut impl EventRegistrar);
}

trait PearlUpdater<Data> {
    fn update(&self, node: &Node, data: &Data);
}

impl<Data, Event: NodeEvent<Data>> PearlUpdater<Data> for Pearl<Event> {
    fn update(&self, node: &Node, data: &Data) {
        if let Err(e) = Event::update(self, node, data) {
            let name = type_name::<Event>();
            error!("There was an error while updating Pearl<{name}>. Error: {e}");
        }
    }
}

struct EventCollection<Data> {
    listeners: IndexMap<BobaId, Box<dyn PearlUpdater<Data>>>,
}

impl<Data> Default for EventCollection<Data> {
    fn default() -> Self {
        Self {
            listeners: Default::default(),
        }
    }
}

impl<Data> EventCollection<Data> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, pearl: &Pearl<impl NodeEvent<Data>>) {
        self.listeners
            .insert(pearl.id().clone(), Box::new(pearl.sealed_clone()));
    }

    pub fn update(&self, node: &Node, data: &Data) {
        for listener in self.listeners.values() {
            listener.update(node, data);
        }
    }
}

pub struct EventMap {
    events: IndexMap<TypeId, Box<dyn Any>>,
}

impl EventRegistrar for EventMap {
    fn add_listener<Data: 'static>(&mut self, pearl: &Pearl<impl NodeEvent<Data>>) {
        let dataid = TypeId::of::<Data>();
        match self.events.entry(dataid) {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => e.insert(Box::new(EventCollection::<Data>::new())),
        }
        .downcast_mut::<EventCollection<Data>>()
        .unwrap()
        .add(pearl);
    }
}

impl EventMap {
    pub fn update<Data: 'static>(&self, node: &Node, data: &Data) {
        let dataid = TypeId::of::<Data>();
        if let Some(any_collection) = self.events.get(&dataid) {
            let collection = any_collection
                .downcast_ref::<EventCollection<Data>>()
                .unwrap();
            collection.update(node, data)
        }
    }
}
