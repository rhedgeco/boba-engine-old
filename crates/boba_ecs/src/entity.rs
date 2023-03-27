use std::{
    collections::VecDeque,
    sync::atomic::{AtomicU16, Ordering},
};

/// An entity is a link into specific components of a [`World`][crate::World]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Entity(u64);

impl Entity {
    const INDEX_BITS: u32 = u64::BITS / 2;
    const GEN_BITS: u32 = u64::BITS / 4;
    const META_BITS: u32 = u64::BITS / 4;
    const GEN_OFFSET: u32 = Self::INDEX_BITS;
    const META_OFFSET: u32 = Self::INDEX_BITS + Self::GEN_BITS;

    /// Returns a new entity with the raw `value`
    ///
    /// Marked as `unsafe`, as manually creating an entity
    /// could be used to index into a [`World`][crate::World] where this entity should be invalid
    #[inline]
    pub unsafe fn from_raw(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying `u64` used as an id for this entity
    #[inline]
    pub fn into_raw(self) -> u64 {
        self.0
    }

    /// Returns a new entity containing the raw parts `index`, `gen`, and `meta`
    ///
    /// Marked as `unsafe`, as manually creating an entity
    /// could be used to index into a [`World`][crate::World] where this entity should be invalid
    #[inline]
    pub unsafe fn from_raw_parts(index: u32, gen: u16, meta: u16) -> Self {
        Self(
            (index as u64)
                + ((gen as u64) << Self::GEN_OFFSET)
                + ((meta as u64) << Self::META_OFFSET),
        )
    }

    /// Decomposes this entity into its raw parts:
    /// - `u32`: index
    /// - `u16`: generation
    /// - `u16`: metadata
    #[inline]
    pub fn into_raw_parts(self) -> (u32, u16, u16) {
        (
            self.0 as u32,
            (self.0 >> Self::GEN_BITS) as u16,
            (self.0 >> Self::META_BITS) as u16,
        )
    }

    /// Returns the raw index value for this entity
    #[inline]
    pub fn index(self) -> u32 {
        self.0 as u32
    }

    /// Returns the index value for this entity as a `usize`
    #[inline]
    pub fn uindex(self) -> usize {
        self.0 as u32 as usize
    }

    /// Returns the raw generation value for this entity
    #[inline]
    pub fn generation(self) -> u16 {
        (self.0 >> Self::GEN_BITS) as u16
    }

    /// Returns the raw metadata value for this entity
    #[inline]
    pub fn metadata(self) -> u16 {
        (self.0 >> Self::META_BITS) as u16
    }
}

struct EntityEntry<T> {
    entity: Entity,
    data: T,
}

impl<T> EntityEntry<T> {
    pub fn new(entity: Entity, data: T) -> Self {
        Self { entity, data }
    }
}

/// A collection of [`Entity`] objects stored for fast lookup times
pub struct EntityManager<T: Copy> {
    id: u16,
    entities: Vec<EntityEntry<T>>,
    open_entities: VecDeque<usize>,
}

impl<T: Copy> Default for EntityManager<T> {
    /// Creates a new entity manager with a unique id
    #[inline]
    fn default() -> Self {
        static ID_GEN: AtomicU16 = AtomicU16::new(0);

        Self {
            id: ID_GEN.fetch_add(1, Ordering::Relaxed),
            entities: Default::default(),
            open_entities: Default::default(),
        }
    }
}

impl<T: Copy> EntityManager<T> {
    /// Creates a new entity manager with a unique id
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the id for this manager
    pub fn id(&self) -> u16 {
        self.id
    }

    /// Returns a new unique entity associated this manager
    pub fn create(&mut self, data: T) -> Entity {
        match self.open_entities.pop_front() {
            Some(index) => {
                let entry = &self.entities[index];
                entry.entity
            }
            None => {
                let index = self.entities.len() as u32;
                if index > u32::MAX {
                    panic!("Entity Capacity Overflow");
                }

                let entity = unsafe { Entity::from_raw_parts(index, 0, self.id) };
                self.entities.push(EntityEntry::new(entity, data));
                entity
            }
        }
    }

    /// Returns true if `entity` is valid for this manager
    pub fn contains(&self, entity: &Entity) -> bool {
        match self.entities.get(entity.uindex()) {
            Some(other) => other.entity.into_raw() == entity.into_raw(),
            None => false,
        }
    }

    /// Returns a reference to the data associated with `entity`.
    ///
    /// Returns `None` if `entity` is invalid for this manager.
    pub fn get_data(&self, entity: &Entity) -> Option<&T> {
        let entry = self.entities.get(entity.uindex())?;
        if &entry.entity != entity {
            return None;
        }

        Some(&entry.data)
    }

    /// Returns a mutable reference to the data associated with `entity`.
    ///
    /// Returns `None` if `entity` is invalid for this manager.
    pub fn get_data_mut(&mut self, entity: &Entity) -> Option<&mut T> {
        let entry = self.entities.get_mut(entity.uindex())?;
        if &entry.entity != entity {
            return None;
        }

        Some(&mut entry.data)
    }

    /// Destroys `entity`, returning its data as `Some(T)`.
    ///
    /// Returns `None` if `entity` is invalid for this manager.
    pub fn destroy(&mut self, entity: &Entity) -> Option<T> {
        match self.entities.get_mut(entity.uindex()) {
            Some(other) if &other.entity == entity => {
                let (index, gen, meta) = other.entity.into_raw_parts();
                other.entity = unsafe { Entity::from_raw_parts(index, gen.wrapping_add(1), meta) };
                self.open_entities.push_back(entity.uindex());
                return Some(other.data);
            }
            _ => return None,
        }
    }
}
