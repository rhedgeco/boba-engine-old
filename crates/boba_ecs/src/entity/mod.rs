mod map;

pub use map::*;

use std::{fmt::Debug, hash::Hash};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Entity {
    id: u64,
}

impl Entity {
    const GEN_OFFSET: u32 = u32::BITS;
    const META_OFFSET: u32 = u32::BITS + u16::BITS;
    const INDEX_MASK: u64 = !(u32::MAX as u64);
    const GEN_MASK: u64 = !((u16::MAX as u64) << Self::GEN_OFFSET);
    const META_MASK: u64 = !((u16::MAX as u64) << Self::META_OFFSET);

    #[inline]
    pub fn from_raw(id: u64) -> Self {
        Self { id }
    }

    #[inline]
    pub fn from_raw_parts(index: u32, generation: u16, meta: u16) -> Self {
        Self {
            id: (index as u64)
                + ((generation as u64) << Self::GEN_OFFSET)
                + ((meta as u64) << Self::META_OFFSET),
        }
    }

    #[inline]
    pub fn into_raw(self) -> u64 {
        self.id
    }

    #[inline]
    pub fn into_raw_parts(self) -> (u32, u16, u16) {
        (self.index(), self.generation(), self.meta())
    }

    #[inline]
    pub fn index(self) -> u32 {
        self.id as u32
    }

    #[inline]
    pub fn uindex(self) -> usize {
        self.index() as usize
    }

    #[inline]
    pub fn generation(self) -> u16 {
        (self.id >> Self::GEN_OFFSET) as u16
    }

    #[inline]
    pub fn meta(self) -> u16 {
        (self.id >> Self::META_OFFSET) as u16
    }

    #[inline]
    pub fn set_index(&mut self, index: u32) {
        self.id &= Self::INDEX_MASK;
        self.id |= index as u64;
    }

    #[inline]
    pub fn set_generation(&mut self, generation: u16) {
        self.id &= Self::GEN_MASK;
        self.id |= (generation as u64) << Self::GEN_OFFSET;
    }

    #[inline]
    pub fn set_meta(&mut self, meta: u16) {
        self.id &= Self::META_MASK;
        self.id |= (meta as u64) << Self::META_OFFSET;
    }

    #[inline]
    pub fn increment_generation(&mut self) {
        self.set_generation(self.generation().wrapping_add(1));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_entity() {
        let entity = Entity::from_raw_parts(0, 1, 2);
        assert!(entity.index() == 0);
        assert!(entity.generation() == 1);
        assert!(entity.meta() == 2);
    }

    #[test]
    fn set_index() {
        let mut entity = Entity::from_raw_parts(0, 1, 2);
        entity.set_index(5);
        assert!(entity.index() == 5);
        assert!(entity.generation() == 1);
        assert!(entity.meta() == 2);
    }

    #[test]
    fn set_generation() {
        let mut entity = Entity::from_raw_parts(0, 1, 2);
        entity.set_generation(5);
        assert!(entity.index() == 0);
        assert!(entity.generation() == 5);
        assert!(entity.meta() == 2);
    }

    #[test]
    fn set_meta() {
        let mut entity = Entity::from_raw_parts(0, 1, 2);
        entity.set_meta(5);
        assert!(entity.index() == 0);
        assert!(entity.generation() == 1);
        assert!(entity.meta() == 5);
    }

    #[test]
    fn increment_generation() {
        let mut entity = Entity::from_raw_parts(0, 1, 2);
        entity.increment_generation();
        assert!(entity.index() == 0);
        assert!(entity.generation() == 2);
        assert!(entity.meta() == 2);
    }

    #[test]
    fn raw() {
        let entity = Entity::from_raw(5);
        let raw = entity.into_raw();
        assert!(raw == 5);
    }

    #[test]
    fn raw_parts() {
        let entity = Entity::from_raw_parts(0, 1, 2);
        let (index, gen, meta) = entity.into_raw_parts();
        assert!(index == 0);
        assert!(gen == 1);
        assert!(meta == 2);
    }
}
