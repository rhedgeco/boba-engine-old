use handle_map::Handle;

use crate::{events::EventRegistry, pearls::Pearl, World};

pub trait AppManager: 'static {
    fn run(&mut self, world: &mut World, events: &mut EventRegistry);
}

#[derive(Default)]
pub struct BobaApp<A: AppManager> {
    world: World,
    events: EventRegistry,
    manager: A,
}

impl<A: AppManager> BobaApp<A> {
    #[inline]
    pub fn new(manager: A) -> Self {
        Self {
            world: Default::default(),
            events: Default::default(),
            manager,
        }
    }

    #[inline]
    pub fn insert_pearl<T: Pearl>(&mut self, pearl: T) -> Handle<T> {
        // if the pearl hasnt been seen before, register it
        if !self.world.pearls.contains_type::<T>() {
            T::register(&mut self.events);
        }

        self.world.pearls.insert(pearl)
    }

    #[inline]
    pub fn insert_resource<T: 'static>(&mut self, resource: T) {
        self.world.resources.insert(resource);
    }

    #[inline]
    pub fn remove_pearl<T: Pearl>(&mut self, handle: &Handle<T>) -> Option<T> {
        self.world.pearls.remove(handle)
    }

    #[inline]
    pub fn remove_resource<T: 'static>(&mut self) -> Option<T> {
        self.world.resources.remove::<T>()
    }

    #[inline]
    pub fn run(&mut self) {
        self.manager.run(&mut self.world, &mut self.events);
    }
}
